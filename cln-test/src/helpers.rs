use bitcoincore_rpc::bitcoin::Address;
use bitcoincore_rpc::{Client, RpcApi};
use std::str::FromStr;

use clightningrpc::LightningRPC;

pub fn fund_node_wallet(client: &Client, block_num: u64, ln_client: &LightningRPC) {
    let new_address = ln_client.newaddr(None).expect("Core node address");
    let str_address = new_address.address.expect("BTC address string");
    let address = Address::from_str(str_address.as_str()).expect("BTC address");
    client
        .generate_to_address(block_num, &address)
        .expect("mined blocks");
}
