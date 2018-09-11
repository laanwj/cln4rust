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

use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use strason::Json;

use super::{Request, Response};
use error::Error;

/// A handle to a remote JSONRPC server
#[derive(Debug)]
pub struct Client {
    sockpath: PathBuf,
    nonce: Arc<Mutex<u64>>,
    timeout: Option<Duration>,
}

/// Filter out top-level parameters with value None, this is used for handling optional
/// parameters correctly as c-lightning expects.
///
/// This function panics if passed Json isn't a json object.
fn filter_nones(params: Json) -> Json {
    let rv = params
        .object()
        .unwrap()
        .iter()
        .filter(|(_, v)| v.null().is_none())
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>();
    Json::from(rv)
}

impl Client {
    /// Creates a new client
    pub fn new<P: AsRef<Path>>(sockpath: P) -> Client {
        Client {
            sockpath: sockpath.as_ref().to_path_buf(),
            nonce: Arc::new(Mutex::new(0)),
            timeout: None,
        }
    }

    /// Set an optional timeout for requests
    pub fn set_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout = timeout;
    }

    /// Sends a request to a client
    pub fn send_request(&self, request: &Request) -> Result<Response, Error> {
        // Build request
        let request_raw = Json::from_serialize(request)?.to_bytes();

        // Setup connection
        let mut stream = UnixStream::connect(&self.sockpath)?;
        stream.set_read_timeout(self.timeout)?;
        stream.set_write_timeout(self.timeout)?;

        stream.write_all(&request_raw)?;

        let response: Response = Json::from_reader(&mut stream)?.into_deserialize()?;
        if response.jsonrpc != None && response.jsonrpc != Some(From::from("2.0")) {
            return Err(Error::VersionMismatch);
        }
        if response.id != request.id {
            return Err(Error::NonceMismatch);
        }

        Ok(response)
    }

    /// Builds a request
    pub fn build_request(&self, name: String, params: Json) -> Request {
        let mut nonce = self.nonce.lock().unwrap();
        *nonce += 1;

        Request {
            method: name,
            params: filter_nones(params),
            id: From::from(*nonce),
            jsonrpc: Some(String::from("2.0")),
        }
    }

    /// Accessor for the last-used nonce
    pub fn last_nonce(&self) -> u64 {
        *self.nonce.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strason::Json;

    #[test]
    fn sanity() {
        let client = Client::new("/tmp/socket/localhost");
        assert_eq!(client.last_nonce(), 0);
        let req1 =
            client.build_request("test".to_owned(), Json::from(Vec::<(String, Json)>::new()));
        assert_eq!(client.last_nonce(), 1);
        let req2 =
            client.build_request("test".to_owned(), Json::from(Vec::<(String, Json)>::new()));
        assert_eq!(client.last_nonce(), 2);
        assert!(req1 != req2);
    }
}
