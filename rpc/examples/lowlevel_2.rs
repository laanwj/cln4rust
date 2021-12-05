extern crate clightningrpc;
extern crate serde_json;

use std::env;

use clightningrpc::{client, requests, responses};

fn main() {
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());
    let client = client::Client::new(&sock);
    for style in &["perkb", "perkw"] {
        let method = "feerates";
        let params = requests::FeeRates { style: style };

        match client
            .send_request(method, params)
            .and_then(|res: clightningrpc::Response<responses::FeeRates>| res.into_result())
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
