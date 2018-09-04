extern crate clightningrpc;

use clightningrpc::lightningrpc::LightningRPC;

fn main() {
    let mut client = LightningRPC::new("/home/user/.lightning/lightning-rpc".to_string());

    println!("getinfo result: {:?}", client.getinfo().unwrap());

    for style in &["perkb", "perkw"] {
        println!("feerates {}: {:?}", style, client.feerates(style).unwrap());
    }
}
