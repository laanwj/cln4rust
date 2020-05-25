// Rust JSON-RPC Library
// Written by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//     Wladimir J. van der Laan <laanwj@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Async client support
//!
//! TODO: write comment
//!

use futures::io::BufReader;
use futures::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, ReadHalf};
use futures::io::{AsyncWrite, AsyncWriteExt, WriteHalf};

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, to_string};

use crate::error::Error;
use crate::{Request, Response};

/// A handle to a remote JSONRPC server
#[derive(Debug)]
pub struct Client<U: AsyncRead + AsyncWrite> {
    read_sock: BufReader<ReadHalf<U>>,
    write_sock: WriteHalf<U>,
    next_id: u64,
}

impl<U> Client<U>
where
    U: AsyncRead + AsyncWrite + Unpin,
{
    /// Creates a new client
    pub fn new(sock: U) -> Self {
        let (r, w) = sock.split();
        Client {
            read_sock: BufReader::new(r),
            write_sock: w,
            next_id: 0,
        }
    }

    /// Sends a request to a client
    pub async fn send_request<S: Serialize, D: DeserializeOwned>(
        &mut self,
        method: &str,
        params: S,
    ) -> Result<Response<D>, Error> {
        let expected_id = self.next_id;
        self.next_id += 1;

        let serialized = to_string(&Request {
            method,
            params,
            id: expected_id,
            jsonrpc: "2.0",
        })?;

        self.write_sock.write_all(serialized.as_bytes()).await?;

        let mut line = String::new();
        loop {
            self.read_sock.read_line(&mut line).await?;
            if !line.chars().all(|c| c.is_whitespace()) {
                break;
            }
        }
        let response: Response<D> = from_str(&line)?;

        if response
            .jsonrpc
            .as_ref()
            .map_or(false, |version| version != "2.0")
        {
            return Err(Error::VersionMismatch);
        }

        if response.id != expected_id {
            return Err(Error::NonceMismatch);
        }

        Ok(response)
    }
}
