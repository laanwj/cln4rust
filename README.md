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

Not all calls supported by c-clightning have been implemented on the high-level interface
`LightningRPC` yet. Contributions are welcome!

Currently implemented:

- `getinfo`
- `feerates`
- `listpeers`
- `listinvoices`
- `invoice`
- `delinvoice`
- `pay`
- `listpayments`
- `decodepay`
- `getroute`
- `connect`
- `disconnect`
- `stop`

TODO:

- `listnodes`
- `listchannels`
- `delexpiredinvoice`
- `autocleaninvoice`
- `waitanyinvoice`
- `waitinvoice`
- `help`
- `getlog`
- `fundchannel`
- `listconfigs`
- `sendpay`
- `waitsendpay`
- `close`
- `ping`
- `withdraw`
- `newaddr`
- `listfunds`
- potentially `dev-*`

# To do

- implement all commands on high-level interface
- document low and high level handling
- document error handling
- reproducible functional test that exercises against actual lightning instances
- a better way to get at the data for failed payments

```
"code" : 205, "message" : "Could not find a route", "data" : ...
```

  `data` could be parsed into a structure, but this depends on the kind of error

- API is inconvenient when there are a lot of optional arguments, this is awful:

```
let pay_result = client_from.pay(invoice.bolt11, None, None, None, None, None, None, None);
```

because Rust has no built-in support for optional arguments, or even variable
number of arguments (?), nor named ones. This also give a lack of
extenisibility in case upstream `lightningd` adds more arguments in the future.
Not sure how to handle this, a 'builder pattern' has been suggested but I'm not
sure how to build a good API around this. If anyone has suggestions please let me know!

# Credits

This library is based on Andrew Poelstra's [rust-jsonrpc](https://github.com/apoelstra/rust-jsonrpc).
