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
use serde_json;
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};

use crate::common;
use crate::common::MSat;

/// structure for network addresses
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NetworkAddress {
    Ipv4 {
        address: Ipv4Addr,
        port: u16,
    },
    Ipv6 {
        address: Ipv6Addr,
        port: u16,
    },
    Torv2 {
        address: String,
        port: u16,
    },
    Torv3 {
        address: String,
        port: u16,
    },
}

/// 'getinfo' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetInfo {
    pub id: String,
    pub alias: String,
    pub color: String,
    pub num_peers: u64,
    pub num_pending_channels: u64,
    pub num_active_channels: u64,
    pub num_inactive_channels: u64,
    pub address: Vec<NetworkAddress>,
    pub binding: Vec<NetworkAddress>,
    pub version: String,
    pub blockheight: u64,
    pub fees_collected_msat: MSat,
    pub network: String,
    #[serde(rename = "lightning-dir")]
    pub ligthning_dir: String,
    pub warning_bitcoind_sync: Option<String>,
    pub warning_lightningd_sync: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRatesInner {
    pub urgent: Option<u64>,
    pub normal: Option<u64>,
    pub slow: Option<u64>,
    pub opening: u64,
    pub mutual_close: u64,
    pub unilateral_close: u64,
    pub delayed_to_us: u64,
    pub htlc_resolution: u64,
    pub penalty: u64,
    pub min_acceptable: u64,
    pub max_acceptable: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRatesOnchain {
    pub opening_channel_satoshis: u64,
    pub mutual_close_satoshis: u64,
    pub unilateral_close_satoshis: u64,
    pub htlc_timeout_satoshis: u64,
    pub htlc_success_satoshis: u64,
}

/// 'feerates' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRates {
    pub perkb: Option<FeeRatesInner>,
    pub perkw: Option<FeeRatesInner>,
    pub warning: Option<String>,
    pub onchain_fee_estimates: Option<FeeRatesOnchain>,
}

/// Sub-structure for 'listnodes' items
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListNodesItem {
    pub nodeid: String,
    pub alias: Option<String>,
    pub color: Option<String>,
    pub last_timestamp: Option<u64>,
    pub features: Option<String>,
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
    pub amount_msat: MSat,
    pub message_flags: u64,
    pub channel_flags: u64,
    pub active: bool,
    pub last_update: u64,
    pub base_fee_millisatoshi: u64,
    pub fee_per_millionth: u64,
    pub delay: u64,
    pub htlc_minimum_msat: MSat,
    pub htlc_maximum_msat: MSat,
    pub features: String,
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
    pub category: String,
    pub description: String,
    pub verbose: String,
}

/// 'help' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Help {
    pub help: Option<Vec<HelpItem>>,
}

/// Sub-structure for 'getlog' and 'listpeers' item
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogEntry {
    #[serde(rename = "type")]
    pub type_: String,
    pub num_skipped: Option<u64>,
    pub time: Option<String>,
    pub node_id: Option<String>,
    pub source: Option<String>,
    pub log: Option<String>,
    pub data: Option<String>,
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

/// Sub-structure for htlcs in 'listpeers'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Htlc {
    pub direction: String,
    pub id: u64,
    pub amount_msat: MSat,
    pub expiry: u64,
    pub payment_hash: String,
    pub state: String,
    pub local_trimmed: Option<bool>,
}

/// Sub-structure for channel in 'listpeers'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    pub state: String,
    pub scratch_txid: Option<String>,
    pub owner: Option<String>,
    pub short_channel_id: Option<String>,
    pub direction: Option<u64>,
    pub channel_id: String,
    pub funding_txid: String,
    pub close_to_addr: Option<String>,
    pub close_to: Option<String>,
    pub private: bool,
    pub funding_msat: HashMap<String, MSat>,
    pub to_us_msat: MSat,
    pub min_to_us_msat: MSat,
    pub max_to_us_msat: MSat,
    pub total_msat: MSat,
    pub dust_limit_msat: MSat,
    pub max_total_htlc_in_msat: MSat, // this exceeds what fits into u64
    pub their_reserve_msat: MSat,
    pub our_reserve_msat: MSat,
    pub spendable_msat: MSat,
    pub receivable_msat: MSat,
    pub minimum_htlc_in_msat: MSat,
    pub their_to_self_delay: u64,
    pub our_to_self_delay: u64,
    pub max_accepted_htlcs: u64,
    pub status: Vec<String>,
    pub in_payments_offered: u64,
    pub in_offered_msat: MSat,
    pub in_payments_fulfilled: u64,
    pub in_fulfilled_msat: MSat,
    pub out_payments_offered: u64,
    pub out_offered_msat: MSat,
    pub out_payments_fulfilled: u64,
    pub out_fulfilled_msat: MSat,
    pub htlcs: Vec<Htlc>,
}

