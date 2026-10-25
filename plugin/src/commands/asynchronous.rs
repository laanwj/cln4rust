//! Methods and hooks written as async functions (feature `async`).
//!
//! An [`AsyncRPCCommand`] gets a clone of the plugin state instead of the
//! plugin, so its future can run as a task on the tokio runtime that
//! [`Plugin::start_async`] reads requests on: the plugin goes on reading
//! the next requests while it is pending.
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;

use super::RPCCommand;
use crate::errors::PluginError;
use crate::plugin::Plugin;
use crate::responder::Responder;

/// The future an [`AsyncRPCCommand`] answers with.
pub type CommandFuture = Pin<Box<dyn Future<Output = Result<Value, PluginError>> + Send>>;

/// A method or hook handler written as an async function.
///
/// Any `Fn(T, Value) -> impl Future<Output = Result<Value, PluginError>>`
/// whose future is `Send` implements it, so an `async fn` can be
/// registered directly:
///
/// ```no_run
/// # use clightningrpc_plugin::{errors::PluginError, plugin::Plugin};
/// # use serde_json::{json, Value};
/// async fn hello(_state: (), request: Value) -> Result<Value, PluginError> {
///     Ok(json!({ "hello": request }))
/// }
///
/// # async fn run() {
/// let mut plugin = Plugin::new((), false);
/// plugin.add_async_rpc_method("hello", "", "Say hello", hello);
/// plugin.start_async().await;
/// # }
/// ```
pub trait AsyncRPCCommand<T>: Send + Sync {
    fn call(&self, state: T, request: Value) -> CommandFuture;
}

impl<T, F, Fut> AsyncRPCCommand<T> for F
where
    F: Fn(T, Value) -> Fut + Send + Sync,
    Fut: Future<Output = Result<Value, PluginError>> + Send + 'static,
{
    fn call(&self, state: T, request: Value) -> CommandFuture {
        Box::pin(self(state, request))
    }
}

/// An [`AsyncRPCCommand`] as an [`RPCCommand`]: each request runs as a
/// task on the current tokio runtime and is answered when it completes.
///
/// [`Plugin::add_async_rpc_method`] wraps methods in it; wrap a hook in it
/// to register it with [`Plugin::register_hook`] or
/// [`Plugin::register_hook_with_filters`].
pub struct AsyncCommand<T>(Arc<dyn AsyncRPCCommand<T>>);

impl<T> AsyncCommand<T> {
    pub fn new<C: AsyncRPCCommand<T> + 'static>(command: C) -> Self {
        AsyncCommand(Arc::new(command))
    }
}

impl<T> Clone for AsyncCommand<T> {
    fn clone(&self) -> Self {
        AsyncCommand(self.0.clone())
    }
}

impl<T: Clone + 'static> RPCCommand<T> for AsyncCommand<T> {
    fn call_deferred(&self, plugin: &mut Plugin<T>, request: Value, responder: Responder) {
        match tokio::runtime::Handle::try_current() {
            Ok(runtime) => {
                let future = self.0.call(plugin.state.clone(), request);
                // A panicking task drops the responder, which answers with
                // an error.
                runtime.spawn(async move { responder.respond(future.await) });
            }
            Err(_) => responder.respond(Err(crate::error!(
                "async handlers need the plugin to run with `start_async`"
            ))),
        }
    }
}
