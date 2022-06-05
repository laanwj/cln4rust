//! plugin macros usage example.
extern crate plugin_macros;
use plugin_macros::rpc_method;
use serde_json::{json, Value};

use clightningrpc_plugin::commands::RPCCommand;
use clightningrpc_plugin::plugin::Plugin;

#[rpc_method(
    rpc_name = "foo",
    description = "This is a simple and short description"
)]
pub fn foo_rpc() -> Value {
    json!({})
}

fn main() {
    let mut plugin = Plugin::new((), true);
    plugin.add_rpc_method("foo", "", "", Foo::new()).start();
}
