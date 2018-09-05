extern crate clightningrpc;
extern crate strason;

use std::env;

use strason::Json;

use clightningrpc::{client, requests, responses};

fn main() {
    let mut sock = env::home_dir().unwrap();
    sock.push(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());
    let client = client::Client::new(&sock);
    for style in &["perkb", "perkw"] {
        let params = Json::from_serialize(requests::FeeRates {
            style: style.to_string(),
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
