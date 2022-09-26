/// Common crate that implements all the primitives
/// to interact with the lightning-rpc socket
///
/// author: https://github.com/vincenzopalazzo
extern crate serde;
extern crate serde_json;

pub mod client;
pub mod errors;
pub mod types;
