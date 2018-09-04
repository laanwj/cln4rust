extern crate clightningrpc;
extern crate strason;

use strason::Json;

use clightningrpc::{client, requests, responses};

fn main() {
    let client = client::Client::new("/home/user/.lightning/lightning-rpc".to_string());
    let params = Json::from_serialize(requests::GetInfo {}).unwrap();
    let request = client.build_request("getinfo".to_string(), params);
    match client.send_request(&request).and_then(|res| res.into_result::<responses::GetInfo>()) {
        Ok(d) => { println!("Ok! {:?}", d); }
        Err(e) => { println!("Error! {}", e); }
    }
}
