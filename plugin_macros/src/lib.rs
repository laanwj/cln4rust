//! plugin_macros is a rust crate that provide a sequence of helper
//! function to allow the user of the API to write a plugin
//! with less code.
//!
//! author: https://github.com/vincenzopalazzo
use convert_case::{Case, Casing};
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::ToTokens;
use std::fmt;
use syn::{parse, parse_macro_input, AttributeArgs, Item, ItemFn};

/// The struct where the tools darling store the information
/// the macros, so we implement the struct where we want to unmarshall
/// the user information.
#[derive(FromMeta)]
struct RPCMethodMacro {
    /// rpc_name is the name of the plugin that the user want to use
    /// regarding the RPC method registered by the plugin.
    rpc_name: String,
    /// description is the short description that the user want to add
    /// to the rpc method.
    description: String,
    /// usage is some tips to give the user on how t use the rpc method
    #[darling(default)]
    usage: String,
    // FIXME: add the long description
}

/// Method to generate the RPC call in a string
/// format
struct RPCCall {
    /// original name of the rpc method specified
    /// by the user
    original_name: String,
    /// the name of the struct that will be created by the
    /// macros by write in the camel case the original_name
    struct_name: String,
    /// the function body of the method specified by the user
    /// in the function.
    fn_body: String,
    /// the description of the rpc method
    description: String,
    /// the usage tips that is use by core lightning to give tips to the user
    usage: String,
}

/// implementation fo the Display method of the rpc method that give
/// that convert the struct in the function call in a valid rust syntax.
impl fmt::Display for RPCCall {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.fn_body)
    }
}

/// procedural macros that can be used wit the following code
/// ```no_run
/// use serde_json::{json, Value};
/// use clightningrpc_plugin_macros::{add_plugin_rpc, rpc_method};
/// use clightningrpc_plugin::commands::RPCCommand;
/// use clightningrpc_plugin::plugin::Plugin;
///
/// #[rpc_method(
///     rpc_name = "foo",
///     description = "This is a simple and short description"
/// )]
/// pub fn foo_rpc(_plugin: Plugin<()>, _request: Value) -> Value {
///     /// The name of the parameters can be used only if used, otherwise can be omitted
///     /// the only rules that the macros require is to have a propriety with the following rules:
///     /// - Plugin as _plugin
///     /// - CLN JSON request as _request
///     /// The function parameter can be specified in any order.
///     json!({"is_dynamic": _plugin.dynamic, "rpc_request": _request})
/// }
/// ```
#[proc_macro_attribute]
pub fn rpc_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse the macros attributes
    let attr_list = parse_macro_input!(attr as AttributeArgs);
    let args = RPCMethodMacro::from_list(&attr_list);
    let macro_args = match args {
        Ok(args) => args,
        Err(err) => panic!("{}", err),
    };

    // parse the item, that in this case need to be a function
    // declaration.
    let ast_item: Item = parse(item.clone()).unwrap();
    let fn_dec = match ast_item {
        Item::Fn(decl) => decl,
        _ => panic!("The macros is applied over a not function declaration"),
    };
    let rpc_call = generate_method_call(&macro_args, fn_dec);
    let res = generate_rpc_method(&item, &rpc_call).parse().unwrap();
    res
}

// helper method to generator the RPCCall struct and make the code more readable and cleaner.
fn generate_method_call(rpc: &RPCMethodMacro, fun_dec: ItemFn) -> RPCCall {
    RPCCall {
        original_name: rpc.rpc_name.to_owned(),
        struct_name: rpc.rpc_name.as_str().to_case(Case::Pascal),
        fn_body: fun_dec.block.into_token_stream().to_string(),
        description: rpc.description.to_string(),
        usage: rpc.usage.to_string(),
    }
}

