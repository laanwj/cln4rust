//!
//!
//! author: https://github.com/vincenzopalazzo
pub mod builtin;
pub mod types;

use serde_json::json;

use super::plugin::Plugin;
use crate::errors::PluginError;
use crate::responder::Responder;

/// RPCCommand is an implementation of the callback using the command pattern.
///
/// The usage of the pattern is a design choice to avoid to define a callback type
/// in contrast, it is more complex but the plugin_macros package will help to simplify the API.
pub trait RPCCommand<T: Clone>: RPCCommandClone<T> {
    /// call is a generic method that it is used to simulate the callback.
    fn call<'c>(
        &self,
        _: &mut Plugin<T>,
        _: serde_json::Value,
    ) -> Result<serde_json::Value, PluginError> {
        Ok(json!({}))
    }

    /// Handle a request that may be answered after this method returns.
    ///
    /// The default runs [`RPCCommand::call`] and answers right away, so the
    /// plugin reads the next request only once this one is answered.
    /// Override it to move `responder` to another thread or task and return
    /// at once: the plugin then serves other requests while this one is in
    /// progress, and the response is written when `responder` answers.
    fn call_deferred(
        &self,
        plugin: &mut Plugin<T>,
        request: serde_json::Value,
        responder: Responder,
    ) {
        responder.respond(self.call(plugin, request));
    }

    /// void call is a generic method that it is used to simulate a callback with a void return type
    fn call_void<'c>(&self, _plugin: &mut Plugin<T>, _request: &'c serde_json::Value) {}
}

// Splitting RPCCommandClone into its own trait allows us to provide a blanket
// implementation for all compatible types, without having to implement the
// rest of RPCCommand. In this case, we implement it for all types that have
// 'static lifetime (*i.e.* they don't contain non-'static pointers), and
// implement both RPCCommand and Clone.  Don't ask me how the compiler resolves
// implementing RPCCommandClone for dyn Animal when RPCCommand requires RPCCommandClone;
// I have *no* idea why this works.
pub trait RPCCommandClone<T: Clone> {
    fn clone_box(&self) -> Box<dyn RPCCommand<T>>;
}

impl<F, T: Clone> RPCCommandClone<T> for F
where
    F: 'static + RPCCommand<T> + Clone,
{
    fn clone_box(&self) -> Box<dyn RPCCommand<T>> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl<T: Clone> Clone for Box<dyn RPCCommand<T>> {
    fn clone(&self) -> Box<dyn RPCCommand<T>> {
        self.clone_box()
    }
}
