use bitcoincore_rpc::{Client, RpcApi};
use futures::FutureExt;
use log::*;
use port_selector::random_free_tcp_port;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tempdir::TempDir;

use clightningrpc::LightningRPC;
use bitcoin::Address;

use cln_btc_test::runner::*;

fn setup_cln_backend(btc_rpc_port: u16, port: u16, rpc_port: u16) -> (Child, TempDir) {
    info!(
        "Starting regtest Core Lightning node on ports: {} and {} (RPC), Bitcoind port {}",
        port, rpc_port, btc_rpc_port,
    );
    let tmp_dir = TempDir::new("cln-backend").expect("temporary data dir created");
    let node_handle = Command::new("lightningd")
        .arg("--disable-plugin=offers")
        .arg("--disable-plugin=fetchinvoice")
        .arg("--disable-plugin=bookkeeper")
        .arg("--disable-plugin=cln-grpc")
        .arg("--disable-plugin=keysend")
        //.arg("--disable-plugin=topology")
        .arg("--disable-plugin=autoclean")
        .arg("--disable-plugin=commando")
        .arg("--disable-plugin=chanbackup")
        //.arg("--wallet=postgres://cln:cln@localhost:5432/cln-backend") // on Jan 25 2023 posgres is unable to work with CLN in tests. No good reason: bitcoin-cli plugin fails and CLN stops.
        .arg("--network=regtest")
        .arg("--bitcoin-rpcuser=regtest")
        .arg("--bitcoin-rpcpassword=regtest")
        .arg(format!("--bitcoin-rpcport={btc_rpc_port}"))
        .arg("--bitcoin-retry-timeout=120")
        .arg("--funding-confirms=1")
        .arg("--dev-bitcoind-poll=5")
        .arg("--dev-fast-gossip")
        .arg("--dev-no-htlc-timeout")
        .arg("--log-level=io")
        .arg("--log-file=/tmp/log-backend-cln")
        .arg(format!("--addr=127.0.0.1:{port}"))
        .arg(format!("--bind-addr=127.0.0.1:{rpc_port}"))
        .arg(format!("--lightning-dir={}", tmp_dir.path().to_str().unwrap()))
        .arg("--dev-force-privkey=0000000000000000000000000000000000000000000000000000000000000001")
        .arg("--dev-force-bip32-seed=0000000000000000000000000000000000000000000000000000000000000001")
        .arg("--dev-force-channel-secrets=0000000000000000000000000000000000000000000000000000000000000010/0000000000000000000000000000000000000000000000000000000000000011/0000000000000000000000000000000000000000000000000000000000000012/0000000000000000000000000000000000000000000000000000000000000013/0000000000000000000000000000000000000000000000000000000000000014/FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")
        .stdout(Stdio::null())
        .spawn()
        .expect("Core Lightning node starts");
    (node_handle, tmp_dir)
}

fn setup_cln_peernode(btc_rpc_port: u16, port: u16, rpc_port: u16) -> (Child, TempDir) {
    info!(
        "Starting regtest Core Lightning node on ports: {} and {} (RPC), Bitcoind port {}",
        port, rpc_port, btc_rpc_port,
    );
    let tmp_dir = TempDir::new("cln-peer").expect("temporary data dir created");
    let node_handle = Command::new("lightningd")
        .arg("--disable-plugin=offers")
        .arg("--disable-plugin=fetchinvoice")
        .arg("--disable-plugin=bookkeeper")
        .arg("--disable-plugin=cln-grpc")
        .arg("--disable-plugin=keysend")
        //.arg("--disable-plugin=topology")
        .arg("--disable-plugin=autoclean")
        .arg("--disable-plugin=commando")
        .arg("--disable-plugin=chanbackup")
        //.arg("--wallet=postgres://cln:cln@localhost:5432/cln-peer") // on Jan 25 2023 posgres is unable to work with CLN in tests. No good reason: bitcoin-cli plugin fails and CLN stops.
        .arg("--network=regtest")
        .arg("--bitcoin-rpcuser=regtest")
        .arg("--bitcoin-rpcpassword=regtest")
        .arg(format!("--bitcoin-rpcport={btc_rpc_port}"))
        .arg("--bitcoin-retry-timeout=120")
        .arg("--funding-confirms=1")
        .arg("--dev-bitcoind-poll=5")
        .arg("--dev-fast-gossip")
        .arg("--dev-no-htlc-timeout")
        .arg("--log-level=io")
        .arg("--log-file=/tmp/log-peer-cln")
        .arg(format!("--addr=127.0.0.1:{port}"))
        .arg(format!("--bind-addr=127.0.0.1:{rpc_port}"))
        .arg(format!("--lightning-dir={}", tmp_dir.path().to_str().unwrap()))
        .arg("--dev-force-privkey=0000000000000000000000000000000000000000000000000000000000000002")
        .arg("--dev-force-bip32-seed=0000000000000000000000000000000000000000000000000000000000000002")
        .arg("--dev-force-channel-secrets=0000000000000000000000000000000000000000000000000000000000000011/0000000000000000000000000000000000000000000000000000000000000111/0000000000000000000000000000000000000000000000000000000000000112/0000000000000000000000000000000000000000000000000000000000000113/0000000000000000000000000000000000000000000000000000000000000114/FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFA")
        .stdout(Stdio::null())
        .spawn()
        .expect("Core Lightning node starts");
    (node_handle, tmp_dir)
}

