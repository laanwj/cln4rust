extern crate clightningrpc_plugin;

use clightningrpc_plugin::types::LogLevel;
use clightningrpc_plugin::{commands::RPCCommand, errors::PluginError, plugin::Plugin};
use serde_json::{json, Value};

#[derive(Clone, Copy)]
struct PluginState(());

/// HelloRPC is used to register the RPC method
#[derive(Clone)]
struct HelloRPC {}

/// Implementation of the RPC method
impl RPCCommand<PluginState> for HelloRPC {
    fn call<'c>(
        &self,
        plugin: &mut Plugin<PluginState>,
        _request: Value,
    ) -> Result<Value, PluginError> {
        plugin.log(LogLevel::Debug, "call the custom rpc method from rust");
        let response = json!({
            "language": "Hello from rust"
        });
        Ok(response)
    }
}

#[derive(Clone)]
struct OnShutdown {}

impl RPCCommand<PluginState> for OnShutdown {
    fn call_void<'c>(&self, _: &mut Plugin<PluginState>, _: &'c Value) {
        std::process::exit(0);
    }
}

#[tokio::main]
async fn main() {
    let plugin = Plugin::<PluginState>::new(PluginState(()), true)
        .add_rpc_method(
            "hello",
            "",
            "show how is possible add a method",
            HelloRPC {},
        )
        .add_opt(
            "foo",
            "flag",
            Some("false".to_owned()),
            "An example of command line option",
            false,
        )
        .register_notification("shutdown", OnShutdown {})
        .on_init(|plugin: &mut Plugin<_>| -> serde_json::Value {
            plugin.log(LogLevel::Debug, "Custom init method called");
            json!({})
        })
        .clone();
    plugin.start().await;
}
