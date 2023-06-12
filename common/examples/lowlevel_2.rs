extern crate clightningrpc_common;
extern crate serde_json;

use std::collections::HashMap;
use std::env;

use clightningrpc_common::{client, types};
use serde_json::{json, Value};

fn main() {
    #[allow(deprecated)]
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());
    let client = client::Client::new(&sock);
    for style in &["perkb", "perkw"] {
        let method = "feerates";
        let params = json!({
            "style": style,
        });
        match client
            .send_request(method, params)
            .and_then(|res: types::Response<HashMap<String, Value>>| res.into_result())
        {
            Ok(d) => {
                println!("Ok! {:?}", d);
            }
            Err(e) => {
                println!("Error! {}", e);
            }
        }
    }
}
