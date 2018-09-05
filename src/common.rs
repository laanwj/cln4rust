#![allow(missing_docs)]
//! Common structures between requests and responses

/// Sub-structure for route in 'pay', 'getroute' and 'sendpay'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RouteItem {
    pub id: String,
    pub channel: String,
    pub msatoshi: i64,
    pub delay: i64,
}
