extern crate clightningrpc_common;
extern crate serde_json;

use std::collections::HashMap;
use std::env;

use clightningrpc_common::{client, types};

fn main() {
    #[allow(deprecated)]
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());
    let client = client::Client::new(&sock);
    for style in &["perkb", "perkw"] {
        let method = "feerates";
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("style".to_string(), style.to_string());
        match client
            .send_request(method, params)
            .and_then(|res: types::Response<HashMap<String, String>>| res.into_result())
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
