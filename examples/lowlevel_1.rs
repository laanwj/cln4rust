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
    let params = Json::from_serialize(requests::GetInfo {}).unwrap();
    let request = client.build_request("getinfo".to_string(), params);
    match client
        .send_request(&request)
        .and_then(|res| res.into_result::<responses::GetInfo>())
    {
        Ok(d) => {
            println!("Ok! {:?}", d);
        }
        Err(e) => {
            println!("Error! {}", e);
        }
    }
}
