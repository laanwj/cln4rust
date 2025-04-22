use std::time::Duration;

use rstest::*;
use tokio::time::sleep;
use uuid::Uuid;

use clightning_testing::cln;
use clightning_testing::prelude::clightningrpc;
use clightningrpc::responses;
use clightningrpc::types::MSat;

use crate::init;

#[tokio_test_shutdown_timeout::test(60)]
async fn getinfo_test_one_async() {
    init();
    let cln = cln::Node::with_params("--developer --allow-deprecated-apis=true")
        .await
        .unwrap();
    let lightningd = cln.rpc();
    let get_info: responses::GetInfo = lightningd
        .call("getinfo", serde_json::json!({}))
        .await
        .unwrap();
    assert_eq!("regtest", get_info.network);
}

#[tokio_test_shutdown_timeout::test(60)]
async fn listfunds_test_one_async() {
    init();
    let cln = cln::Node::with_params("--developer --allow-deprecated-apis=true")
        .await
        .unwrap();
    let lightningd = cln.rpc();
    let list_funds: responses::ListFunds = lightningd
        .call("listfunds", serde_json::json!({}))
        .await
        .unwrap();
    assert_eq!(0, list_funds.channels.len());
    assert_eq!(0, list_funds.outputs.len());
}

#[tokio_test_shutdown_timeout::test(60)]
async fn listinvoice_by_payment_hash_test_one_async() {
    init();
    let cln = cln::Node::with_params("--developer --allow-deprecated-apis=true")
        .await
        .unwrap();
    let lightningd = cln.rpc();
    let listinvoice: Result<responses::ListInvoices, _> =
        lightningd.call("listinvoices", serde_json::json!({})).await;
    assert!(listinvoice.is_ok());
}

#[tokio_test_shutdown_timeout::test(60)]
async fn generate_amountless_invoice_test_one_async() {
    init();
    let cln = cln::Node::with_params("--developer --allow-deprecated-apis=true")
        .await
        .unwrap();
    let lightningd = cln.rpc();
    let label = format!("{}", Uuid::new_v4());
    let invoice: responses::Invoice = lightningd
        .call(
            "invoice",
            serde_json::json!({
                "amount_msat": "any",
                "label": label,
                "description": "generate an any invoice",
            }),
        )
        .await
        .unwrap();
    let decode: responses::DecodePay = lightningd
        .call("decodepay", serde_json::json!({ "bolt11": invoice.bolt11 }))
        .await
        .unwrap();
    assert_eq!(decode.amount_msat, None);
}

#[tokio_test_shutdown_timeout::test(60)]
async fn generate_invoice_with_amount_test_one_async() {
    init();
    let cln = cln::Node::with_params("--developer --allow-deprecated-apis=true")
        .await
        .unwrap();
    let lightningd = cln.rpc();
    let label = format!("{}", Uuid::new_v4());
    let invoice: responses::Invoice = lightningd
        .call(
            "invoice",
            serde_json::json!({
                "amount_msat": 1,
                "label": label,
                "description": "generate an any invoice",
            }),
        )
        .await
        .unwrap();
    let decode: responses::DecodePay = lightningd
        .call("decodepay", serde_json::json!({ "bolt11": invoice.bolt11 }))
        .await
        .unwrap();
    assert_eq!(decode.amount_msat, Some(MSat(1)));
}

#[tokio_test_shutdown_timeout::test(60)]
async fn generate_invoice_with_description_hash_async() {
    init();
    let cln = cln::Node::with_params("--developer --allow-deprecated-apis=true")
        .await
        .unwrap();
    let lightningd = cln.rpc();
    let label = format!("{}", Uuid::new_v4());
    let invoice: responses::Invoice = lightningd
        .call(
            "invoice",
            serde_json::json!({
                "amount_msat": 1,
                "label": label,
                "description": "description for hash",
                "deschashonly": true,
            }),
        )
        .await
        .unwrap();
    // FIXME: use the decode command
    let decode: responses::DecodePay = lightningd
        .call("decodepay", serde_json::json!({ "bolt11": invoice.bolt11 }))
        .await
        .unwrap();
    assert_eq!(decode.amount_msat, Some(MSat(1)));
    assert_eq!(
        decode.description_hash,
        Some("62af1b6b91d49301648cb3e6e5c88ced5d72a8c1db3e6711dcf89add72436479".to_string())
    );
}
