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
use clightningrpc_plugin_macros::{add_plugin_rpc, rpc_method};
use serde_json::{json, Value};

use clightningrpc_plugin::add_rpc;
use clightningrpc_plugin::commands::RPCCommand;
use clightningrpc_plugin::plugin::Plugin;

#[rpc_method(
rpc_name = "foo",
description = "This is a simple and short description"
)]
pub fn foo_rpc(_plugin: Plugin<()>, _request: Value) -> Value {
    /// The name of the parameters can be used only if used, otherwise can be omitted
    /// the only rules that the macros require is to have a propriety with the following rules:
    /// - Plugin as _plugin
    /// - CLN JSON request as _request
    /// The function parameter can be specified in any order.
    json!({"is_dynamic": _plugin.dynamic, "rpc_request": _request})
}

fn main() {
    // as fist step you need to make a new plugin instance
    // more docs about Plugin struct is provided under the clightning_plugin crate
    let mut plugin = Plugin::new((), true);

    // The macros helper that help to register a RPC method with the name
    // without worry about all the rules of the library
    add_plugin_rpc!(plugin, "foo");

    plugin.start();
}
```

# Contributing guidelines

- Four spaces
- Call `make fmt` before committing
- If you can, GPG-sign at least your top commit when filing a PR

# Supports

If you want support this library consider to donate with the following methods

- Lightning address: vincenzopalazzo@lntxbot.com
- [Github donation](https://github.com/sponsors/vincenzopalazzo)
