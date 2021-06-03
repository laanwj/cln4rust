extern crate clightningrpc;

use std::path::Path;
use clightningrpc::LightningRPC;
use rstest::*;

#[fixture]
pub fn lightningd() -> LightningRPC {
    // FIXME(vincenzopalazzo):  Using the env to take the path of the RPC file.
    let sock = Path::new("/workdir/lightning_dir_two/regtest/lightning-rpc");
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
