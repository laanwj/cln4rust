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

#[derive(FromMeta)]
struct RPCMethodMacro {
    rpc_name: String,
    description: String,
}

/// Method to generate the RPC call in a string
/// format
struct RPCCall {
    original_name: String,
    struct_name: String,
    fn_body: String,
    description: String,
}

impl fmt::Display for RPCCall {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.fn_body)
    }
}

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
    //println!("{}", res);
    res
}

fn generate_method_call(rpc: &RPCMethodMacro, fun_dec: ItemFn) -> RPCCall {
    RPCCall {
        original_name: rpc.rpc_name.to_owned(),
        struct_name: rpc.rpc_name.as_str().to_case(Case::Pascal),
        fn_body: fun_dec.block.into_token_stream().to_string(),
        description: rpc.description.to_string(),
    }
}

// Helper function to generate the RPC Generator over a generic type
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
      _phantom: Option<PhantomData<T>>
    }}

   impl<T> {}<T> {{
      pub fn new() -> Self {{
         {}::<T>{{
             name: \"{}\".to_string(),
             description: \"{}\".to_string(),
             long_description: \"{}\".to_string(),
             _phantom: None,
          }}
      }}

      {}

   }}


    impl<T: Clone + 'static> RPCCommand<T> for {}<T> {{
       fn call<'c>(&self, _plugin: &mut Plugin<T>, _request: &'c Value) -> Value {{
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
        item.to_string(),
        method_call.struct_name,
        method_call.to_string(),
    )
    .to_owned()
}

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
