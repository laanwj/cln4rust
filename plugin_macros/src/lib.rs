//! plugin_macros is a rust crate that provide a sequence of helper
//! function to allow the user of the API to write a plugin
//! with less code.
//!
//! author: https://github.com/vincenzopalazzo
use kproc_parser::kparser::KParserTracer;
use kproc_parser::proc_macro::TokenStream;

mod notification;
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
/// pub fn foo_rpc(_plugin: &mut Plugin<State>, _request: Value) -> Result<Value, PluginError> {
///     /// The name of the parameters can be used only if used, otherwise can be omitted
///     /// the only rules that the macros require is to have a propriety with the following rules:
///     /// - Plugin as _plugin
///     /// - CLN JSON request as _request
///     /// The function parameter can be specified in any order.
///     Ok(json!({"is_dynamic": _plugin.dynamic, "rpc_request": _request}))
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
