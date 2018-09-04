extern crate clightningrpc;
extern crate strason;

use strason::Json;

use clightningrpc::{client, requests, responses};

fn main() {
    let client = client::Client::new("/home/user/.lightning/lightning-rpc".to_string());
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
