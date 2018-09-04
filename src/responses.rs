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
// Structures representing responses to API calls

/// structure for network addresses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkAddress {
    #[serde(rename = "type")]
    pub type_: String,
    pub address: String,
    pub port: String,
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
    pub blockheight: i64,
    pub network: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRatesInner {
    pub urgent: i64,
    pub normal: i64,
    pub slow: i64,
    pub min_acceptable: i64,
    pub max_acceptable: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRatesOnchain {
    pub opening_channel_satoshis: i64,
    pub mutual_close_satoshis: i64,
    pub unilateral_close_satoshis: i64,
}

/// 'feerates' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRates {
    pub perkb: Option<FeeRatesInner>,
    pub perkw: Option<FeeRatesInner>,
    pub onchain_fee_estimates: Option<FeeRatesOnchain>,
}

/// Sub-structure for peer in 'listpeers'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Peer {
    pub id: String,
}

/// 'listpeers' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListPeers {
    pub peers: Vec<Peer>,
}
