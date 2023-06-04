//! plugin_macros is a rust crate that provide a sequence of helper
//! function to allow the user of the API to write a plugin
//! with less code.
//!
//! author: https://github.com/vincenzopalazzo
use kproc_parser::kparser::KParserTracer;
use kproc_parser::proc_macro::TokenStream;

mod notification;
mod plugin;
mod rpc_method;

mod attr_parser;

struct Tracer;

impl KParserTracer for Tracer {
    fn log(&self, msg: &str) {
        eprintln!("\x1b[93mkproc-tracing\x1b[1;97m {msg}");
    }
}
/// procedural macros that can be used wit the following code
/// ```no_run
/// use serde_json::{json, Value};
/// use clightningrpc_plugin_macros::{rpc_method, plugin};
/// use clightningrpc_plugin::commands::RPCCommand;
/// use clightningrpc_plugin::plugin::Plugin;
/// use clightningrpc_plugin::errors::PluginError;
///
/// #[derive(Clone)]
/// struct State;
///
/// impl State {
///    pub fn new() -> Self {
///        Self
///    }
/// }
///
/// #[rpc_method(
///     rpc_name = "foo",
///     description = "This is a simple and short description"
/// )]
/// pub fn foo_rpc(plugin: &mut Plugin<State>, request: Value) -> Result<Value, PluginError> {
///     Ok(json!({"is_dynamic": plugin.dynamic, "rpc_request": request}))
/// }
///
/// fn main() {
///     let plugin = plugin! {
///         state: State::new(),
///         dynamic: true,
///         notification: [],
///         methods: [
///             foo_rpc,
///         ],
///     };
///     plugin.start();
/// }
/// ```
#[proc_macro]
pub fn plugin(attr: TokenStream) -> TokenStream {
    plugin::parse(attr)
}

/// procedural macros that can be used wit the following code
/// ```no_run
/// use serde_json::{json, Value};
/// use clightningrpc_plugin_macros::rpc_method;
/// use clightningrpc_plugin::commands::RPCCommand;
/// use clightningrpc_plugin::plugin::Plugin;
/// use clightningrpc_plugin::errors::PluginError;
///
/// #[derive(Clone)]
/// struct State;
///
/// #[rpc_method(
///     rpc_name = "foo",
///     description = "This is a simple and short description"
/// )]
/// pub fn foo_rpc(plugin: &mut Plugin<State>, request: Value) -> Result<Value, PluginError> {
///     Ok(json!({"is_dynamic": plugin.dynamic, "rpc_request": request}))
/// }
/// ```
#[proc_macro_attribute]
pub fn rpc_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    rpc_method::parse(attr, item)
}

/// procedural macros that can be used wit the following code
/// ```no_run
/// use serde_json::{json, Value};
/// use clightningrpc_plugin_macros::notification;
/// use clightningrpc_plugin::commands::RPCCommand;
/// use clightningrpc_plugin::plugin::Plugin;
/// use clightningrpc_plugin::types::LogLevel;
/// use clightningrpc_plugin::errors::PluginError;
///
/// #[derive(Clone)]
/// struct State;
///
/// #[notification(on = "rpc_command")]
/// fn on_rpc(plugin: &mut Plugin<State>, request: &Value) {
///    plugin.log(LogLevel::Info, "received an RPC notification");
/// }
/// ```
#[proc_macro_attribute]
pub fn notification(attr: TokenStream, item: TokenStream) -> TokenStream {
    notification::parse(attr, item)
}
