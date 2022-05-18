//! RPC method built inside the plugin that are required
//! by core lightning in other to configure the plugin at startup.
//!
//! author: https://github.com/vincenzopalazzo
use crate::commands::{
    types::{InitConf, RPCHookInfo, RPCMethodInfo},
    RPCCommand,
};
// FIXME: move this inside the common crater
use crate::commands::json_utils::{add_bool, add_vec, init_payload};
use crate::plugin::Plugin;
use crate::types::RpcOption;
use serde_json::Value;

#[derive(Clone)]
pub struct ManifestRPC {}

impl<T: Clone> RPCCommand<T> for ManifestRPC {
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
        add_vec::<String>(
            &mut response,
            "subscriptions",
            plugin.rpc_nofitication.keys().cloned().collect(),
        );
        add_vec::<RPCHookInfo>(
            &mut response,
            "hooks",
            plugin.hook_info.clone().into_iter().collect(),
        );
        // FIXME: adding possibility to register a plugin notification
        add_bool(&mut response, "dynamic", plugin.dynamic);
        response
    }
}

#[derive(Clone)]
pub struct InitRPC {}

impl<T: Clone> RPCCommand<T> for InitRPC {
    fn call<'c>(&self, _plugin: &mut Plugin<T>, request: &'c Value) -> Value {
        let response = init_payload();
        let _conf: InitConf = serde_json::from_value(request.to_owned()).unwrap();
        response
    }
}