// helper function to generate the RPC Generator over a generic type
// to make sure that the user can use the plugin state to build the RPC method.
fn generate_rpc_method(item: &TokenStream, method_call: &RPCCall) -> String {
    format!(
        "
    use std::marker::PhantomData;

    #[derive(Clone, Default)]
    struct {}<T> {{
      // keep the information added in the macros to
      // help future macros to register the plugin.
      name: String,
      description: String,
      long_description: String,
      usage: String,
      _phantom: Option<PhantomData<T>>
    }}

   impl<T> {}<T> {{
      pub fn new() -> Self {{
         {}::<T>{{
             name: \"{}\".to_string(),
             description: \"{}\".to_string(),
             long_description: \"{}\".to_string(),
             usage: \"{}\".to_string(),
             _phantom: None,
          }}
      }}

      {}

   }}


    impl<T: Clone + 'static> RPCCommand<T> for {}<T> {{
       fn call<'c>(&self, _plugin: &mut Plugin<T>, _request: &'c Value) -> Result<Value, PluginError> {{
           {}
       }}
    }}
",
        method_call.struct_name,
        method_call.struct_name,
        method_call.struct_name,
        method_call.original_name,
        method_call.description,
        method_call.description,
        method_call.usage,
        item.to_string(),
        method_call.struct_name,
        method_call.to_string(),
    )
    .to_owned()
}

/// procedural macros to generate the code to register a RPC method created with the
/// `rpc_method` procedural macro.
///
/// this procedural macro hide the complicity that the user need to learn to register
/// a rpc method with `rpc_method` and continue to be under magic.
///
/// the macros take in input as first parameter the plugin, and as second the name of the
/// rpc function specified by the user.
///
/// ```no_run
/// use clightningrpc_plugin_macros::{add_plugin_rpc, rpc_method};
/// use serde_json::{json, Value};
///
/// use clightningrpc_plugin::add_rpc;
/// use clightningrpc_plugin::commands::RPCCommand;
/// use clightningrpc_plugin::plugin::Plugin;
///
/// #[rpc_method(
///     rpc_name = "foo",
///     description = "This is a simple and short description"
/// )]
/// pub fn foo_rpc(_plugin: Plugin<()>, _request: Value) -> Value {
///     json!({"is_dynamic": _plugin.dynamic, "rpc_request": _request})
/// }
///
/// fn main() {
///     // as fist step you need to make a new plugin instance
///     // more docs about Plugin struct is provided under the clightning_plugin crate
///     let mut plugin = Plugin::new((), true);
///
///     // The macros helper that help to register a RPC method with the name
///     // without worry about all the rules of the library
///     add_plugin_rpc!(plugin, "foo");
///
///     plugin.start();
/// }
/// ```
#[proc_macro]
pub fn add_plugin_rpc(items: TokenStream) -> TokenStream {
    let input = items.into_iter().collect::<Vec<_>>();
    // FIXME: improve parsing
    assert_eq!(input.len(), 3);
    format!(
        "use clightningrpc_plugin::add_rpc;
    add_rpc!({}, {});",
        input[0],
        input[2]
            .to_string()
            .replace("\"", "")
            .as_str()
            .to_case(Case::Pascal)
    )
    .parse()
    .unwrap()
}

/// The struct where the tools darling store the information
/// the macros regardng the notification, so we implement the struct where we want to unmarshall
/// the user information.
#[derive(FromMeta)]
struct NotificationMethodMacro {
    /// rpc_name is the name of the plugin that the user want to use
    /// regarding the RPC method registered by the plugin.
    on: String,
}

/// RPC notification struct contains all the information to generate a struct
struct RPCNotification {
    /// The name of the notification specified by the user
    original_name: String,
    /// struct name for the command struct
    struct_name: String,
    /// function body defined by the user
    fn_body: String,
}

/// implementation fo the Display method of the rpc method that give
/// that convert the struct in the function call in a valid rust syntax.
impl fmt::Display for RPCNotification {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.fn_body)
    }
}