/// Sub-structure for peer in 'listpeers'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Peer {
    pub id: String,
    pub connected: bool,
    pub netaddr: Option<Vec<String>>,
    pub features: Option<String>,
    pub channels: Vec<Channel>,
    pub log: Option<Vec<LogEntry>>,
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
    pub amount_msat: Option<MSat>,
    pub status: String,
    pub pay_index: Option<u64>,
    pub amount_received_msat: Option<MSat>,
    pub paid_at: Option<u64>,
    pub payment_preimage: Option<String>,
    pub description: Option<String>,
    pub expires_at: u64,
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
    pub payment_hash: String,
    pub destination: String,
    pub msatoshi: u64,
    pub msatoshi_sent: u64,
    pub created_at: f64,
    pub status: String,
    pub payment_preimage: String,
    pub parts: u64,
}

/// 'sendpay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendPay {
    pub message: Option<String>,

    pub id: u64,
    pub payment_hash: String,
    pub partid: Option<u64>,
    pub destination: Option<String>,
    pub amount_msat: Option<MSat>,
    pub amount_sent_msat: MSat,
    pub created_at: u64,
    pub status: String,
    pub payment_preimage: Option<String>,
    pub description: Option<String>,
    pub bolt11: Option<String>,
    pub erroronion: Option<String>,

    pub onionreply: Option<String>,
    pub erring_index: Option<u64>,
    pub failcode: Option<u64>,
    pub failcodename: Option<String>,
    pub erring_node: Option<String>,
    pub erring_channel: Option<String>,
    pub erring_direction: Option<u64>,
    pub raw_message: Option<String>,
}

/// Sub-structure for payments in 'listsendpays' and 'waitsendpay'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListSendPaysItem {
    pub id: u64,
    pub payment_hash: String,
    pub partid: Option<u64>,
    pub destination: Option<String>,
    pub amount_msat: Option<MSat>,
    pub amount_sent_msat: MSat,
    pub created_at: u64,
    pub status: String,
    pub payment_preimage: Option<String>,
    pub description: Option<String>,
    pub bolt11: Option<String>,
    pub erroronion: Option<String>,
}

/// 'waitsendpay' command
pub type WaitSendPay = ListSendPaysItem;

/// 'listsendpays' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListSendPays {
    pub payments: Vec<ListSendPaysItem>,
}

/// Sub-structure for fallbacks in 'decodepay'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fallback {
    #[serde(rename = "type")]
    pub type_: String,
    pub addr: String,
    pub hex: String,
}

/// Sub-structure for routes in 'decodepay'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecodePayRoute {
    pub pubkey: String,
    pub short_channel_id: String,
    pub fee_base_msat: u64,
    pub fee_proportional_millionths: u64,
    pub cltv_expiry_delta: u64,
}

/// Sub-structure for extra in 'decodepay'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Extra {
    pub tag: String,
    pub data: String,
}

/// 'decodepay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecodePay {
    pub currency: String,
    pub created_at: u64,
    pub expiry: u64,
    pub payee: String,
    pub amount_msat: Option<MSat>,
    pub description: Option<String>,
    pub description_hash: Option<String>,
    pub min_final_cltv_expiry: u64,
    pub payment_secret: Option<String>,
    pub features: Option<String>,
    pub fallbacks: Option<Vec<Fallback>>,
    pub routes: Option<Vec<Vec<DecodePayRoute>>>,
    pub extra: Option<Vec<Extra>>,
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
    pub features: String,
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
    pub amount_msat: MSat,
    pub address: String,
    pub status: String,
}

/// Sub-structure for 'listfunds' channel
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListFundsChannel {
    pub peer_id: String,
    pub connected: bool,
    pub short_channel_id: Option<String>,
    pub our_amount_msat: MSat,
    pub amount_msat: MSat,
    pub funding_txid: String,
    pub funding_output: u64,
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
    pub address: Option<String>,
    pub bech32: Option<String>,
    #[serde(rename = "p2sh-segwit")]
    pub p2sh_segwit: Option<String>,
}

/// 'stop' command
pub type Stop = String;
