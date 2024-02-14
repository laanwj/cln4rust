//! plugin macros usage example.
extern crate clightningrpc_plugin_macros;
use clightningrpc_plugin_macros::*;

use serde_json::json;
use serde_json::Value;

use clightningrpc_plugin::commands::RPCCommand;
use clightningrpc_plugin::errors::PluginError;
use clightningrpc_plugin::plugin::Plugin;

#[derive(Clone)]
struct State;

// FIXME: implement a derive macros to register
// the option plugins
impl State {
    pub fn new() -> Self {
        Self
    }
}

#[rpc_method(
    rpc_name = "foo_macro",
    description = "This is a simple and short description"
)]
pub fn foo_rpc(plugin: &mut Plugin<State>, request: Value) -> Result<Value, PluginError> {
    let response = json!({"is_dynamic": plugin.dynamic, "rpc_request": request});
    Ok(response)
}

#[notification(on = "warning")]
fn on_warning(plugin: &mut Plugin<State>, request: &Value) {
    use clightningrpc_plugin::types::LogLevel;
    plugin.log(LogLevel::Info, "received an RPC notification");
}

#[notification(on = "shutdown")]
fn on_shutdown(_: &mut Plugin<State>, _: &Value) {
    std::process::exit(0);
}

fn main() {
    let plugin = plugin! {
        state: State::new(),
        dynamic: true,
        notification: [
            on_warning,
            on_shutdown,
        ],
        methods: [
            foo_rpc,
        ],
        hooks: [],
    };
    plugin.start();
}
