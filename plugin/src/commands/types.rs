//! contains types that are essential to implement the plugin library.
//!
//! author: https://github.com/vincenzopalazzo
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq, Serialize)]
/// Type to define metadata for custom RPC Methods
pub struct RPCMethodInfo {
    pub name: String,
    pub usage: String,
    pub description: String,
    pub long_description: String,
    pub deprecated: bool,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
/// Type to define metadata for custom RPC Hooks
pub struct RPCHookInfo {
    pub name: String,
    pub before: Option<Vec<String>>,
    pub after: Option<Vec<String>>,
}

#[derive(Deserialize, Clone)]
/// Type to define attributes for the plugin's init method
pub(crate) struct InitConf {
    pub options: HashMap<String, serde_json::Value>,
    pub configuration: CLNConf,
}

#[derive(Deserialize, Clone)]
/// Type to define the configuration options for the plugin's init method
pub struct CLNConf {
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
/// Type to define the network information for the plugin's configuration
pub struct ProxyInfo {
    #[serde(alias = "type")]
    pub tup: String,
    pub address: String,
    pub port: i64,
}
