use std::io;
use std::io::{Read, Seek, SeekFrom};

pub struct PeekableStream<R: Read + Seek> {
    inner: R,
    peek_buffer: Vec<u8>,
}

impl<R: Read + Seek> PeekableStream<R> {
    pub fn new(inner: R) -> Self {
        PeekableStream {
            inner,
            peek_buffer: Vec::new(),
        }
    }

    // Peek and return a u16 without consuming it
    // Assumes big-endian byte order for this example; adjust as necessary
    pub fn peek_msgtype(&mut self) -> io::Result<u16> {
        while self.peek_buffer.len() < 2 {
            let mut buf = [0; 1];
            let n = self.inner.read(&mut buf)?;
            if n == 0 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached"));
            }
            self.peek_buffer.push(buf[0]);
        }

        Ok(((self.peek_buffer[0] as u16) << 8) | (self.peek_buffer[1] as u16))
    }

    // Reset peek buffer
    pub fn reset_peek(&mut self) {
        self.peek_buffer.clear();
    }

    // Adds seeking functionality relative to the current position
    // Note: This simplistic implementation only supports positive offsets
    pub fn seek_relative(&mut self, offset: i64) -> io::Result<u64> {
        let peek_len = self.peek_buffer.len() as i64;
        // Calculate how much we need to seek in the underlying stream
        let seek_offset = offset - peek_len;
        if seek_offset < 0 {
            // If seeking backwards within the peek buffer, this would require a different approach
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Seeking backwards not supported",
            ))
        } else {
            // Clear the peek buffer since we're seeking past it
            self.peek_buffer.clear();
            // Seek in the underlying stream
            self.inner.seek(SeekFrom::Current(seek_offset))
        }
    }
}

impl<R: Read + Seek> Read for PeekableStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.peek_buffer.len() >= buf.len() {
            let n = buf.len();
            for (i, byte) in self.peek_buffer.drain(0..n).enumerate() {
                buf[i] = byte;
            }
            return Ok(n);
        }

        if !self.peek_buffer.is_empty() {
            let n = self.peek_buffer.len();
            for (i, byte) in self.peek_buffer.drain(..).enumerate() {
                buf[i] = byte;
            }
            let m = self.inner.read(&mut buf[n..])?;
            return Ok(n + m);
        }

        self.inner.read(buf)
    }
}
