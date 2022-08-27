//! plugin macros usage example.
extern crate clightningrpc_plugin_macros;
use clightningrpc_plugin_macros::{
    add_plugin_rpc, notification, plugin_register_notification, rpc_method,
};
use serde_json::{json, Value};

use clightningrpc_plugin::commands::RPCCommand;
use clightningrpc_plugin::errors::PluginError;
use clightningrpc_plugin::plugin::Plugin;
use clightningrpc_plugin::types::LogLevel;
use clightningrpc_plugin::{add_rpc, register_notification};

#[rpc_method(
    rpc_name = "foo_macro",
    description = "This is a simple and short description"
)]
pub fn foo_rpc(_plugin: Plugin<()>, _request: Value) -> Result<Value, PluginError> {
    /// The name of the parameters can be used only if used, otherwise can be omitted
    /// the only rules that the macros require is to have a propriety with the following rules:
    /// - Plugin as _plugin
    /// - CLN JSON request as _request
    /// The function parameter can be specified in any order.
    let response = json!({"is_dynamic": _plugin.dynamic, "rpc_request": _request});
    Ok(response)
}

#[notification(on = "rpc_command")]
fn on_rpc(_plugin: Plugin<()>, _request: Value) {
    _plugin.log(LogLevel::Info, "received an RPC notification");
}

fn main() {
    // as fist step you need to make a new plugin instance
    // more docs about Plugin struct is provided under the clightning_plugin crate
    let mut plugin = Plugin::new((), true);

    // The macros helper that help to register a RPC method with the name
    // without worry about all the rules of the library
    add_plugin_rpc!(plugin, "foo_macro");

    // the macros helper that help to register a notification with the
    // event name without worry about the rules of the library :)
    plugin_register_notification!(plugin, "rpc_command");

    plugin.start();
}
