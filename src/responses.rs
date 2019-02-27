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
#![allow(missing_docs)]
//! Structures representing responses to API calls
use std::collections::HashMap;

use common;

/// structure for network addresses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkAddress {
    #[serde(rename = "type")]
    pub type_: String,
    pub address: String,
    pub port: u16,
}

/// 'getinfo' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetInfo {
    pub id: String,
    pub alias: String,
    pub color: String,
    pub address: Vec<NetworkAddress>,
    pub binding: Vec<NetworkAddress>,
    pub version: String,
    pub blockheight: u64,
    pub network: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRatesInner {
    pub urgent: u64,
    pub normal: u64,
    pub slow: u64,
    pub min_acceptable: u64,
    pub max_acceptable: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRatesOnchain {
    pub opening_channel_satoshis: u64,
    pub mutual_close_satoshis: u64,
    pub unilateral_close_satoshis: u64,
}

/// 'feerates' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRates {
    pub perkb: Option<FeeRatesInner>,
    pub perkw: Option<FeeRatesInner>,
    pub onchain_fee_estimates: Option<FeeRatesOnchain>,
}

/// Sub-structure for 'listnodes' items
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListNodesItem {
    pub nodeid: String,
    pub alias: Option<String>,
    pub color: Option<String>,
    pub last_timestamp: Option<u64>,
    pub global_features: Option<String>,
    pub addresses: Option<Vec<NetworkAddress>>,
}

/// 'listnodes' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListNodes {
    pub nodes: Vec<ListNodesItem>,
}

/// Sub-structure for 'listchannels' item
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListChannelsItem {
    pub source: String,
    pub destination: String,
    pub short_channel_id: String,
    pub public: bool,
    pub satoshis: u64,
    pub flags: u64,
    pub active: bool,
    pub last_update: u64,
    pub base_fee_millisatoshi: u64,
    pub fee_per_millionth: u64,
    pub delay: u64,
}

/// 'listchannels' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListChannels {
    pub channels: Vec<ListChannelsItem>,
}

/// Sub-structure for 'help' item
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HelpItem {
    pub command: String,
    pub description: String,
}

/// 'help' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Help {
    pub help: Option<Vec<HelpItem>>,
    pub verbose: Option<String>,
}

/// Sub-structure for 'getlog' item
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogEntry {
    #[serde(rename = "type")]
    pub type_: String,
    pub num_skipped: Option<u64>,
    pub time: Option<String>,
    pub source: Option<String>,
    pub log: Option<String>,
}

/// 'getlog' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetLog {
    pub created_at: String,
    pub bytes_used: u64,
    pub bytes_max: u64,
    pub log: Vec<LogEntry>,
}

/// 'listconfigs' command
pub type ListConfigs = HashMap<String, serde_json::Value>;

/// Sub-structure for channel in 'listpeers'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    pub state: String,
    pub owner: Option<String>,
    pub short_channel_id: Option<String>,
    pub channel_id: String,
    pub funding_txid: String,
    pub msatoshi_to_us: u64,
    pub msatoshi_to_us_min: u64,
    pub msatoshi_to_us_max: u64,
    pub msatoshi_total: u64,
    pub dust_limit_satoshis: u64,
    pub max_htlc_value_in_flight_msat: u64, // this exceeds what fits into u64
    pub their_channel_reserve_satoshis: u64,
    pub our_channel_reserve_satoshis: u64,
    pub spendable_msatoshi: u64,
    pub htlc_minimum_msat: u64,
    pub their_to_self_delay: u64,
    pub our_to_self_delay: u64,
    pub max_accepted_htlcs: u64,
    pub status: Vec<String>,
    pub in_payments_offered: u64,
    pub in_msatoshi_offered: u64,
    pub in_payments_fulfilled: u64,
    pub in_msatoshi_fulfilled: u64,
    pub out_payments_offered: u64,
    pub out_msatoshi_offered: u64,
    pub out_payments_fulfilled: u64,
    pub out_msatoshi_fulfilled: u64,
}

/// Sub-structure for log entry in 'listpeers'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Log {
    #[serde(rename = "type")]
    pub type_: String,
    pub time: String,
    pub source: String,
    pub log: String,
}

/// Sub-structure for peer in 'listpeers'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Peer {
    pub id: String,
    pub connected: bool,
    pub netaddr: Option<Vec<String>>,
    pub local_features: Option<String>,
    pub global_features: Option<String>,
    pub channels: Vec<Channel>,
    pub log: Option<Vec<Log>>,
}

