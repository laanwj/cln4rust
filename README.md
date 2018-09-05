# Rust c-lightning client

[![Status](https://travis-ci.org/laanwj/rust-clightning-rpc.png?branch=master)](https://travis-ci.org/laanwj/rust-clightning-rpc)

This crate provides an interface from rust to the [c-lightning](https://github.com/ElementsProject/lightning) daemon through RPC.

```rust
extern crate clightningrpc;

use clightningrpc::LightningRPC;

fn main() {
    let mut client = LightningRPC::new("/home/user/.lightning/lightning-rpc".to_string());

    println!("getinfo result: {:?}", client.getinfo().unwrap());
}
```

See [examples](examples/) directory for more usage examples.

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

- API is inconvenient when there are a lot of optional arguments, for example `pay` is awful:

```
let pay_result = client_from.pay(invoice.bolt11, None, None, None, None, None, None, None);
```

because Rust has no built-in support for optional arguments, or even variable
number of arguments (?), nor named ones. This also give a lack of
extenisibility in case upstream `lightningd` adds more arguments in the future.
Not sure how to handle this, a 'builder pattern' has been suggested but I'm not
sure how to build a good API around this. If anyone has suggestions please let me know!

- the API could make more use of enums where the possible values are known; for example the
  `addresstype` parameter to `newaddr`, but also in returned structures. This has to be weighted
  agains flexibility, though, in case the API is extended later.

- decide on `&str` versus `String` on high-level API (but at least make sure it is consistent)

# Style guidelines

- Four spaces
- Call `rustfmt src/*.rs` before committing

# Credits

This library is based on Andrew Poelstra's [rust-jsonrpc](https://github.com/apoelstra/rust-jsonrpc).
