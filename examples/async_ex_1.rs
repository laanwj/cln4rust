extern crate clightningrpc;

use std::env;

use tokio::net::UnixStream;
use tokio_util::compat::Tokio02AsyncReadCompatExt;

use clightningrpc::aio::LightningRPC;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());
    let stream = UnixStream::connect(sock).await?;
    let mut client = LightningRPC::new(stream.compat());

    println!("getinfo result: {:#?}", client.getinfo().await?);

    for style in &["perkb", "perkw"] {
        println!("feerates {}: {:#?}", style, client.feerates(style).await?);
    }

    Ok(())
}
