//! Core of the plugin API
//!
//! Unofficial API interface to develop plugin in Rust.
use crate::commands::builtin::{InitRPC, ManifestRPC};
use crate::commands::types::{CLNConf, RPCHookInfo, RPCMethodInfo};
use crate::commands::RPCCommand;
use crate::errors::PluginError;
use crate::types::{LogLevel, RpcOption};
use clightningrpc_common::json_utils::{add_str, init_payload, init_success_response};
use clightningrpc_common::types::Request;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::string::String;
use std::sync::Arc;
use std::{io, io::Write};

#[cfg(feature = "log")]
pub use log::*;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Plugin<T>
where
    // FIXME: move the static life time to a local life time for plugin
    T: 'static + Clone,
{
    pub state: T,
    /// all the option contained inside the
    /// hash map.
    pub option: HashMap<String, RpcOption>,
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
    /// core lightning configuration sent with the init call.
    pub configuration: Option<CLNConf>,
    /// onInit callback called when the method on init is ran.
    on_init: Option<Arc<dyn Fn(&mut Plugin<T>) -> Value>>,
}

#[cfg(feature = "log")]
pub struct Log;

#[cfg(feature = "log")]
impl log::Log for Log {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level: LogLevel = record.level().into();
            let msg = record.args();

            let mut writer = io::stdout();
            let mut payload = init_payload();
            add_str(&mut payload, "level", &level.to_string());
            add_str(&mut payload, "message", &format!("{msg}"));
            let request = Request {
                id: None,
                jsonrpc: "2.0".to_owned(),
                method: "log".to_owned(),
                params: payload,
            };
            writer
                .write_all(serde_json::to_string(&request).unwrap().as_bytes())
                .unwrap();
            writer.flush().unwrap();
        }
    }

    fn flush(&self) {}
}

impl<'a, T: 'a + Clone> Plugin<T> {
    pub fn new(state: T, dynamic: bool) -> Self {
        Plugin {
            state,
            option: HashMap::new(),
            rpc_method: HashMap::new(),
            rpc_info: HashSet::new(),
            rpc_hook: HashMap::new(),
            hook_info: HashSet::new(),
            rpc_notification: HashMap::new(),
            dynamic,
            configuration: None,
            on_init: None,
        }
    }

    pub fn on_init<C: 'static>(&'a mut self, callback: C) -> Self
    where
        C: Fn(&mut Plugin<T>) -> Value,
    {
        self.on_init = Some(Arc::new(callback));
        self.clone()
    }

    pub fn log(&self, level: LogLevel, msg: &str) {
        let mut writer = io::stdout();
        let mut payload = init_payload();
        add_str(&mut payload, "level", &level.to_string());
        add_str(&mut payload, "message", msg);
        let request = Request {
            id: None,
            jsonrpc: "2.0".to_owned(),
            method: "log".to_owned(),
            params: payload,
        };
        writer
            .write_all(serde_json::to_string(&request).unwrap().as_bytes())
            .unwrap();
        writer.flush().unwrap();
    }

    /// register the plugin option.
    pub fn add_opt(
        &mut self,
        name: &str,
        opt_type: &str,
        def_val: Option<String>,
        description: &str,
        deprecated: bool,
    ) -> &mut Self {
        self.option.insert(
            name.to_owned(),
            RpcOption {
                name: name.to_string(),
                opt_typ: opt_type.to_string(),
                default: def_val,
                description: description.to_string(),
                deprecated,
                value: None,
            },
        );
        self
    }

    /// get an optionue that cln sent back to the plugin.
    pub fn get_opt<R: for<'de> serde::de::Deserialize<'de>>(
        &self,
        name: &str,
    ) -> Result<R, PluginError> {
        let opt = self.option.get(name).unwrap();
        Ok(opt.value())
    }

    // FIXME: adding the long description as parameter
    pub fn add_rpc_method<F: 'static>(
        &'a mut self,
        name: &str,
        usage: &str,
        description: &str,
        callback: F,
    ) -> Self
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
        self.clone()
    }

    fn call_rpc_method(
        &'a mut self,
        name: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, PluginError> {
        let command = self.rpc_method.get(name).unwrap().clone();
        command.call(self, params)
    }

    fn handle_notification(&'a mut self, name: &str, params: serde_json::Value) {
        let notification = self.rpc_notification.get(name).unwrap().clone();
        if let Err(json_res) = notification.call(self, params) {
            self.log(
                LogLevel::Debug,
                format!("Notification end with and error: {json_res}").as_str(),
            );
        }
    }

    pub fn register_hook<F: 'static>(
        &'a mut self,
        hook_name: &str,
        before: Option<Vec<String>>,
        after: Option<Vec<String>>,
        callback: F,
    ) -> Self
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
        self.clone()
    }

    pub fn register_notification<F: 'static>(&mut self, name: &str, callback: F) -> Self
    where
        F: 'static + RPCCommand<T> + Clone,
    {
        self.rpc_notification
            .insert(name.to_owned(), Box::new(callback));
        self.clone()
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

    pub fn start(mut self) {
        let reader = io::stdin();
        let mut writer = io::stdout();
        let mut buffer = String::new();
        #[cfg(feature = "log")]
        {
            let _ = log::set_logger(&Log {}).map(|()| log::set_max_level(LevelFilter::Trace));
        }
        self.rpc_method
            .insert("getmanifest".to_owned(), Box::new(ManifestRPC {}));
        self.rpc_method.insert(
            "init".to_owned(),
            Box::new(InitRPC::<T> {
                on_init: self.on_init.clone(),
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
                // when the id is specified this is a RPC or Hook, so we need to return a response
                let response = self.call_rpc_method(&request.method, request.params);
                let mut rpc_response = init_success_response(id);
                self.write_respose(&response, &mut rpc_response);
                writer
                    .write_all(serde_json::to_string(&rpc_response).unwrap().as_bytes())
                    .unwrap();
                writer.flush().unwrap();
            } else {
                // in case of the id is None, we are receiving the notification, so the server is not
                // interested in the answer.
                self.handle_notification(&request.method, request.params);
            }
        }
    }
}
