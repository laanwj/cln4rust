// Rust JSON-RPC Library
// Written in 2015 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
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

//! Crate that provides an RPC binding from rust code to the c-lightning daemon
//!
//! This crate provides both a high and a low-level interface.
//! Most likely, you'll want to use the high-level interface through `LightningRPC`, as this is
//! most convenient,
//! but it is also possible to construct Request and Response objects manually and
//! send them through the pipe.

#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![crate_name = "clightningrpc"]
// Coding conventions
#![deny(missing_debug_implementations)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod client;
pub mod common;
pub mod error;
pub mod lightningrpc;
pub mod requests;
pub mod responses;

pub mod aio;

// Re-export error type
pub use crate::error::Error;
// Re-export high-level connection type
pub use crate::lightningrpc::LightningRPC;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
/// A JSONRPC request object
pub struct Request<'f, T: Serialize> {
    /// The name of the RPC call
    pub method: &'f str,
    /// Parameters to the RPC call
    pub params: T,
    /// Identifier for this Request, which should appear in the response
    pub id: u64,
    /// jsonrpc field, MUST be "2.0"
    pub jsonrpc: &'f str,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
/// A JSONRPC response object
pub struct Response<T> {
    /// A result if there is one, or null
    pub result: Option<T>,
    /// An error if there is one, or null
    pub error: Option<error::RpcError>,
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
