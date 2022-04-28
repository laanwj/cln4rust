extern crate clightningrpc_plugin;

use clightningrpc_plugin::plugin::{IPlugin, Plugin};
use serde_json::Value;

struct PluginState(());

//#[rpc_method("foo_rpc")]
fn foo_rpc(_plugin: &mut Plugin<PluginState>, _request: &Value) {}

fn main() {
    let plugin = Plugin::<PluginState>::new();
    plugin.add_rpc_method(foo_rpc).start(&PluginState(()));
}
