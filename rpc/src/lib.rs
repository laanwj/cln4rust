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
//! This create provides both a high and a low-level interface.
//! Most likely, you'll want to use the high-level interface through `LightningRPC`, as this is
//! most convenient,
//! but it is also possible to construct Request and Response objects manually and
//! send them through the pipe.

#![crate_name = "clightningrpc"]
// Coding conventions
#![deny(missing_debug_implementations)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod error;
pub mod lightningrpc;
pub mod requests;
pub mod responses;
pub mod types;

// Re-export high-level connection type
pub use crate::lightningrpc::LightningRPC;
