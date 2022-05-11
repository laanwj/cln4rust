//! RPC method built inside the plugin that are required
//! by core lightning in other to configure the plugin at startup.
//!
//! author: https://github.com/vincenzopalazzo
use crate::commands::{
    types::{InitConf, RPCMethodInfo},
    RPCMethod,
};
// FIXME: move this inside the common crater
use crate::commands::json_utils::{add_bool, add_vec, init_payload};
use crate::plugin::Plugin;
use crate::types::RpcOption;
use serde_json::Value;

#[derive(Clone)]
pub struct ManifestRPC {}

impl<T: Clone> RPCMethod<T> for ManifestRPC {
    fn call<'c>(&self, plugin: &mut Plugin<T>, _request: &'c Value) -> Value {
        let mut response = init_payload();
        add_vec::<RpcOption>(
            &mut response,
            "options",
            plugin.option.clone().into_iter().collect(),
        );
        add_vec::<RPCMethodInfo>(
            &mut response,
            "rpcmethods",
            plugin.rpc_info.clone().into_iter().collect(),
        );
        // TODO: fill later
        add_vec::<String>(&mut response, "subscriptions", vec![]);
        add_vec::<String>(&mut response, "hooks", vec![]);
        add_bool(&mut response, "dynamic", plugin.dynamic);
        response
    }
}

#[derive(Clone)]
pub struct InitRPC {}

impl<T: Clone> RPCMethod<T> for InitRPC {
    fn call<'c>(&self, _plugin: &mut Plugin<T>, request: &'c Value) -> Value {
        let response = init_payload();
        let _conf: InitConf = serde_json::from_value(request.to_owned()).unwrap();
        response
    }
}
