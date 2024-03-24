//! async io module of the plugin io.
//!
//! Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>
use std::io::Write;
use std::io::{self, Read};
use std::os::fd::AsRawFd;
use std::time::Duration;

const SERVER_TOKEN: mio::Token = mio::Token(0);

pub struct AsyncIO {
    poll: mio::Poll,
    events: mio::Events,
}

impl AsyncIO {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            poll: mio::Poll::new()?,
            events: mio::Events::with_capacity(1024),
        })
    }

    pub fn register(&self) -> io::Result<()> {
        let stdin = std::io::stdin().as_raw_fd();
        let mut stdin = mio::unix::SourceFd(&stdin);
        self.poll
            .registry()
            .register(&mut stdin, SERVER_TOKEN, mio::Interest::READABLE)?;
        Ok(())
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn into_loop<F>(&mut self, mut async_callback: F) -> io::Result<()>
    where
        F: FnMut(String) -> Option<String>,
    {
        loop {
            self.poll
                .poll(&mut self.events, Some(Duration::from_millis(100)))?;
            for event in self.events.iter() {
                match event.token() {
                    SERVER_TOKEN => {
                        if event.is_readable() {
                            self.handle_connection(&mut async_callback)?;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn handle_connection<F>(&self, async_callback: &mut F) -> io::Result<()>
    where
        F: FnMut(String) -> Option<String>,
    {
        loop {
            let mut reader = io::stdin().lock();
            let mut buffer = String::new();
            loop {
                let mut byte = [0; 1];
                crate::poll_check!(reader.read_exact(&mut byte))?;

                // Append the byte to the buffer
                buffer.push(byte[0] as char);

                // Check if the buffer ends with double newline
                if buffer.ends_with("\n\n") {
                    break; // Exit the loop
                }
            }

            if let Some(resp) = async_callback(buffer.clone()) {
                let mut writer = io::stdout().lock();
                crate::poll_check!(writer.write_all(resp.as_bytes()))?;
                crate::poll_check!(writer.flush())?;
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! poll_check {
    ($expr:expr) => {{
        match $expr {
            Ok(val) => Ok::<_, std::io::Error>(val),
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                // Handle WouldBlock error
                // For example, wait for readiness event and retry
                // You may need to use mio's event loop to wait for readiness
                // and then retry the operation
                // For simplicity, we'll just continue the loop here
                break;
            }
            Err(err) => Err(err.into()),
        }
    }};
}

#[macro_export]
macro_rules! poll_loop {
    ($code:block) => {{
        while let Err(ref err) = $code {
            if err.kind() == std::io::ErrorKind::WouldBlock {
                // Handle WouldBlock error
                // For example, wait for readiness event and retry
                // You may need to use mio's event loop to wait for readiness
                // and then retry the operation
                // For simplicity, we'll just continue the loop here
                continue;
            }
        }
    }};
}
