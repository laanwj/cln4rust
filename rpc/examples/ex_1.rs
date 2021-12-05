extern crate clightningrpc;

use std::env;

use clightningrpc::LightningRPC;

fn main() {
    let sock = env::home_dir().unwrap().join(".lightning/lightning-rpc");
    println!("Using socket {}", sock.display());

    let client = LightningRPC::new(&sock);

    println!("getinfo result: {:?}", client.getinfo().unwrap());

    for style in &["perkb", "perkw"] {
        println!("feerates {}: {:?}", style, client.feerates(style).unwrap());
    }
}
