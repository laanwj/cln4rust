# Rust c-lightning client

This provides an interface from rust to the [c-lightning](https://github.com/ElementsProject/lightning) daemon.

```rust
extern crate clightningrpc;

use clightningrpc::lightningrpc::LightningRPC;

fn main() {
    let mut client = LightningRPC::new("/home/user/.lightning/lightning-rpc".to_string());

    println!("getinfo result: {:?}", client.getinfo().unwrap());
}
```

See [examples](examples/) directory for more usage examples.

Not all calls supported by c-clightning have been implemented on the high-level interface
`LightningRPC` yet. Contributions are welcome!

# Credits

This library is based on Andre Poelstra's [rust-jsonrpc](https://github.com/apoelstra/rust-jsonrpc).
