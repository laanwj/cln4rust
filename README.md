<div align="center">
  <h1>core lightning Rust Framework</h1>

  <p>
    <strong>A collection of libraries to develop and work with core lighting.</strong>
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
  
  <a href="https://crates.io/clightningrpc">
    <img alt="Crates.io" src="https://img.shields.io/crates/d/clightningrpc?style=flat-square"/>
  </a>
  
  <a href="https://docs.rs/clightningrpc">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/clightningrpc?style=flat-square"/>
  </a>

</div>

This repository contains a sequence of craters that are usuefult to work with core lightning and develop with core lightning
using Rust.

## Craters

These are the complete list of craters supported right now

| Crate     | Description |  Version |
|:----------|:-----------:|--:|
| clightningrpc-common          |    Crate that provides an Generic RPC binding from rust code to the core lightning daemon    | ![Crates.io](https://img.shields.io/crates/v/clightningrpc-common?style=flat-square)  |
| clightningrpc |    Crate that provides a strong typed RPC binding from rust code to the core lightning daemon     | ![Crates.io](https://img.shields.io/crates/v/clightningrpc?style=flat-square) |
| clightningrpc-plugin |    Crate that provides a plugin API to give the possibility to implement a plugin in Rust     | ![Crates.io](https://img.shields.io/crates/v/clightningrpc-plugin?style=flat-square) |
| clightningrpc-plugin-macros |    Crate that provides a procedural macros implementation to make easy to develop a plugin developer to build a plugin     | ![Crates.io](https://img.shields.io/crates/v/clightningrpc-plugin_macros?style=flat-square) |

## Contributing guidelines

Read our [Hacking guide](/docs/MAINTAINERS.md)

## Supports

If you want support this library consider to donate with the following methods

- Lightning address: vincenzopalazzo@lntxbot.com
- [Github donation](https://github.com/sponsors/vincenzopalazzo)

## Credits

This library is based on Andrew Poelstra's [rust-jsonrpc](https://github.com/apoelstra/rust-jsonrpc).
