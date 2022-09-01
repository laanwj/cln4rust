//! plugin crater provide the building blocks to build
//! plugin in core lightning in a modular way.
//!
//! ### Ecosystem feature
//! * **clightningrpc** -
//!  When enable provide the typed API to the plugin
//!
//! author and mantainer: Vincenzo Palazzo https://github.com/vincenzopalazzo
#![crate_name = "clightningrpc_plugin"]
#![feature(trait_alias)]
pub mod commands;
pub mod errors;
pub mod macros;
pub mod plugin;
pub mod types;
