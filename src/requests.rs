// Rust JSON-RPC Library
// Written by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//     Wladimir J. van der Laan <laanwj@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//
//! Structures representing requests to API calls
#![allow(missing_docs)]
use serde::{Serialize, Serializer};

use common;

/// 'getinfo' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetInfo {}

/// 'aeerates' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRates<'a> {
    pub style: &'a str,
}

/// 'listnodes' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListNodes<'a> {
    pub id: Option<&'a str>,
}

/// 'listchannels' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListChannels<'a> {
    pub short_channel_id: Option<&'a str>,
}

/// 'help' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Help<'a> {
    pub command: Option<&'a str>,
}

/// 'getlog' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetLog<'a> {
    pub level: Option<&'a str>,
}

/// 'listconfigs' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListConfigs<'a> {
    pub config: Option<&'a str>,
}

/// 'listpeers' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListPeers<'a> {
    pub id: Option<&'a str>,
    pub level: Option<&'a str>,
}

/// 'listinvoices' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListInvoices<'a> {
    pub label: Option<&'a str>,
}

/// 'invoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invoice<'a> {
    pub msatoshi: u64,
    pub label: &'a str,
    pub description: &'a str,
    pub expiry: Option<u64>,
}

/// 'delinvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DelInvoice<'a> {
    pub label: &'a str,
    pub status: &'a str,
}

/// 'delexpiredinvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DelExpiredInvoice {
    pub maxexpirytime: Option<u64>,
}

/// 'autocleaninvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoCleanInvoice {
    pub cycle_seconds: Option<u64>,
    pub expired_by: Option<u64>,
}

/// 'waitanyinvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaitAnyInvoice {
    pub lastpay_index: Option<u64>,
}

/// 'waitinvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaitInvoice<'a> {
    pub label: &'a str,
}

/// 'pay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pay<'a> {
    pub bolt11: &'a str,
    pub msatoshi: Option<u64>,
    pub description: Option<&'a str>,
    pub riskfactor: Option<f64>,
    pub maxfeepercent: Option<f64>,
    pub exemptfee: Option<u64>,
    pub retry_for: Option<u64>,
    pub maxdelay: Option<u64>,
}

/// 'sendpay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendPay<'a> {
    pub route: Vec<common::RouteItem>,
    pub payment_hash: &'a str,
    pub description: Option<&'a str>,
    pub msatoshi: Option<u64>,
}

/// 'waitsendpay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaitSendPay<'a> {
    pub payment_hash: &'a str,
    pub timeout: u64,
}

/// 'listpayments' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListPayments<'a> {
    pub bolt11: Option<&'a str>,
    pub payment_hash: Option<&'a str>,
}

/// 'decodepay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecodePay<'a> {
    pub bolt11: &'a str,
    pub description: Option<&'a str>,
}

/// 'getroute' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetRoute<'a> {
    pub id: &'a str,
    pub msatoshi: u64,
    pub riskfactor: f64,
    pub cltv: Option<u64>,
    pub fromid: Option<&'a str>,
    pub fuzzpercent: Option<f64>,
    pub seed: Option<&'a str>,
}

/// 'connect' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Connect<'a> {
    pub id: &'a str,
    pub host: Option<&'a str>,
}

/// 'disconnect' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Disconnect<'a> {
    pub id: &'a str,
}

/// enum type that can either hold an integer amount, or All
#[derive(Debug, Clone)]
pub enum AmountOrAll {
    All,
    Amount(u64),
}

impl Serialize for AmountOrAll {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            AmountOrAll::Amount(a) => serializer.serialize_u64(*a),
            AmountOrAll::All => serializer.serialize_str("all"),
        }
    }
}

/// 'aundchannel' command
#[derive(Debug, Clone, Serialize)]
pub struct FundChannel<'a> {
    pub id: &'a str,
    pub satoshi: AmountOrAll,
    pub feerate: Option<u64>,
}

/// 'close' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Close<'a> {
    pub id: &'a str,
    pub force: Option<bool>,
    pub timeout: Option<u64>,
}

/// 'ping' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ping<'a> {
    pub id: &'a str,
    pub len: Option<u64>,
    pub pongbytes: Option<u64>,
}

/// 'listfunds' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListFunds {}

/// 'withdraw' command
#[derive(Debug, Clone, Serialize)]
pub struct Withdraw<'a> {
    pub destination: &'a str,
    pub satoshi: AmountOrAll,
    pub feerate: Option<u64>,
}

/// 'newaddr' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewAddr<'a> {
    pub addresstype: Option<&'a str>,
}

/// 'stop' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stop {}

#[cfg(test)]
mod tests {
    use super::*;
    use strason::Json;

    #[test]
    fn fundchannel() {
        // Tests AmountOrAll as well as basic JSON serialization
        let result = Json::from_serialize(FundChannel {
            id: "12345",
            satoshi: AmountOrAll::Amount(123456),
            feerate: None,
        }).unwrap();
        assert_eq!(
            result.to_string(),
            r#"{"id": "12345", "satoshi": 123456, "feerate": null}"#
        );

        let result = Json::from_serialize(FundChannel {
            id: "12345",
            satoshi: AmountOrAll::All,
            feerate: Some(123),
        }).unwrap();
        assert_eq!(
            result.to_string(),
            r#"{"id": "12345", "satoshi": "all", "feerate": 123}"#
        );
    }
}
