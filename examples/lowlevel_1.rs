extern crate clightningrpc;

use std::env;

use clightningrpc::{client, requests, responses};

fn main() {
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());
    let client = client::Client::new(&sock);
    let method = "getinfo";
    let params = requests::GetInfo {};
    match client
        .send_request(method, params)
        .and_then(|res: clightningrpc::Response<responses::GetInfo>| res.into_result())
    {
        Ok(d) => {
            println!("Ok! {:?}", d);
        }
        Err(e) => {
            println!("Error! {}", e);
        }
    }
}