async fn wait_for_cln_node(client: &LightningRPC) {
    for _ in 0..100 {
        let res = client.getinfo();
        if res.is_ok() {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    client.getinfo().expect("final check on connection");
}

#[allow(clippy::expect_fun_call)]
async fn connect_cln_node(client: &LightningRPC, uri: &str) {
    client
        .connect(uri, None)
        .expect(format!("couldn't connect to {uri}").as_str());
}

async fn setup_cln_backend_ready(
    btc_client: &Client,
    btc_rpc_port: u16,
    port: u16,
    rpc_port: u16,
) -> (Child, LightningRPC, TempDir) {
    btc_client
        .get_blockchain_info()
        .expect("no connection with Bitcoind");

    let (node_handle, temp_dir) = setup_cln_backend(btc_rpc_port, port, rpc_port);
    let cln_socket = temp_dir.path().join("regtest/lightning-rpc");

    info!("Using CLN backend socket {}", cln_socket.display());
    let mut client = LightningRPC::new(cln_socket);
    client
        .client()
        .set_timeout(Some(Duration::from_millis(100))); // 1 might be too small

    wait_for_cln_node(&client).await;

    (node_handle, client, temp_dir)
}

async fn setup_cln_peernode_ready(
    btc_client: &Client,
    btc_rpc_port: u16,
    port: u16,
    rpc_port: u16,
) -> (Child, LightningRPC, TempDir) {
    btc_client
        .get_blockchain_info()
        .expect("no connection with Bitcoind");

    let (node_handle, temp_dir) = setup_cln_peernode(btc_rpc_port, port, rpc_port);
    let cln_socket = temp_dir.path().join("regtest/lightning-rpc");

    info!("Using CLN peer node socket {}", cln_socket.display());
    let mut client = LightningRPC::new(cln_socket);
    client
        .client()
        .set_timeout(Some(Duration::from_millis(100))); // 1 might be too small

    wait_for_cln_node(&client).await;

    (node_handle, client, temp_dir)
}

pub async fn mine_and_sync(btc: &Client, block_num: u64, address: &Address, ln_nodes: Vec<&LightningRPC>) -> u64
{
    let _r = btc
        .generate_to_address(block_num, address)
        .expect("New blocks");

    let chain_info = btc.get_blockchain_info().expect("blockchain info");

    for node in ln_nodes {
        let mut proceed = true;
        while proceed {
            let ln_info = node.getinfo().expect("LN node info");
            if chain_info.blocks != ln_info.blockheight {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                //debug!("Core height {}, LN {}", chain_info.blocks, ln_info.blockheight);
            } else {
                proceed = false;
            }
        }
    }
    chain_info.blocks
}

pub async fn wait_for_htlc(cln: &LightningRPC) -> u64
{
    // Ensures all HTLC settled
    // scids can be a list of strings. If unset wait on all channels.
    let peers = cln.listpeers(None, None).expect("All peers of the node");
    let mut msec = 0;
    for p in peers.peers {
        for c in p.channels {
            if c.htlcs.len() > 0 {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                msec = msec + 100;
            }
        }
    };
    msec
}


pub async fn run_cln_test<F, Fut>(test_body: F)
where
    F: FnOnce(Client, LightningRPC, LightningRPC) -> Fut,
    Fut: Future<Output = ()>,
{
    let _ = env_logger::builder().is_test(true).try_init();

    let btc_port = random_free_tcp_port().expect("available port");
    let btc_rpc_port = random_free_tcp_port().expect("available port");
    let (btc_handle, btc_client, _tmp_btc_dir) = setup_btc_node_ready(btc_port, btc_rpc_port).await;

    let cln_port = random_free_tcp_port().expect("available port");
    let cln_rpc_port = random_free_tcp_port().expect("available port");
    let (_, cln_back_client, _tmp_cln_back_dir) =
        setup_cln_backend_ready(&btc_client, btc_rpc_port, cln_port, cln_rpc_port).await;

    let cln_peer_port = random_free_tcp_port().expect("available port");
    let cln_peer_rpc_port = random_free_tcp_port().expect("available port");
    let (_, cln_peer_client, _tmp_cln_peer_dir) =
        setup_cln_peernode_ready(&btc_client, btc_rpc_port, cln_peer_port, cln_peer_rpc_port).await;

    let uri = format!("02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5@127.0.0.1:{cln_peer_port}");

    connect_cln_node(&cln_peer_client, uri.as_str()).await;
    //fund_cln_channel(&cln_peer_client, uri.as_str()).await;

    let res = AssertUnwindSafe(test_body(btc_client, cln_back_client, cln_peer_client))
        .catch_unwind()
        .await;

    println!("Tearing down BTC");
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    teardown_btc_node(btc_handle);
    assert!(res.is_ok());
}
