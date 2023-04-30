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

#[rpc_method(
    rpc_name = "foo_macro",
    description = "This is a simple and short description"
)]
pub fn foo_rpc(plugin: &mut Plugin<State>, request: Value) -> Result<Value, PluginError> {
    let response = json!({"is_dynamic": plugin.dynamic, "rpc_request": request});
    Ok(response)
}

#[notification(on = "rpc_command")]
fn on_rpc(plugin: &mut Plugin<State>, request: &Value) {
    use clightningrpc_plugin::types::LogLevel;
    plugin.log(LogLevel::Info, "received an RPC notification");
}

fn main() {
    // as fist step you need to make a new plugin instance
    // more docs about Plugin struct is provided under the clightning_plugin crate
    let mut plugin = Plugin::new(State, true);

    // FIXME: this is just for now, we will write a plugin macros
    // that define the definition like in the linux kernel a module is
    // defined.
    //
    // ```
    // module! {
    //  type: RustMinimal,
    //  name: "rust_minimal",
    //  author: "Rust for Linux Contributors",
    //  description: "Rust minimal sample",
    //  license: "GPL",
    // }
    // ```
    let call = on_rpc();
    plugin.register_notification(&call.on_event.clone(), call);
    let call = foo_rpc();
    plugin.add_rpc_method(
        &call.name.clone(),
        &call.usage.clone(),
        &call.description.clone(),
        call,
    );
    plugin.start();
}
