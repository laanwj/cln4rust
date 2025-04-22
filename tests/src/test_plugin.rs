use std::collections::HashMap;

use rstest::*;
use serde_json::Value;

use clightning_testing::cln;

use crate::fixtures::lightningd;

#[rstest]
fn plugin_rpc_call_call(lightningd: cln::Node) {
    let lightningd = lightningd.rpc();
    let response = lightningd
        .call::<HashMap<String, Value>, HashMap<String, Value>>("hello", HashMap::new())
        .unwrap();
    assert!(response.contains_key("language"));
}

#[rstest]
fn plugin_macros_rpc_call_call(lightningd: cln::Node) {
    let rpc = lightningd.rpc();
    let response =
        rpc.call::<HashMap<String, Value>, HashMap<String, Value>>("foo_macro", HashMap::new());
    assert!(response.is_ok(), "{:#?}", response);
    let response = response.unwrap();
    assert!(response.contains_key("is_dynamic"));
}
