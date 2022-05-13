extern crate clightningrpc_plugin;

use clightningrpc_plugin::{commands::RPCMethod, plugin::Plugin};
use serde_json::{json, Value};

#[derive(Clone)]
struct PluginState(());

#[derive(Clone)]
struct HelloRPC {}

impl RPCMethod<PluginState> for HelloRPC {
    fn call<'c>(&self, _plugin: &mut Plugin<PluginState>, _request: &'c Value) -> Value {
        json!({
            "language": "Hello from rust"
        })
    }
}

fn main() {
    let mut plugin = Plugin::<PluginState>::new(PluginState(()), true)
        .add_rpc_method(
            "hello",
            "",
            "show how is possible add a method",
            HelloRPC {},
        )
        .add_opt(
            "foo",
            "flag",
            None,
            "An example of command line option",
            false,
        )
        .clone();
    plugin.start();
}