#[proc_macro_attribute]
pub fn notification(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse the macros attributes
    let attr_list = parse_macro_input!(attr as AttributeArgs);
    let args = NotificationMethodMacro::from_list(&attr_list);
    let macro_args = match args {
        Ok(args) => args,
        Err(err) => panic!("{}", err),
    };

    // parse the item, that in this case need to be a function
    // declaration.
    let ast_item: Item = parse(item.clone()).unwrap();
    let fn_dec = match ast_item {
        Item::Fn(decl) => decl,
        _ => panic!("The macros is applied over a not function declaration"),
    };
    let notification = generate_notification_call(&macro_args, fn_dec);
    generate_notification_method(&item, &notification)
        .parse()
        .unwrap()
}

// helper method to generator the RPCCall struct and make the code more readable and cleaner.
fn generate_notification_call(
    notification: &NotificationMethodMacro,
    fun_dec: ItemFn,
) -> RPCNotification {
    RPCNotification {
        original_name: notification.on.to_owned(),
        // FIXMEL append some suffix
        struct_name: notification.on.as_str().to_case(Case::Pascal),
        fn_body: fun_dec.block.into_token_stream().to_string(),
    }
}

/// helper method to generate the necessary Rust code to implement
fn generate_notification_method(item: &TokenStream, method_call: &RPCNotification) -> String {
    format!(
        "
    #[derive(Clone, Default)]
    struct {}<T> {{
      // keep the information added in the macros to
      // help future macros to register the plugin.
      on_event: String,
      _phantom: Option<PhantomData<T>>
    }}

   impl<T> {}<T> {{
      pub fn new() -> Self {{
         {}::<T>{{
             on_event: \"{}\".to_string(),
             _phantom: None,
          }}
      }}

      {}

   }}


    impl<T: Clone + 'static> RPCCommand<T> for {}<T> {{
       fn call_void<'c>(&self, _plugin: &mut Plugin<T>, _request: &'c Value) {{
           {}
       }}
    }}
",
        method_call.struct_name,
        method_call.struct_name,
        method_call.struct_name,
        method_call.original_name,
        item.to_string(),
        method_call.struct_name,
        method_call.to_string(),
    )
    .to_owned()
}

/// procedural macros to generate the code to register a RPC method created with the
/// `rpc_method` procedural macro.
///
/// this procedural macro hide the complicity that the user need to learn to register
/// a rpc method with `rpc_method` and continue to be under magic.
///
/// the macros take in input as first parameter the plugin, and as second the name of the
/// rpc function specified by the user.
///
/// ```no_run
/// use std::marker::PhantomData;
/// use serde_json::{json, Value};
///
/// use clightningrpc_plugin::commands::RPCCommand;
/// use clightningrpc_plugin::plugin::Plugin;
/// use clightningrpc_plugin::types::LogLevel;
/// use clightningrpc_plugin_macros::{add_plugin_rpc, notification, rpc_method, plugin_register_notification};
///
/// #[notification(on = "rpc_command")]
/// fn on_rpc(_plugin: Plugin<()>, _request: Value) {
///     _plugin.log(LogLevel::Info, "received an RPC notification");
/// }
///
/// fn main() {
///     // as fist step you need to make a new plugin instance
///     // more docs about Plugin struct is provided under the clightning_plugin crate
///     let mut plugin = Plugin::new((), true);
///
///     // plugin_register_notification macros helper that help to register a notification with the
///    // event name without worry about the rules of the library :)
///    plugin_register_notification!(plugin, "rpc_command");
///
///     plugin.start();
/// }
/// ```
#[proc_macro]
pub fn plugin_register_notification(items: TokenStream) -> TokenStream {
    let input = items.into_iter().collect::<Vec<_>>();
    // FIXME: improve parsing
    assert_eq!(input.len(), 3);
    format!(
        "use clightningrpc_plugin::register_notification;
    register_notification!({}, {});",
        input[0],
        input[2]
            .to_string()
            .replace("\"", "")
            .as_str()
            .to_case(Case::Pascal)
    )
    .parse()
    .unwrap()
}
