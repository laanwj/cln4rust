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

//! # Client support
//!
//! Support for connecting to JSONRPC servers over UNIX socets, sending requests,
//! and parsing responses
//!

use std::io::Write;
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};

use strason::Json;

use super::{Request, Response};
use error::Error;

/// A handle to a remote JSONRPC server
pub struct Client {
    sockname: String,
    nonce: Arc<Mutex<u64>>,
}

/// Filter out top-level parameters with value None, this is used for handling optional
/// parameters correctly as c-lightning expects.
fn filter_nones(params: Json) -> Json {
    let mut rv: Vec<(String, Json)> = Vec::new();
    for (k, v) in params.object().unwrap() {
        if let None = v.null() {
            rv.push((k.clone(), v.clone()));
        }
    }
    Json::from(rv)
}

impl Client {
    /// Creates a new client
    pub fn new(sockname: String) -> Client {
        Client {
            sockname: sockname,
            nonce: Arc::new(Mutex::new(0)),
        }
    }

    /// Sends a request to a client
    pub fn send_request(&self, request: &Request) -> Result<Response, Error> {
        // Build request
        let request_raw = Json::from_serialize(request)?.to_bytes();

        // Setup connection
        let mut stream = UnixStream::connect(&self.sockname)?;

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

    #[test]
    fn sanity() {
        let client = Client::new("localhost".to_owned(), None, None);
        assert_eq!(client.last_nonce(), 0);
        let req1 = client.build_request("test".to_owned(), vec![]);
        assert_eq!(client.last_nonce(), 1);
        let req2 = client.build_request("test".to_owned(), vec![]);
        assert_eq!(client.last_nonce(), 2);
        assert!(req1 != req2);
    }
}
