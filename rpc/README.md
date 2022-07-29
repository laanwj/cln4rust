<div align="center">
  <h1>Rust c-lightning client</h1>

  <p>
    <strong>This crate provides an interface from rust to the c-lightning daemon through RPC.</strong>
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
    <img alt="Crates.io" src="https://img.shields.io/crates/v/clightningrpc?style=flat-square"/>
  </a>
  
  <a href="https://docs.rs/clightningrpc">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/clightningrpc?style=flat-square"/>
  </a>

</div>

This crate provides an interface from rust to the [c-lightning](https://github.com/ElementsProject/lightning) daemon through RPC.

```rust
extern crate clightningrpc;
use std::env;
use clightningrpc::LightningRPC;

fn main() {
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    let mut client = LightningRPC::new(&sock);

    println!("getinfo result: {:?}", client.getinfo().unwrap());
}
```

See [examples](examples/) directory for more usage examples. To build and run an example do `cargo run --example ex_1`.
API documentation for the lastest version can be found on [docs.rs](https://docs.rs/clightningrpc/latest/clightningrpc/).

Currently implemented (this covers all non-dev commands as of c-lightning v0.6.1rc1):

- `getinfo`
- `feerates`
- `listnodes`
- `listchannels`
- `help`
- `getlog`
- `listconfigs`
- `listpeers`
- `listinvoices`
- `invoice`
- `delinvoice`
- `delexpiredinvoice`
- `autocleaninvoice`
- `waitanyinvoice`
- `waitinvoice`
- `pay`
- `sendpay`
- `waitsendpay`
- `listpayments`
- `decodepay`
- `getroute`
- `connect`
- `disconnect`
- `fundchannel`
- `close`
- `ping`
- `listfunds`
- `withdraw`
- `newaddr`
- `stop`

Be aware that the API (of rust-clighting-rpc, but also that of c-lightning
itself) is not finalized. This means that it may change from version to version and break your
compile, sorry!

N.B: A good solution if you have some missing compatibility between core lightning and the rust library, considering to use the [common crate](../common).

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
