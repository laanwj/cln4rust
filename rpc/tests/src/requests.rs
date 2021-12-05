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

use crate::common;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
}

/// 'listchannels' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListChannels<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_channel_id: Option<&'a str>,
}

/// 'help' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Help<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<&'a str>,
}

/// 'getlog' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetLog<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<&'a str>,
}

/// 'listconfigs' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListConfigs<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<&'a str>,
}

/// 'listpeers' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListPeers<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<&'a str>,
}

/// 'listinvoices' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListInvoices<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<&'a str>,
}

/// 'invoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invoice<'a> {
    pub msatoshi: u64,
    pub label: &'a str,
    pub description: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxexpirytime: Option<u64>,
}

/// 'autocleaninvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoCleanInvoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_by: Option<u64>,
}

/// 'waitanyinvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaitAnyInvoice {
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msatoshi: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub riskfactor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxfeepercent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemptfee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_for: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxdelay: Option<u64>,
}

/// 'sendpay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendPay<'a> {
    pub route: Vec<common::RouteItem>,
    pub payment_hash: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msatoshi: Option<u64>,
}

/// 'waitsendpay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WaitSendPay<'a> {
    pub payment_hash: &'a str,
    pub timeout: u64,
}

/// 'listsendpays' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListSendPays<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bolt11: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_hash: Option<&'a str>,
}

/// 'decodepay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecodePay<'a> {
    pub bolt11: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
}

/// 'getroute' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetRoute<'a> {
    pub id: &'a str,
    pub msatoshi: u64,
    pub riskfactor: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cltv: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromid: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzzpercent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<&'a str>,
}

/// 'connect' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Connect<'a> {
    pub id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
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
        match *self {
            AmountOrAll::Amount(a) => serializer.serialize_u64(a),
            AmountOrAll::All => serializer.serialize_str("all"),
        }
    }
}

/// 'aundchannel' command
#[derive(Debug, Clone, Serialize)]
pub struct FundChannel<'a> {
    pub id: &'a str,
    pub amount: AmountOrAll,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate: Option<u64>,
}

/// 'close' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Close<'a> {
    pub id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}

/// 'ping' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ping<'a> {
    pub id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pongbytes: Option<u64>,
}

/// 'listfunds' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListFunds {}

/// 'withdraw' command
#[derive(Debug, Clone, Serialize)]
pub struct Withdraw<'a> {
    pub destination: &'a str,
    pub amount: AmountOrAll,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feerate: Option<u64>,
}

/// 'newaddr' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewAddr<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresstype: Option<&'a str>,
}

/// 'stop' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stop {}
