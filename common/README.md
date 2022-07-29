<div align="center">
  <h1>Rust core lightning common client</h1>

  <p>
    <strong>This crate provides an generic interface from rust to the core lightning daemon through RPC with a generic interface.</strong>
  </p>

  <p>
  </p>

  <h4>
    <a href="https://github.com/laanwj/rust-clightning-rpc">Project Homepage</a>
  </h4>
 
  <a href="https://github.com/laanwj/rust-clightning-rpc/actions">
    <img alt="GitHub Workflow Status (branch)" src="https://img.shields.io/github/workflow/status/laanwj/rust-clightning-rpc/Integration%20testing/master?style=flat-square"/>
  </a>
  
  <a href="https://crates.io/clightningrpc">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/clightningrpc-common?style=flat-square">
  </a>

  <a href="https://docs.rs/clightningrpc">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/clightningrpc?style=flat-square"/>
  </a>

</div>

This crate provides an generic interface from rust to the [c-lightning](https://github.com/ElementsProject/lightning) daemon through RPC _with a generic interface_.

From the crate [clightningrpc](../rpc) you can find this quote

>Be aware that the API (of rust-clighting-rpc, but also that of c-lightning
itself) is not finalized. This means that it may change from version to version and break your
compile, sorry!

This crate solve the versioning with core lightning by offering a strongly type library with a generic interface, an example can be:

```rust
extern crate clightningrpc_common;

use serde::{Deserialize, Serialize};
use std::env;

use clightningrpc_common::client;
use clightningrpc_common::types::Response;

/// Example of type definition
#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetInfoResponse {
    pub id: String,
}

/// Example of type definition
#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetInfoRequest {}

fn main() {
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());
    let client = client::Client::new(&sock);
    let method = "getinfo";
    let params = GetInfoRequest {};
    match client
        .send_request(method, params)
        .and_then(|res: Response<GetInfoResponse>| res.into_result())
    {
        Ok(d) => {
            println!("Ok! {:?}", d);
        }
        Err(e) => {
            println!("Error! {}", e);
        }
    }
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

# Credits

This library is based on Andrew Poelstra's [rust-jsonrpc](https://github.com/apoelstra/rust-jsonrpc).
