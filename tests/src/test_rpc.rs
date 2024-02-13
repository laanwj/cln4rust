//! RPC integration testing
use std::{thread, time};

use rstest::*;
use uuid::Uuid;

use clightning_testing::cln;
use clightning_testing::prelude::clightningrpc;
use clightningrpc::types::MSat;

use crate::fixtures::*;

// FIXME(vincenzopalazzo) Move this in a utils test
#[allow(dead_code)]
fn wait_for(millisecond: u64) {
    let wait_time = time::Duration::from_millis(millisecond);
    thread::sleep(wait_time);
}

#[rstest]
fn getinfo_test_one(lightningd: cln::Node) {
    let lightningd = lightningd.rpc();
    let get_info = lightningd.getinfo().unwrap();
    assert_eq!("regtest", get_info.network);
}

#[rstest]
fn listfunds_test_one(lightningd: cln::Node) {
    let lightningd = lightningd.rpc();
    let list_funds = lightningd.listfunds().unwrap();
    assert_eq!(0, list_funds.channels.len());
    assert_eq!(0, list_funds.outputs.len());
}

#[rstest]
fn listinvoice_by_payment_hash_test_one(lightningd: cln::Node) {
    let lightningd = lightningd.rpc();
    let listinvoice = lightningd.listinvoices(None, None, None, None);
    assert!(listinvoice.is_ok());
}

#[rstest]
fn generate_amountless_invoice_test_one(lightningd: cln::Node) {
    let label = format!("{}", Uuid::new_v4());
    let lightningd = lightningd.rpc();
    let invoice = lightningd
        .invoice(
            None,
            label.as_str(),
            "generate an any invoice",
            None,
            None,
            None,
        )
        .unwrap();
    let decode = lightningd.decodepay(&invoice.bolt11, None).unwrap();
    assert_eq!(decode.amount_msat, None);
}

#[rstest]
fn generate_invoice_with_amount_test_one(lightningd: cln::Node) {
    let label = format!("{}", Uuid::new_v4());
    let lightningd = lightningd.rpc();
    let invoice = lightningd
        .invoice(
            Some(1),
            label.as_str(),
            "generate an any invoice",
            None,
            None,
            None,
        )
        .unwrap();
    let decode = lightningd.decodepay(&invoice.bolt11, None).unwrap();
    assert_eq!(decode.amount_msat, Some(MSat(1)));
}

#[rstest]
fn generate_invoice_with_description_hash(lightningd: cln::Node) {
    let lightningd = lightningd.rpc();
    let label = format!("{}", Uuid::new_v4());
    let invoice = lightningd
        .invoice(
            Some(1),
            label.as_str(),
            "description for hash",
            None,
            None,
            Some(true),
        )
        .unwrap();
    println!("{:?}", invoice);
    let decode = lightningd.decodepay(&invoice.bolt11, None).unwrap();
    assert_eq!(decode.amount_msat, Some(MSat(1)));
    assert_eq!(
        decode.description_hash,
        Some("62af1b6b91d49301648cb3e6e5c88ced5d72a8c1db3e6711dcf89add72436479".to_string())
    );
}
