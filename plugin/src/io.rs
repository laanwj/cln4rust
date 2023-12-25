//! async io module of the plugin io.
//!
//! Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>
use std::io;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;

const IN: mio::Token = mio::Token(0);

pub(crate) struct AsyncIO {
    poll: mio::Poll,
}

impl AsyncIO {
    /// Create a new instance of an AsyncIO
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            poll: mio::Poll::new()?,
        })
    }

    pub fn register(&mut self) -> io::Result<()> {
        let stdin = std::io::stdin().as_raw_fd();
        let mut stdin = mio::unix::SourceFd(&stdin);

        self.poll.registry().register(
            &mut stdin,
            IN,
            mio::Interest::READABLE | mio::Interest::WRITABLE,
        )?;
        Ok(())
    }

    pub fn into_loop<F: FnMut(String) -> Option<String>>(
        &mut self,
        mut async_callback: F,
    ) -> io::Result<()> {
        let mut events = mio::Events::with_capacity(1024);
        loop {
            self.poll.poll(&mut events, None)?;
            for event in events.iter() {
                #[cfg(feature = "log")]
                log::info!("getting the event: {:?}", event);
                match event.token() {
                    IN => {
                        if event.is_readable() {
                            let mut reader = io::stdin().lock();
                            let mut buffer = String::new();
                            loop {
                                let mut byte = [0; 1];
                                reader.read_exact(&mut byte).unwrap();

                                // Append the byte to the buffer
                                buffer.push(byte[0] as char);

                                // Check if the buffer ends with double newline
                                if buffer.ends_with("\n\n") {
                                    drop(reader);
                                    break; // Exit the loop
                                }
                            }
                            let Some(resp) = async_callback(buffer.clone()) else {
                                continue;
                            };
                            let mut writer = io::stdout().lock();
                            writer.write_all(resp.as_bytes())?;
                            writer.flush()?;
                        }
                    }
                    _ => unreachable!(),
                }
                #[cfg(feature = "log")]
                log::info!("event handled: {:?}", event);
            }
        }
    }
}
