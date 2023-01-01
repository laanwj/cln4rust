extern crate clightningrpc_common;

use serde::{Deserialize, Serialize};
use std::env;

use clightningrpc_common::client;
use clightningrpc_common::types::Response;

/// Example of type definition
#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetInfoResponse {
    pub id: String,
}

/// Example of type definition
#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetInfoRequest {}

fn main() {
    #[allow(deprecated)]
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());
    let client = client::Client::new(&sock);
    let method = "getinfo";
    let params = GetInfoRequest {};
    match client
        .send_request(method, params)
        .and_then(|res: Response<GetInfoResponse>| res.into_result())
    {
        Ok(d) => {
            println!("Ok! {:?}", d);
        }
        Err(e) => {
            println!("Error! {}", e);
        }
    }
}
