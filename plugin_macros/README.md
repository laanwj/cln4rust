<div align="center">
  <h1>Rust core lightning plugin macros crate</h1>

  <p>
    <strong>Crate that provides a procedural macros implementation to make easy to develop a plugin developer to build a plugin.</strong>
  </p>

  <p>
  </p>

  <h4>
    <a href="https://github.com/laanwj/rust-clightning-rpc">Project Homepage</a>
  </h4>
 
  <a href="https://github.com/laanwj/rust-clightning-rpc/actions">
    <img alt="GitHub Workflow Status (branch)" src="https://img.shields.io/github/workflow/status/laanwj/rust-clightning-rpc/Integration%20testing/master?style=flat-square"/>
  </a>
  
  <a href="https://crates.io/clightningrpc-plugin_macros">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/clightningrpc-plugin_macros?style=flat-square"/>
  </a>

  
  <a href="https://docs.rs/clightningrpc">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/clightningrpc?style=flat-square"/>
  </a>

</div>

Crate that provides a procedural macros implementation to make easy to develop a plugin developer to build a plugin.

```rust
use serde_json::json;
use serde_json::Value;

use clightningrpc_plugin_macros::*;
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

#[notification(on = "rpc_command")]
fn on_rpc(plugin: &mut Plugin<State>, request: &Value) {
    use clightningrpc_plugin::types::LogLevel;
    plugin.log(LogLevel::Info, "received an RPC notification");
}

fn main() {
    let plugin = plugin! {
        state: State::new(),
        dynamic: true,
        notification: [
            on_rpc,
        ],
        methods: [
            foo_rpc,
        ],
        hooks: [],
    };
    plugin.start();
}
```

# Contributing guidelines

- Four spaces
- Call `make fmt` before committing
- If you can, GPG-sign at least your top commit when filing a PR

# Supports

If you want support this library consider to donate with the following methods

- Lightning address: vincenzopalazzo@coinos.io
- BOLT 12: https://bruce.lnmetrics.info/donation
- [Github donation](https://github.com/sponsors/vincenzopalazzo)
