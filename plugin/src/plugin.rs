//! Core of the plugin API
//!
//! Unofficial API interface to develop plugin in Rust.
use crate::commands::json_utils::{add_str, init_payload, init_success_response};
use crate::commands::{
    builtin::{InitRPC, ManifestRPC},
    types::{InitConf, RPCHookInfo, RPCMethodInfo},
    RPCCommand,
};
use crate::errors::PluginError;
use crate::types::{LogLevel, RpcOption};
use clightningrpc_common::types::Request;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::string::String;
use std::{io, io::Write};

pub type OnInit<T> = dyn Fn(&mut Plugin<T>) -> Value + Send + 'static;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Plugin<T>
where
    // FIXME: move the static life time to a local life time for plugin
    T: 'static + Clone,
{
    pub(crate) state: T,
    /// all the option contained inside the
    /// hash map.
    pub option: HashSet<RpcOption>,
    /// all the options rpc method that the
    /// plugin need to support, included the builtin rpc method.
    pub rpc_method: HashMap<String, Box<dyn RPCCommand<T>>>,
    /// keep the info of the method in a separate list
    /// FIXME: move the RPCMethodInfo as key of the rpc_method map.
    pub rpc_info: HashSet<RPCMethodInfo>,
    /// all the hook where the plugin is register during the configuration
    pub rpc_hook: HashMap<String, Box<dyn RPCCommand<T>>>,
    /// keep all the info about the hooks in a separate set.
    /// FIXME: put the RPCHookInfo as key of the hash map.
    pub hook_info: HashSet<RPCHookInfo>,
    /// all the notification that the plugin is register on
    pub rpc_notification: HashMap<String, Box<dyn RPCCommand<T>>>,
    /// mark a plugin as dynamic, in this way the plugin can be run
    /// from core lightning without stop the lightningd daemon
    pub dynamic: bool,
    /// onInit callback called when the method on init is runned.
    on_init: Option<&'static OnInit<T>>,
}

impl<'a, T: 'a + Clone> Plugin<T> {
    pub fn new(state: T, dynamic: bool) -> Self {
        return Plugin {
            state,
            option: HashSet::new(),
            rpc_method: HashMap::new(),
            rpc_info: HashSet::new(),
            rpc_hook: HashMap::new(),
            hook_info: HashSet::new(),
            rpc_notification: HashMap::new(),
            dynamic,
            on_init: None,
        };
    }

    pub fn on_init(&'a mut self, callback: &'static OnInit<T>) -> &'a Self {
        self.on_init = Some(callback);
        self
    }

    pub fn log(&self, level: LogLevel, msg: &str) -> &Self {
        let mut writer = io::stdout();
        let mut payload = init_payload();
        // FIXME: add other log level supported by cln
        let level = match level {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
        };
        add_str(&mut payload, "level", level);
        add_str(&mut payload, "message", msg);
        let request = Request {
            id: None,
            jsonrpc: "2.0",
            method: "log",
            params: payload,
        };
        writer
            .write_all(serde_json::to_string(&request).unwrap().as_bytes())
            .unwrap();
        writer.flush().unwrap();
        self
    }

    pub fn add_opt(
        &mut self,
        name: &str,
        opt_type: &str,
        def_val: Option<String>,
        description: &str,
        deprecated: bool,
    ) -> &mut Self {
        self.option.insert(RpcOption {
            name: name.to_string(),
            opt_typ: opt_type.to_string(),
            default: def_val,
            description: description.to_string(),
            deprecated,
        });
        self
    }

    // FIXME: adding the long description as parameter
    pub fn add_rpc_method<F: 'static>(
        &'a mut self,
        name: &str,
        usage: &str,
        description: &str,
        callback: F,
    ) -> &mut Self
    where
        F: RPCCommand<T> + 'static,
    {
        self.rpc_method.insert(name.to_owned(), Box::new(callback));
        self.rpc_info.insert(RPCMethodInfo {
            name: name.to_string(),
            usage: usage.to_string(),
            description: description.to_string(),
            long_description: description.to_string(),
            deprecated: false,
        });
        self
    }

    fn call_rpc_method(
        &'a mut self,
        name: &str,
        params: &serde_json::Value,
    ) -> Result<serde_json::Value, PluginError> {
        let command = self.rpc_method.get(name).unwrap().clone();
        command.call(self, params)
    }

    fn handle_notification(&'a mut self, name: &str, params: &serde_json::Value) {
        let notification = self.rpc_notification.get(name).unwrap().clone();
        if let Err(json_res) = notification.call(self, params) {
            self.log(
                LogLevel::Debug,
                format!("Notification end with and error: {}", json_res).as_str(),
            );
        }
    }

    pub fn register_hook<F: 'static>(
        &'a mut self,
        hook_name: &str,
        before: Option<Vec<String>>,
        after: Option<Vec<String>>,
        callback: F,
    ) -> &mut Self
    where
        F: RPCCommand<T> + 'static,
    {
        self.rpc_hook
            .insert(hook_name.to_owned(), Box::new(callback));
        self.hook_info.insert(RPCHookInfo {
            name: hook_name.to_owned(),
            before,
            after,
        });
        self
    }

    pub fn register_notification<F: 'static>(&mut self, name: &str, callback: F) -> &mut Self
    where
        F: 'static + RPCCommand<T> + Clone,
    {
        self.rpc_notification
            .insert(name.to_owned(), Box::new(callback));
        self
    }

    fn write_respose(
        &mut self,
        result: &Result<serde_json::Value, PluginError>,
        response: &mut serde_json::Value,
    ) {
        match result {
            Ok(json_resp) => response["result"] = json_resp.to_owned(),
            Err(json_err) => {
                let err_resp = serde_json::to_value(json_err).unwrap();
                response["error"] = err_resp;
            }
        }
    }

    pub fn start(&'a mut self) {
        let reader = io::stdin();
        let mut writer = io::stdout();
        let mut buffer = String::new();

        self.rpc_method
            .insert("getmanifest".to_owned(), Box::new(ManifestRPC {}));
        self.rpc_method.insert(
            "init".to_owned(),
            Box::new(InitRPC::<T> {
                on_init: self.on_init,
            }),
        );
        // FIXME: core lightning end with the double endline, so this can cause
        // problem for some input reader.
        // we need to parse the writer, and avoid this while loop
        loop {
            let _ = reader.read_line(&mut buffer);
            let req_str = buffer.to_string();
            if req_str.trim().is_empty() {
                continue;
            }
            buffer.clear();
            let request: Request<serde_json::Value> = serde_json::from_str(&req_str).unwrap();
            if let Some(id) = request.id {
                // whe the id is specified this is a RPC or Hook, so we need to return a response
                let response = self.call_rpc_method(request.method, &request.params);
                let mut rpc_response = init_success_response(id);
                self.write_respose(&response, &mut rpc_response);
                writer
                    .write_all(serde_json::to_string(&rpc_response).unwrap().as_bytes())
                    .unwrap();
                writer.flush().unwrap();
            } else {
                // in case of the id is None, we are receiving the notification, so the server is not
                // interested in the answer.
                self.handle_notification(request.method, &request.params);
            }
        }
    }
}
