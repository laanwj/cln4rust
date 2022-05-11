//! command types contains all the type that are important
//! to implement the plugin library.
//!
//! author: https://github.com/vincenzopalazzo
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq, Serialize)]
pub struct RPCMethodInfo {
    pub name: String,
    pub usage: String,
    pub description: String,
    pub long_description: String,
    pub deprecated: bool,
}

#[derive(Deserialize)]
pub struct InitConf {
    pub options: serde_json::Value,
    pub configuration: ConfFiled,
}

#[derive(Deserialize)]
pub struct ConfFiled {
    #[serde(rename = "lightning-dir")]
    pub lightning_dir: String,
    #[serde(rename = "rpc-file")]
    pub rpc_file: String,
    pub startup: bool,
    pub network: String,
    pub feature_set: HashMap<String, String>,
    pub proxy: ProxyInfo,
    #[serde(rename = "torv3-enabled")]
    pub torv3_enabled: bool,
    pub always_use_proxy: bool,
}

#[derive(Deserialize)]
pub struct ProxyInfo {
    #[serde(alias = "type")]
    pub tup: String,
    pub address: String,
    pub port: i64,
}
