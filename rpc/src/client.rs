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

//! Client support
//!
//! Support for connecting to JSONRPC servers over UNIX socets, sending requests,
//! and parsing responses
//!

use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{to_writer, Deserializer};

use super::{Request, Response};
use crate::error::Error;

/// A handle to a remote JSONRPC server
#[derive(Debug)]
pub struct Client {
    sockpath: PathBuf,
    timeout: Option<Duration>,
}

impl Client {
    /// Creates a new client
    pub fn new<P: AsRef<Path>>(sockpath: P) -> Client {
        Client {
            sockpath: sockpath.as_ref().to_path_buf(),
            timeout: None,
        }
    }

    /// Set an optional timeout for requests
    pub fn set_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout = timeout;
    }

    /// Sends a request to a client
    pub fn send_request<S: Serialize, D: DeserializeOwned>(
        &self,
        method: &str,
        params: S,
    ) -> Result<Response<D>, Error> {
        // Setup connection
        let mut stream = UnixStream::connect(&self.sockpath)?;
        stream.set_read_timeout(self.timeout)?;
        stream.set_write_timeout(self.timeout)?;

        to_writer(
            &mut stream,
            &Request {
                method,
                params,
                id: 0, // we always open a new connection, so we don't have to care about the nonce
                jsonrpc: "2.0",
            },
        )?;

        let response: Response<D> = Deserializer::from_reader(&mut stream)
            .into_iter()
            .next()
            .map_or(Err(Error::NoErrorOrResult), |res| Ok(res?))?;
        if response
            .jsonrpc
            .as_ref()
            .map_or(false, |version| version != "2.0")
        {
            return Err(Error::VersionMismatch);
        }

        // nonce will always be 0
        if response.id != 0 {
            return Err(Error::NonceMismatch);
        }

        Ok(response)
    }
}
