extern crate clightningrpc;
extern crate strason;

use std::env;

use strason::Json;

use clightningrpc::{client, requests, responses};

fn main() {
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());
    let client = client::Client::new(&sock);
    for style in &["perkb", "perkw"] {
        let params = Json::from_serialize(requests::FeeRates {
            style: style,
        }).unwrap();
        let request = client.build_request("feerates".to_string(), params);
        match client
            .send_request(&request)
            .and_then(|res| res.into_result::<responses::FeeRates>())
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
