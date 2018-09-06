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

/// 'feerates' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRates {
    pub style: String,
}

/// 'listnodes' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListNodes {
    pub id: Option<String>,
}

/// 'listchannels' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListChannels {
    pub short_channel_id: Option<String>,
}

/// 'help' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Help {
    pub command: Option<String>,
}

/// 'getlog' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetLog {
    pub level: Option<String>,
}

/// 'listconfigs' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListConfigs {
    pub config: Option<String>,
}

/// 'listpeers' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListPeers {
    pub id: Option<String>,
    pub level: Option<String>,
}

/// 'listinvoices' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListInvoices {
    pub label: Option<String>,
}

/// 'invoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invoice {
    pub msatoshi: i64,
    pub label: String,
    pub description: String,
    pub expiry: Option<i64>,
}

/// 'delinvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DelInvoice {
    pub label: String,
    pub status: String,
}

/// 'delexpiredinvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DelExpiredInvoice {
    pub maxexpirytime: Option<i64>,
}

/// 'autocleaninvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoCleanInvoice {
    pub cycle_seconds: Option<i64>,
    pub expired_by: Option<i64>,
}

/// 'waitanyinvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaitAnyInvoice {
    pub lastpay_index: Option<i64>,
}

/// 'waitinvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaitInvoice {
    pub label: String,
}

/// 'pay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pay {
    pub bolt11: String,
    pub msatoshi: Option<i64>,
    pub description: Option<String>,
    pub riskfactor: Option<f64>,
    pub maxfeepercent: Option<f64>,
    pub exemptfee: Option<i64>,
    pub retry_for: Option<i64>,
    pub maxdelay: Option<i64>,
}

/// 'sendpay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendPay {
    pub route: Vec<common::RouteItem>,
    pub payment_hash: String,
    pub description: Option<String>,
    pub msatoshi: Option<i64>,
}

/// 'waitsendpay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaitSendPay {
    pub payment_hash: String,
    pub timeout: i64,
}

/// 'listpayments' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListPayments {
    pub bolt11: Option<String>,
    pub payment_hash: Option<String>,
}

/// 'decodepay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecodePay {
    pub bolt11: String,
    pub description: Option<String>,
}

/// 'getroute' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetRoute {
    pub id: String,
    pub msatoshi: i64,
    pub riskfactor: f64,
    pub cltv: Option<i64>,
    pub fromid: Option<String>,
    pub fuzzpercent: Option<f64>,
    pub seed: Option<String>,
}

/// 'connect' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Connect {
    pub id: String,
    pub host: Option<String>,
}

/// 'disconnect' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Disconnect {
    pub id: String,
}

/// enum type that can either hold an integer amount, or All
#[derive(Debug, Clone)]
pub enum AmountOrAll {
    All,
    Amount(i64),
}

impl Serialize for AmountOrAll {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            AmountOrAll::Amount(a) => serializer.serialize_i64(*a),
            AmountOrAll::All => serializer.serialize_str("all"),
        }
    }
}

/// 'fundchannel' command
#[derive(Debug, Clone, Serialize)]
pub struct FundChannel {
    pub id: String,
    pub satoshi: AmountOrAll,
    pub feerate: Option<i64>,
}

/// 'close' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Close {
    pub id: String,
    pub force: Option<bool>,
    pub timeout: Option<i64>,
}

/// 'ping' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ping {
    pub peerid: String,
    pub len: Option<i64>,
    pub pongbytes: Option<i64>,
}

/// 'listfunds' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListFunds {}

/// 'withdraw' command
#[derive(Debug, Clone, Serialize)]
pub struct Withdraw {
    pub destination: String,
    pub satoshi: AmountOrAll,
    pub feerate: Option<i64>,
}

/// 'newaddr' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewAddr {
    pub addresstype: Option<String>,
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
            id: "12345".to_string(),
            satoshi: AmountOrAll::Amount(123456),
            feerate: None,
        }).unwrap();
        assert_eq!(
            result.to_string(),
            r#"{"id": "12345", "satoshi": 123456, "feerate": null}"#
        );

        let result = Json::from_serialize(FundChannel {
            id: "12345".to_string(),
            satoshi: AmountOrAll::All,
            feerate: Some(123),
        }).unwrap();
        assert_eq!(
            result.to_string(),
            r#"{"id": "12345", "satoshi": "all", "feerate": 123}"#
        );
    }
}