/// 'listpeers' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListPeers {
    pub peers: Vec<Peer>,
}

/// Sub-structure for invoices in 'listinvoices'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListInvoice {
    pub label: String,
    pub bolt11: String,
    pub payment_hash: String,
    pub msatoshi: u64,
    pub status: String,
    pub expires_at: u64,
    pub pay_index: Option<u64>,
    pub paid_at: Option<u64>,
}

/// 'listinvoices' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListInvoices {
    pub invoices: Vec<ListInvoice>,
}

/// 'invoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Invoice {
    pub payment_hash: String,
    pub expires_at: u64,
    pub bolt11: String,
}

/// 'delinvoice' command
pub type DelInvoice = ListInvoice;

/// 'delexpiredinvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DelExpiredInvoice {}

/// 'autocleaninvoice' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoCleanInvoice {}

/// 'waitanyinvoice' command
pub type WaitAnyInvoice = ListInvoice;

/// 'waitinvoice' command
pub type WaitInvoice = ListInvoice;

/// Sub-structure for failure in 'pay'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FailureItem {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub erring_index: u64,
    pub failcode: u64,
    pub erring_node: String,
    pub erring_channel: String,
    pub channel_update: Option<String>,
    pub route: Vec<common::RouteItem>,
}

/// 'pay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pay {
    pub id: u64,
    pub payment_hash: String,
    pub destination: String,
    pub msatoshi: u64,
    pub msatoshi_sent: u64,
    pub created_at: u64,
    pub status: String,
    pub payment_preimage: String,
    pub description: String,
    pub getroute_tries: u64,
    pub sendpay_tries: u64,
    pub route: Vec<common::RouteItem>,
    pub failures: Vec<FailureItem>,
}

/// 'sendpay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendPay {
    pub message: Option<String>,

    pub id: u64,
    pub payment_hash: String,
    pub destination: String,
    pub msatoshi: u64,
    pub msatoshi_sent: u64,
    pub created_at: u64,
    pub status: String,
    pub payment_preimage: Option<String>,
    pub description: Option<String>,
}

/// Sub-structure for payments in 'listpayments' and 'waitsendpay'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListPaymentsItem {
    pub id: u64,
    pub payment_hash: String,
    pub destination: String,
    pub msatoshi: u64,
    pub msatoshi_sent: u64,
    pub created_at: u64,
    pub status: String,
    pub payment_preimage: Option<String>,
    pub description: Option<String>,
}

/// 'waitsendpay' command
pub type WaitSendPay = ListPaymentsItem;

/// 'listpayments' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListPayments {
    pub payments: Vec<ListPaymentsItem>,
}

/// 'decodepay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecodePay {
    pub currency: String,
    pub created_at: u64,
    pub expiry: u64,
    pub payee: String,
    pub msatoshi: u64,
    pub description: String,
    pub min_final_cltv_expiry: u64,
    pub payment_hash: String,
    pub signature: String,
}

/// 'getroute' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetRoute {
    pub route: Vec<common::RouteItem>,
}

/// 'connect' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Connect {
    pub id: String,
}

/// 'disconnect' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Disconnect {}

/// 'fundchannel' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FundChannel {
    pub tx: String,
    pub txid: String,
    pub channel_id: String,
}

/// 'close' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Close {
    pub tx: String,
    pub txid: String,
    #[serde(rename = "type")]
    pub type_: String,
}

/// 'ping' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ping {
    pub totlen: u64,
}

/// Sub-structure for 'listfunds' output
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListFundsOutput {
    pub txid: String,
    pub output: u64,
    pub value: u64,
    pub address: String,
    pub status: String,
}

/// Sub-structure for 'listfunds' channel
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListFundsChannel {
    pub peer_id: String,
    pub short_channel_id: Option<String>,
    pub channel_sat: u64,
    pub channel_total_sat: u64,
    pub funding_txid: String,
}

/// 'listfunds' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListFunds {
    pub outputs: Vec<ListFundsOutput>,
    pub channels: Vec<ListFundsChannel>,
}

/// 'withdraw' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Withdraw {
    pub tx: String,
    pub txid: String,
}

/// 'newaddr' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewAddr {
    pub address: String,
}

/// 'stop' command
pub type Stop = String;
