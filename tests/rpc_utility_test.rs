extern crate clightningrpc;

use clightningrpc::requests::AmountOrAll;
use clightningrpc::responses::NetworkAddress;
use clightningrpc::LightningRPC;
use rstest::*;
use std::path::Path;

#[fixture]
pub fn lightningd() -> LightningRPC {
    // FIXME(vincenzopalazzo):  Using the env to take the path of the RPC file.
    let sock = Path::new("/workdir/lightning_dir_two/regtest/lightning-rpc");
    let client = LightningRPC::new(&sock);
    client
}

#[fixture]
pub fn lightningd_second() -> LightningRPC {
    // FIXME(vincenzopalazzo):  Using the env to take the path of the RPC file.
    let sock = Path::new("/workdir/lightning_dir_one/regtest/lightning-rpc");
    let client = LightningRPC::new(&sock);
    client
}

#[rstest]
fn getinfo_test_one(lightningd: LightningRPC) {
    let get_info = lightningd.getinfo().unwrap();
    assert_eq!("regtest", get_info.network);
}

#[rstest]
fn listfunds_test_one(lightningd: LightningRPC) {
    let list_funds = lightningd.listfunds().unwrap();
    assert_eq!(0, list_funds.channels.len());
    assert_ne!(0, list_funds.outputs.len());
}

#[rstest]
fn connect_test_one(lightningd: LightningRPC, lightningd_second: LightningRPC) {
    let node_info = lightningd_second.getinfo().unwrap();
    let node_id = node_info.id;
    let addr = &node_info.binding[0];
    let mut host = "".to_string();
    let host = match addr {
        NetworkAddress::Ipv4 { address, port } => {
            host.push_str(&address.to_string());
            host.push_str(":");
            host.push_str(&port.to_string());
            host
        }
        _ => panic!("Network address unexpected"),
    };
    let connect_result = lightningd.connect(&node_id, Some(&host)).unwrap();
    assert_eq!(node_id, connect_result.id);
}

#[rstest]
fn fundchannel_test_one(lightningd: LightningRPC, lightningd_second: LightningRPC) {
    let info_node = lightningd_second.getinfo().unwrap();
    let node_id = info_node.id;
    let addr = &info_node.binding[0];
    let mut host = "".to_string();
    let host = match addr {
        NetworkAddress::Ipv4 { address, port } => {
            host.push_str(&address.to_string());
            host.push_str(":");
            host.push_str(&port.to_string());
            host
        }
        _ => panic!("unexpected addr type"),
    };
    let _ = lightningd.connect(&node_id, Some(&host)).unwrap();
    let fundchannel = lightningd
        .fundchannel(&node_id, AmountOrAll::Amount(800), None)
        .unwrap();
    assert!(fundchannel.txid.chars().count() == 64);
}
