extern crate clightningrpc;

use std::path::Path;
use clightningrpc::LightningRPC;

#[test]
fn getinfo_test_one() {
    // FIXME(vincenzopalazzo):  Using the env to take the path of the RPC file.
    let sock = Path::new("/workdir/lightning_dir_two/regtest/lightning-rpc");
    // let sock = Path::new("/media/vincent/Maxtor/C-lightning/node/bitcoin/lightning-rpc");
    let client = LightningRPC::new(&sock);

    let get_info = client.getinfo().unwrap();
    assert_eq!("regtest", get_info.network);
}
