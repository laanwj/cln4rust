extern crate clightningrpc;

use clightningrpc::client;
use clightningrpc::responses::GetInfo;

fn main() {
    let client = client::Client::new("/home/user/.lightning/lightning-rpc".to_string());
    let request = client.build_request("getinfo".to_string(), vec![]);
    match client.send_request(&request).and_then(|res| res.into_result::<GetInfo>()) {
        Ok(d) => { println!("Ok! {:?}", d); }
        Err(e) => { println!("Error! {}", e); }
    }
}
