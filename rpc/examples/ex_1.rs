extern crate clightningrpc;

use std::env;

use clightningrpc::LightningRPC;

#[tokio::main]
async fn main() {
    #[allow(deprecated)]
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());

    let client = LightningRPC::new(&sock);

    #[cfg(feature = "async")]
    println!("getinfo result: {:?}", client.getinfo().await.unwrap());

    #[cfg(not(feature = "async"))]
    println!("getinfo result: {:?}", client.getinfo().unwrap());

    for style in &["perkb", "perkw"] {
        #[cfg(feature = "async")]
        println!(
            "feerates {}: {:?}",
            style,
            client.feerates(style).await.unwrap()
        );

        #[cfg(not(feature = "async"))]
        println!("feerates {}: {:?}", style, client.feerates(style).unwrap());
    }
}
