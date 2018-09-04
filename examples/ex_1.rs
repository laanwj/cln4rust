extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate clightningrpc;

use clightningrpc::client;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkAddress {
    #[serde(rename="type")]
    pub type_: String,
    pub address: String,
    pub port: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetInfo {
    pub id: String,
    pub alias: String,
    pub color: String,
    pub address: Vec<NetworkAddress>,
    pub binding: Vec<NetworkAddress>,
    pub version: String,
    pub blockheight: i64,
    pub network: String,
}

fn main() {
    let client = client::Client::new("/home/user/.lightning/lightning-rpc".to_string());
    let request = client.build_request("getinfo".to_string(), vec![]);
    match client.send_request(&request).and_then(|res| res.into_result::<GetInfo>()) {
        Ok(d) => { println!("Ok! {:?}", d); }
        Err(e) => { println!("Error! {}", e); }
    }
}
