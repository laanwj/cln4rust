extern crate clightningrpc_plugin;

use clightningrpc_common::client::Client;
use rstest::*;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

#[fixture]
pub fn lightningd() -> Client {
    // FIXME(vincenzopalazzo):  Using the env to take the path of the RPC file.
    let sock = Path::new("/workdir/lightning_dir_one/regtest/lightning-rpc");
    Client::new(&sock)
}

#[rstest]
fn plugin_macros_rpc_call_call(lightningd: Client) {
    let response = lightningd
        .send_request::<HashMap<String, Value>, HashMap<String, Value>>("foo_macro", HashMap::new())
        .unwrap();
    assert!(response.result.unwrap().contains_key("is_dynamic"));
}
