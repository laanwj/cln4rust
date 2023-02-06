use bitcoin::Address;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use futures::FutureExt;
use log::*;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use port_selector::random_free_tcp_port;
use std::future::Future;
use std::panic::AssertUnwindSafe;
use std::process::{Child, Command, Stdio};
use tempdir::TempDir;

fn setup_btc_node(port: u16, rpc_port: u16) -> (Child, TempDir) {
    info!(
        "Starting regtest node on ports: {}, {} (RPC)",
        port, rpc_port
    );
    let tmp_dir = TempDir::new("regtest-data").expect("temporary data dir created");
    let node_handle = Command::new("bitcoind")
        .arg("-regtest")
        .arg("-server")
        .arg("-rpcuser=regtest")
        .arg("-rpcpassword=regtest")
        .arg("-fallbackfee=0.000002")
        .arg(format!("-port={port}"))
        .arg(format!("-rpcport={rpc_port}"))
        .arg(format!("-datadir={}", tmp_dir.path().to_str().unwrap()))
        .stdout(Stdio::null())
        .spawn()
        .expect("bitcoin node starts");
    (node_handle, tmp_dir)
}

pub fn teardown_btc_node(mut node_handle: Child) {
    info!("Teardown regtest node");
    signal::kill(Pid::from_raw(node_handle.id() as i32), Signal::SIGTERM).unwrap();
    node_handle.wait().expect("Node terminated");
}

pub async fn setup_btc_node_ready(port: u16, rpc_port: u16) -> (Child, Client, TempDir) {
    let (node_handle, temp_dir) = setup_btc_node(port, rpc_port);

    let rpc_url = format!("http://127.0.0.1:{rpc_port}/wallet/default");
    let client = Client::new(
        &rpc_url,
        Auth::UserPass("regtest".to_owned(), "regtest".to_owned()),
    )
        .expect("Node client");
    wait_for_btc_node(&client).await;
    client
        .create_wallet("default", None, None, None, None)
        .map(|_| ())
        .unwrap_or_else(|e| warn!("Cannot create default wallet: {}", e));
    (node_handle, client, temp_dir)
}

pub async fn wait_for_btc_node(client: &Client) {
    for _ in 0..100 {
        let res = client.generate(1, None);
        if res.is_ok() {
            return;
        }
        let res = client.get_blockchain_info();
        if res.is_ok() {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    client
        .get_blockchain_info()
        .expect("final check on connection");
}

pub async fn generate_to_address(client: &Client, address: Address) {
    client
        .generate_to_address(101, &address)
        .expect("generate to address failed");
}

pub async fn run_btc_test<F, Fut>(test_body: F)
    where
        F: FnOnce(Client) -> Fut,
        Fut: Future<Output = ()>,
{
    let _ = env_logger::builder().is_test(true).try_init();
    let node_port = random_free_tcp_port().expect("available port");
    let node_rpc_port = random_free_tcp_port().expect("available port");
    let (node_handle, client, _tmp_dir) = setup_btc_node_ready(node_port, node_rpc_port).await;
    //let api_port = setup_api(node_rpc_port).await;
    //info!("Running API server on {api_port}");
    //let api_client = BtcClient::new(&format!("http://127.0.0.1:{api_port}"));
    //let res = AssertUnwindSafe(test_body(client, api_client))
    let res = AssertUnwindSafe(test_body(client)).catch_unwind().await;
    teardown_btc_node(node_handle);
    assert!(res.is_ok());
}

pub async fn run_two_nodes_test<F, Fut>(test_body: F)
    where
        F: FnOnce(Client, Client) -> Fut,
        Fut: Future<Output = ()>,
{
    let _ = env_logger::builder().is_test(true).try_init();

    let node_1_port = random_free_tcp_port().expect("available port");
    let node_1_rpc_port = random_free_tcp_port().expect("available port");
    let (node_1_handle, client_1, _tmp1) = setup_btc_node_ready(node_1_port, node_1_rpc_port).await;

    let node_2_port = random_free_tcp_port().expect("available port");
    let node_2_rpc_port = random_free_tcp_port().expect("available port");
    let (node_2_handle, client_2, _tmp2) = setup_btc_node_ready(node_2_port, node_2_rpc_port).await;

    client_1
        .add_node(&format!("127.0.0.1:{node_2_port}"))
        .unwrap();

    let res = AssertUnwindSafe(test_body(client_1, client_2))
        .catch_unwind()
        .await;
    teardown_btc_node(node_1_handle);
    teardown_btc_node(node_2_handle);
    assert!(res.is_ok());
}

// This function is used for manual testing.
// It starts 2 BTC regtest nodes so we can top up the
// user's balance from an external wallet.
// It also starts instance of hexstody-btc API.
pub async fn run_regtest<F, Fut>(body: F)
    where
        F: FnOnce((u16, Client), (u16, Client)) -> Fut,
        Fut: Future<Output = ()>,
{
    // Start 1st BTC node
    let node_1_port = 9803;
    let node_1_rpc_port = 9804;
    let (node_1_handle, client_1, _tmp1) = setup_btc_node_ready(node_1_port, node_1_rpc_port).await;

    // Start 2nd BTC node
    let node_2_port = 9805;
    let node_2_rpc_port = 9806;
    let (node_2_handle, client_2, _tmp2) = setup_btc_node_ready(node_2_port, node_2_rpc_port).await;

    // Connect them together
    client_1
        .add_node(&format!("127.0.0.1:{node_2_port}"))
        .unwrap_or_else(|e| info!("Failed to connect nodes: {}!", e));

    // Start btc API and connect it to 1st BTC node.
    /*
    let api_port = 9802;
    let polling_duration = Duration::from_secs(300);
    setup_api_regtest(
        operator_api_domain,
        operator_public_keys,
        node_1_rpc_port,
        api_port,
        polling_duration,
    )
    .await;
    let api_url = format!("http://127.0.0.1:{api_port}");
    let api_client = BtcClient::new(&api_url);
    */

    body(
        (node_1_rpc_port, client_1),
        (node_2_rpc_port, client_2),
        //(api_url, api_client),
    )
        .await;
    teardown_btc_node(node_1_handle);
    teardown_btc_node(node_2_handle);
}
