//! plugin macros usage example.
extern crate plugin_macros;
use plugin_macros::rpc_method;
use serde_json::{json, Value};

use clightningrpc_plugin::commands::RPCMethod;
use clightningrpc_plugin::plugin::Plugin;

#[rpc_method(rpc_name = "foo", _description = "")]
pub fn foo_rpc() -> Value {
    json!({})
}

fn main() {}
