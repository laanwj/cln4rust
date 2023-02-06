use bitcoincore_rpc::RpcApi;
use cln_btc_test::runner::run_btc_test;

// Check that we have node and API operational
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn basic_test() {
    run_btc_test(|btc| async move {
        println!("Running basic test");
        let info = btc.get_blockchain_info().expect("blockchain info");
        assert_eq!(info.chain, "regtest");
    })
    .await;
}
