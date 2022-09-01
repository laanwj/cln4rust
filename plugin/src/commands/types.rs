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

#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RPCHookInfo {
    pub name: String,
    pub before: Option<Vec<String>>,
    pub after: Option<Vec<String>>,
}

#[derive(Deserialize, Clone)]
pub struct InitConf {
    pub options: serde_json::Value,
    pub configuration: ConfFiled,
}

#[derive(Deserialize, Clone)]
pub struct ConfFiled {
    #[serde(rename = "lightning-dir")]
    pub lightning_dir: String,
    #[serde(rename = "rpc-file")]
    pub rpc_file: String,
    pub startup: bool,
    pub network: String,
    pub feature_set: HashMap<String, String>,
    pub proxy: Option<ProxyInfo>,
    #[serde(rename = "torv3-enabled")]
    pub torv3_enabled: Option<bool>,
    pub always_use_proxy: Option<bool>,
}

#[derive(Deserialize, Clone)]
pub struct ProxyInfo {
    #[serde(alias = "type")]
    pub tup: String,
    pub address: String,
    pub port: i64,
}
