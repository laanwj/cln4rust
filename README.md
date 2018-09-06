# Rust c-lightning client

[![Status](https://travis-ci.org/laanwj/rust-clightning-rpc.png?branch=master)](https://travis-ci.org/laanwj/rust-clightning-rpc)
[![Crates.io](https://img.shields.io/crates/v/clightningrpc.svg)](https://crates.io/crates/clightningrpc)

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

# To do

- verify use of `i64` versus `u64` in API
- `fundchannel`, `withdraw`: allow passing `all` for `satoshi`
- document low and high level handling
- document error handling
- reproducible functional test that exercises against actual lightning instances (regtest?)
- a better way to get at the data for failed payments

```
"code" : 205, "message" : "Could not find a route", "data" : ...
```

  `data` could be parsed into a structure, but this depends on the kind of error

- the API could make more use of enums where the possible values are known; for example the
  `addresstype` parameter to `newaddr`, but also in returned structures. This has to be weighted
  against flexibility, though, in case the API is extended later.

- decide on `&str` versus `String` on high-level API (but at least make sure it is consistent)

# Style guidelines

- Four spaces
- Call `rustfmt src/*.rs` before committing

# Credits

This library is based on Andrew Poelstra's [rust-jsonrpc](https://github.com/apoelstra/rust-jsonrpc).
