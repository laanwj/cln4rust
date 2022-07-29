<div align="center">
  <h1>Rust core lightning plugin crate</h1>

  <p>
    <strong>Crate that provides a procedural API to develop cln plugins.</strong>
  </p>

  <p>
  </p>

  <h4>
    <a href="https://github.com/laanwj/rust-clightning-rpc">Project Homepage</a>
  </h4>
 
  <a href="https://github.com/laanwj/rust-clightning-rpc/actions">
    <img alt="GitHub Workflow Status (branch)" src="https://img.shields.io/github/workflow/status/laanwj/rust-clightning-rpc/Integration%20testing/master?style=flat-square"/>
  </a>
  
  <a href="https://crates.io/clightningrpc-plugin">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/clightningrpc-plugin?style=flat-square"/>
  </a>
  
  <a href="https://docs.rs/clightningrpc">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/clightningrpc?style=flat-square"/>
  </a>

</div>

Crate that provides a procedural macros implementation to make easy to develop a plugin developer to build a plugin.

```rust
extern crate clightningrpc_plugin;

use clightningrpc_plugin::types::LogLevel;
use clightningrpc_plugin::{commands::RPCCommand, plugin::Plugin};
use serde_json::{json, Value};

#[derive(Clone)]
struct PluginState(());

/// HelloRPC is used to register the RPC method
#[derive(Clone)]
struct HelloRPC {}

/// Implementation of the RPC method
impl RPCCommand<PluginState> for HelloRPC {
    fn call<'c>(&self, plugin: &mut Plugin<PluginState>, _request: &'c Value) -> Value {
        plugin.log(LogLevel::Debug, "call the custom rpc method from rust");
        json!({
            "language": "Hello from rust"
        })
    }
}

#[derive(Clone)]
struct OnChannelOpened {}

impl RPCCommand<PluginState> for OnChannelOpened {
    fn call_void<'c>(&self, _plugin: &mut Plugin<PluginState>, _request: &'c Value) {
        _plugin.log(LogLevel::Debug, "A new channel was opened!");
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
        .register_notification("channel_opened", OnChannelOpened {})
        .clone();
    plugin.start();
}
```

# Contributing guidelines

Read our [Hacking guide](https://github.com/laanwj/rust-clightning-rpc/blob/master/docs/MAINTAINERS.md)

# Supports

If you want support this library consider to donate with the following methods

- Lightning address: vincenzopalazzo@lntxbot.com
- [Github donation](https://github.com/sponsors/vincenzopalazzo)
