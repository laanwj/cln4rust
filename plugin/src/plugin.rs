//! Core of the plugin API
//!
//! Unofficial API interface to develop plugin in Rust.
use crate::commands::{
    builtin::{InitRPC, ManifestRPC},
    types::RPCMethodInfo,
    RPCMethod,
};
use crate::types::RpcOption;

use crate::commands::json_utils::init_success_response;
use clightningrpc_common::types::Request;
use std::collections::{HashMap, HashSet};
use std::io;
use std::string::String;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Plugin<T>
where
    T: Clone,
{
    state: T,
    /// all the option contained inside the
    /// hash map.
    pub option: HashSet<RpcOption>,
    /// all the options rpc method that the
    /// plugin need to support, included the builtin rpc method.
    pub rpc_method: HashMap<String, Box<dyn RPCMethod<T>>>,
    /// keep the info of the method in a separate list
    /// FIXME: move the RPCMethodInfo as key of the rpc_method map.
    pub rpc_info: HashSet<RPCMethodInfo>,
    /// mark a plugin as dynamic, in this way the plugin can be run
    /// from core lightning without stop the lightningd deamon
    pub dynamic: bool,
}

impl<'a, T: 'a + Clone> Plugin<T> {
    pub fn new(state: T, dynamic: bool) -> Self {
        return Plugin {
            state,
            option: HashSet::new(),
            rpc_method: HashMap::new(),
            rpc_info: HashSet::new(),
            dynamic,
        };
    }

    pub fn log(&self) -> &Self {
        todo!()
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

    //TODO: adding the long description as parameter
    // FIXME: see if with the macros the API can be improved
    pub fn add_rpc_method<F: 'static>(
        &'a mut self,
        name: &str,
        usage: &str,
        description: &str,
        callback: F,
    ) -> &mut Self
    where
        F: RPCMethod<T> + 'static,
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

    fn call_rpc_method(&'a mut self, name: &str, params: &serde_json::Value) -> serde_json::Value {
        let command = self.rpc_method.get(name).unwrap().clone();
        command.call(self, params)
    }

    pub fn register_hook(&mut self) -> &mut Self {
        todo!()
    }

    pub fn register_notification(&mut self) -> &mut Self {
        todo!()
    }

    pub fn start(&'a mut self) {
        let writer = io::stdin();
        let mut buffer = String::new();

        self.rpc_method
            .insert("getmanifest".to_owned(), Box::new(ManifestRPC {}));
        self.rpc_method
            .insert("init".to_owned(), Box::new(InitRPC {}));
        // FIXME: core lightning end with the double endline, so this can cause
        // problem for some input reader.
        // we need to parse the writer, and avoid this while loop
        loop {
            let _ = writer.read_line(&mut buffer);
            let req_str = buffer.to_string();
            if req_str.trim().is_empty() {
                continue;
            }
            buffer.clear();
            let request: Request<serde_json::Value> = serde_json::from_str(&req_str).unwrap();
            let response = self.call_rpc_method(request.method, &request.params);
            let mut rpc_response = init_success_response(request.id);
            rpc_response["result"] = response;
            // TODO: improve the stout writing!
            println!("{}", serde_json::to_string(&rpc_response).unwrap());
        }
    }
}
