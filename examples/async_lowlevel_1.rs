extern crate clightningrpc;

use std::env;

use tokio::net::UnixStream;

use tokio_util::compat::Tokio02AsyncReadCompatExt;

use clightningrpc::aio::client;
use clightningrpc::{requests, responses};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());
    let stream = UnixStream::connect(sock).await?;
    let mut client = client::Client::new(stream.compat());

    match client
        .send_request("getinfo", requests::GetInfo {})
        .await
        .and_then(|res: clightningrpc::Response<responses::GetInfo>| res.into_result())
    {
        Ok(d) => {
            println!("Ok! {:#?}", d);
        }
        Err(e) => {
            println!("Error! {}", e);
        }
    }
    match client
        .send_request("listfunds", requests::ListFunds {})
        .await
        .and_then(|res: clightningrpc::Response<responses::ListFunds>| res.into_result())
    {
        Ok(d) => {
            println!("Ok! {:#?}", d);
        }
        Err(e) => {
            println!("Error! {}", e);
        }
    }
    Ok(())
}
