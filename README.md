# Rust c-lightning client

This provides an interface from rust to the [c-lightning](https://github.com/ElementsProject/lightning) daemon.

See [examples](examples/) directory for usage examples.

For example, `ex_1` will print `getinfo` output for the local lightning node.
```bash
$ cargo run --example ex_1
```

# Credits

This library is based on Andre Poelstra's [rust-jsonrpc](https://github.com/apoelstra/rust-jsonrpc).
