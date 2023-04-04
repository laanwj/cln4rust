//! RPC method built inside the plugin that are required
//! by core lightning in other to configure the plugin at startup.
//!
//! author: https://github.com/vincenzopalazzo
use std::collections::HashMap;
use std::sync::Arc;

use crate::commands::{
    types::{RPCHookInfo, RPCMethodInfo},
    RPCCommand,
};
use crate::errors::PluginError;
use crate::plugin::Plugin;
use crate::types::RpcOption;
use clightningrpc_common::json_utils::{add_bool, add_vec, init_payload};
use serde_json::Value;

use super::types::InitConf;

#[derive(Clone)]
/// Type to define the manifest method and its attributes, used during plugin initialization
pub struct ManifestRPC {}

impl<T: Clone> RPCCommand<T> for ManifestRPC {
    fn call<'c>(&self, plugin: &mut Plugin<T>, _request: Value) -> Result<Value, PluginError> {
        let mut response = init_payload();
        add_vec::<RpcOption>(
            &mut response,
            "options",
            plugin.option.clone().into_iter().map(|it| it.1).collect(),
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
/// Type to define the init method and its attributes, used in plugin
pub struct InitRPC<T: 'static + Clone> {
    pub(crate) on_init: Option<Arc<dyn Fn(&mut Plugin<T>) -> Value>>,
}

impl<T: Clone> InitRPC<T> {
    fn parse_option(&self, plugin: &mut Plugin<T>, options: &HashMap<String, serde_json::Value>) {
        for option_name in options.keys() {
            let option = options.get(option_name).unwrap();
            plugin.option.get_mut(option_name).unwrap().value = Some(option.to_owned());
        }
    }
}

impl<T: Clone> RPCCommand<T> for InitRPC<T> {
    fn call<'c>(&self, plugin: &mut Plugin<T>, request: Value) -> Result<Value, PluginError> {
        let mut response = init_payload();
        let init: InitConf = serde_json::from_value(request.to_owned()).unwrap();
        plugin.configuration = Some(init.configuration);
        self.parse_option(plugin, &init.options);

        if let Some(callback) = &self.on_init {
            response = callback(plugin);
        }
        Ok(response)
    }
}
