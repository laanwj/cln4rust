use bitcoincore_rpc::bitcoin::Address;
use bitcoincore_rpc::RpcApi;
use clightningrpc::lightningrpc::PayOptions;
use clightningrpc::requests::AmountOrAll;
use clightningrpc::responses::NetworkAddress::Ipv4;
use p256::elliptic_curve::weierstrass::add;
use std::str::FromStr;

use cln_btc_test::runner::*;
use cln_test::runner::*;

// Check that we have node and API operational
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn basic_test() {
    run_cln_test(|btc, cln_back, cln_peer| async move {
        println!("Getting info from Bitcoind");
        let info = btc.get_blockchain_info().expect("blockchain info");
        println!("Blockchain tip {}", info.blocks);
        assert_eq!(info.chain, "regtest");

        println!("Getting info from Core Lightning");
        let info = cln_back.getinfo().expect("blockchain info");
        println!("\tBackend node {:?}", info);
        assert_eq!(info.network, "regtest");
        let info = cln_peer.getinfo().expect("blockchain info");
        println!("\tPeer node {:?}", info);
        assert_eq!(info.network, "regtest");

        let a = cln_back.newaddr(None).expect("address wasn't provided");
        let addr_str = a.bech32.unwrap();
        println!("Depositing onto CLN backend address {}", addr_str);
        let addr = Address::from_str(addr_str.as_str()).unwrap();

        let _h = mine_and_sync(&btc, 101, &addr, vec![&cln_back, &cln_peer]).await;

        let info = btc.get_blockchain_info().expect("BTC blockchain info");
        let info_back = cln_back.getinfo().expect("CLN blockchain info");
        let info_peer = cln_peer.getinfo().expect("CLN blockchain info");
        println!(
            "Network synced: Core tip {}, CLN Back height {}, CLN Peer height {} ",
            info.blocks, info_back.blockheight, info_peer.blockheight
        );

        let mut uri = format!("{}", info_peer.id);
        for addr in info_peer.binding {
            match addr {
                Ipv4 { address, port } => {
                    uri = format!("{}@{}:{}", uri, address, port);
                    break; // there are 2 hosts in binding. the first just worked. c'est la vie
                }
                _ => println!("skipping {:?}", addr),
            }
        }

        println!("Opening channel with {}", uri);
        let connect = cln_back
            .connect(uri.as_str(), None)
            .expect("Connecting to peer node");

        println!("Connected {}, features {}", connect.id, connect.features);

        let funding = cln_back
            .fundchannel(info_peer.id.as_str(), AmountOrAll::All, Some(5))
            .expect("Channel funding");
        println!("{} - {}", funding.txid, funding.channel_id);

        let _h = mine_and_sync(&btc, 101, &addr, vec![&cln_back, &cln_peer]).await;

        let info_back = cln_back.getinfo().expect("CLN blockchain info");
        let info_peer = cln_peer.getinfo().expect("CLN blockchain info");
        println!(
            "CLN Back peers {}, CLN Peer peers {} ",
            info_back.num_peers, info_peer.num_peers
        );
        println!(
            "CLN Back channels active {}, CLN Peer channels active {} ",
            info_back.num_active_channels, info_peer.num_active_channels
        );

        let a = cln_peer.newaddr(None).expect("address wasn't provided");
        let addr_str = a.bech32.unwrap();
        let withdraw = cln_back
            .withdraw(addr_str.as_str(), AmountOrAll::Amount(10000), None, None)
            .expect("Withdraw");
        println!("Withdraw to {} Passed: {}", addr_str, withdraw.txid);

        let fees = cln_back.feerates("perkw").expect("Network fees");
        println!("Fees Passed: {:?}", fees);

        let peers = cln_back.listpeers(None, None).expect("Getting peers");
        println!("List Peers Passed: {} peers", peers.peers.len());

        // let node update graph
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

        let chans = cln_back
            .listchannels(None, None, None)
            .expect("Node channels");
        println!("List Channels Passed: {} channels", chans.channels.len());

        let funds = cln_back.listfunds().expect("Node funds");
        println!("List Funds Passed: {:?}", funds.channels);
        println!("List Funds Passed: {:?}", funds.outputs);

        for n in 1..2 {
            let preimage = "0000000000000000000000000000000000000000000000000000000000000000";
            let hash = "66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925";

            let invoice = cln_peer
                .invoice(
                    Some(10000000),
                    format!("payment - {}", n).as_str(),
                    "test",
                    Some(preimage),
                    None,
                    None,
                )
                .expect("Failed to get invoice");
            println!("{:?}", invoice);

            let invoices = cln_peer
                .listinvoices(None, None, None, None)
                .expect("Failed to get invoices");
            println!("{:?}", invoices);

            let _i = btc.get_blockchain_info().expect("BTC blockchain info");
            //let _h = mine_and_sync(&btc, 1, &addr, vec![&cln_back, &cln_peer]).await;

            let decoded = cln_back
                .decodepay(invoice.bolt11.as_str(), None)
                .expect("Decoded invoice");
            println!("Decode invoice passed {}", n);

            let route = cln_back
                .getroute(
                    info_peer.id.as_str(),
                    decoded.amount_msat.unwrap().0,
                    0.0,
                    None,
                    None,
                    None,
                    None,
                )
                .expect("Route to the peer");
            println!("Get route passed, {} hops", route.route.len());

            //let pay = cln_back.pay(invoice.bolt11.as_str(), Default::default()).expect("Payment doesn't go");
            //wait_for_htlc(&cln_back).await;
            //println!("{:?}", pay);
            assert_eq!(decoded.payment_hash.as_str(), hash);
            let sendpay = cln_back
                .sendpay(
                    route.route,
                    decoded.payment_hash.as_str(),
                    None,
                    Some(decoded.amount_msat.unwrap().0),
                )
                .expect("Payment doesn't go");
            println!("Sendpay {} passed {:?}", n, sendpay);
            let wait = cln_back
                .waitsendpay(decoded.payment_hash.as_str(), 120)
                .expect("Waiting for payment");
            println!("Wait sendpay passed {:?}", wait);

            let _i = btc.get_blockchain_info().expect("BTC blockchain info");
            //let _h = mine_and_sync(&btc, 1, &addr, vec![&cln_back, &cln_peer]).await;
            tokio::time::sleep(std::time::Duration::from_millis(10000)).await;
        }

        let listpays = cln_back.listsendpays(None, None).expect("Sent payments");
        for p in listpays.payments {
            println!("Payment {:?}", p);
        }

        /*
        let logs = cln_back.getlog(None).expect("Expected logs");
        println!("{}", logs.log.len());
        for l in logs.log {
            println!("{:?}", l);
        }
        */

        cln_back.stop().expect("CLN client didn't stop");
        cln_peer.stop().expect("CLN client didn't stop");

        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    })
    .await;
}
