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

/// 'getinfo' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetInfo {}

/// 'feerates' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeeRates {
    pub style: String,
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

/// 'decodepay' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecodePay {
    pub bolt11: String,
    pub description: Option<String>,
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

/// 'stop' command
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stop {}
