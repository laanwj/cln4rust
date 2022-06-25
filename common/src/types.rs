/// Common types used in the common crate
///
/// author: https://github.com/vincenzopalazzo
use serde::{Deserialize, Serialize};

use crate::errors::{Error, RpcError};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A JSONRPC request object
pub struct Request<'f, T: Serialize> {
    /// The name of the RPC call
    pub method: &'f str,
    /// Parameters to the RPC call
    pub params: T,
    /// Identifier for this Request, which should appear in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    /// jsonrpc field, MUST be "2.0"
    pub jsonrpc: &'f str,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
/// A JSONRPC response object
pub struct Response<T> {
    /// A result if there is one, or null
    pub result: Option<T>,
    /// An error if there is one, or null
    pub error: Option<RpcError>,
    /// Identifier for this Request, which should match that of the request
    pub id: u64,
    /// jsonrpc field, MUST be "2.0"
    pub jsonrpc: Option<String>,
}

impl<T> Response<T> {
    /// Extract the result from a response, consuming the response
    pub fn into_result(self) -> Result<T, Error> {
        if let Some(e) = self.error {
            return Err(Error::Rpc(e));
        }

        self.result.ok_or(Error::NoErrorOrResult)
    }

    /// Returns whether or not the `result` field is empty
    pub fn is_none(&self) -> bool {
        self.result.is_none()
    }
}
