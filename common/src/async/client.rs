//! Async Client support
//!
//! Support for connecting to JSONRPC servers over UNIX sockets asynchronously,
//! sending requests, and parsing responses using Tokio.
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Deserializer;

use crate::errors::Error;
use crate::types::{Request, Response};

/// A handle to a remote JSONRPC server for async operations
#[derive(Debug)]
pub struct AsyncClient {
    /// Path to the lightning-rpc socket file
    sockpath: PathBuf,
    /// Timeout for RPC request
    timeout: Option<Duration>,
}

impl AsyncClient {
    /// Creates a new async client using the path to the socket file and initializing the timeout field to None
    pub fn new<P: AsRef<Path>>(sockpath: P) -> AsyncClient {
        AsyncClient {
            sockpath: sockpath.as_ref().to_path_buf(),
            timeout: None,
        }
    }

    /// Set an optional timeout for requests
    pub fn set_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout = timeout;
    }

    /// Sends a request to a client asynchronously
    pub async fn send_request<S: Serialize, D: DeserializeOwned>(
        &self,
        method: &str,
        params: S,
    ) -> Result<Response<D>, Error> {
        // Setup connection
        let mut stream = UnixStream::connect(&self.sockpath).await?;
        if let Some(timeout) = self.timeout {
            tokio::time::timeout(timeout, async {
                // Serialize and send the request
                let request = Request {
                    method: method.to_owned(),
                    params,
                    id: Some("cln4rust/async/0".into()),
                    jsonrpc: "2.0".to_owned(),
                };
                let request_data = serde_json::to_vec(&request)?;
                stream.write_all(&request_data).await?;
                stream.flush().await?;

                // Read the response incrementally
                let mut response_data = Vec::new();
                let mut buffer = [0; 1024];
                loop {
                    let n = stream.read(&mut buffer).await?;
                    if n == 0 {
                        break;
                    }
                    response_data.extend_from_slice(&buffer[..n]);
                }

                // Deserialize the response
                let response: Response<D> = Deserializer::from_slice(&response_data)
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

                Ok(response)
            })
            .await
            .map_err(|_| {
                Error::Io(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "Request timed out",
                ))
            })?
        } else {
            // Serialize and send the request
            let request = Request {
                method: method.to_owned(),
                params,
                id: Some("cln4rust/async/0".into()),
                jsonrpc: "2.0".to_owned(),
            };
            let request_data = serde_json::to_vec(&request)?;
            stream.write_all(&request_data).await?;
            stream.flush().await?;

            // Read the response incrementally
            let mut response_data = Vec::new();
            let mut buffer = [0; 1024];
            loop {
                let n = stream.read(&mut buffer).await?;
                if n == 0 {
                    break;
                }
                response_data.extend_from_slice(&buffer[..n]);
            }

            // Deserialize the response
            let response: Response<D> = Deserializer::from_slice(&response_data)
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

            Ok(response)
        }
    }
}
