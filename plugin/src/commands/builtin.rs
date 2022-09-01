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
use crate::errors::PluginError;
use crate::plugin::{OnInit, Plugin};
use crate::types::RpcOption;
use serde_json::Value;

#[derive(Clone)]
pub struct ManifestRPC {}

impl<T: Clone> RPCCommand<T> for ManifestRPC {
    fn call<'c>(&self, plugin: &mut Plugin<T>, _request: &'c Value) -> Result<Value, PluginError> {
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
            plugin.rpc_notification.keys().cloned().collect(),
        );
        add_vec::<RPCHookInfo>(
            &mut response,
            "hooks",
            plugin.hook_info.clone().into_iter().collect(),
        );
        // FIXME: adding possibility to register a plugin notification
        add_bool(&mut response, "dynamic", plugin.dynamic);
        Ok(response)
    }
}

#[derive(Clone)]
pub struct InitRPC<T: 'static + Clone> {
    pub(crate) on_init: Option<&'static OnInit<T>>,
}

impl<T: Clone> RPCCommand<T> for InitRPC<T> {
    fn call<'c>(&self, _plugin: &mut Plugin<T>, request: &'c Value) -> Result<Value, PluginError> {
        let response = init_payload();
        let conf: InitConf = serde_json::from_value(request.to_owned()).unwrap();
        if let Some(callback) = self.on_init {
            (*callback)(_plugin);
        }
        Ok(response)
    }
}
