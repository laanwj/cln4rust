//! Core of the plugin API
//!
//! Unofficial API interface to develop plugin in Rust.

use std::collections::HashMap;

use serde_json;

/// Call back to register a new RPC method inside the plugin
type RPCMethodCallBack<'a, T> = Box<dyn Fn(&'a mut Plugin<T>, &'a serde_json::Value) + 'static>;

/// Generic traits to implement a plugin for core lightning.
pub trait IPlugin<'a, T: 'a> {
    /// Create the new plugin with the state.
    fn new() -> &'a mut Self;

    /// log the message on core lightning log.
    /// TODO: finish the signature of the method.
    fn log(&'a self) -> &'a mut Self;

    /// Add a new plugin option.
    /// TODO: finish the signature of the method
    fn add_opt(&'a mut self) -> &'a mut Self;

    /// Add a new RPC method defined from the user
    /// TODO: finish the signature of the method
    fn add_rpc_method<F>(&'a mut self, callback: F) -> &'a mut Self
    where
        F: Fn(&'a mut Plugin<T>, &'a serde_json::Value) + 'static;

    /// Register a call back for a specific hook
    /// TODO: finish the signature of the method
    fn register_hook(&'a mut self) -> &'a mut Self;

    /// Register a call back for a specific notification
    /// TODO: finish the signature of the method
    fn register_notification(&'a mut self) -> &'a mut Self;

    /// Start the plugin on the I/O
    fn start(&'a mut self, state: &'a T);
}

pub struct Plugin<'a, T> {
    state: &'a T,
    /// the plugin option.
    /// TODO: Finish the type here
    option: HashMap<String, ()>,
    /// the rpc method of the plugin.
    /// TODO: finish the type
    rpc_method: HashMap<String, RPCMethodCallBack<'a, T>>,
}

impl<'a: 'static, T: 'static> IPlugin<'a, T> for Plugin<'a, T> {
    fn new() -> &'a mut Self {
        todo!()
    }

    fn log(&'a self) -> &'a mut Self {
        todo!()
    }

    fn add_opt(&'a mut self) -> &'a mut Self {
        self.option.insert("".to_owned(), ());
        self
    }

    fn add_rpc_method<F>(&'a mut self, callback: F) -> &'a mut Self
    where
        F: Fn(&'a mut Plugin<T>, &'a serde_json::Value) + 'static,
    {
        self.rpc_method.insert("".to_owned(), Box::new(callback));
        self
    }

    fn register_hook(&'a mut self) -> &'a mut Self {
        todo!()
    }

    fn register_notification(&'a mut self) -> &'a mut Self {
        todo!()
    }

    fn start(&'a mut self, state: &'a T) {
        self.state = &state;
    }
}
