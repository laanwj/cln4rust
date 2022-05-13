//! plugin_macros is a rust crate that provide a sequence of helper
//! function to allow the user of the API to write a plugin
//! with less code.
//!
// author: https://github.com/vincenzopalazzo
use darling::FromMeta;
use proc_macro::TokenStream;
use std::fmt;
use syn::{parse, parse_macro_input, AttributeArgs, Item, ItemFn};

#[derive(FromMeta)]
struct RPCMethodMacro {
    rpc_name: String,
    _description: String,
}

/// Method to generate the RPC call in a string
/// format
struct RPCCall {
    struct_name: String,
    fn_name: String,
}

impl fmt::Display for RPCCall {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}::<T>::{}()", self.struct_name, self.fn_name)
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
    let rpc_call = generate_method_call(&macro_args.rpc_name, fn_dec);
    let res = generate_rpc_method(&macro_args, &item, &rpc_call)
        .parse()
        .unwrap();
    //println!("{}", res);
    res
}

fn generate_method_call(rpc_name: &String, fun_dec: ItemFn) -> RPCCall {
    RPCCall {
        struct_name: rpc_name.to_string(),
        fn_name: fun_dec.sig.ident.to_string(),
    }
}

// Helper function to generate the RPC Generator over a generic type
// to make sure that the user can use the plugin state to build the RPC method.
fn generate_rpc_method(args: &RPCMethodMacro, item: &TokenStream, method_call: &RPCCall) -> String {
    format!(
        "
    use std::marker::PhantomData;

    #[derive(Clone, Default)]
    struct {}<T> {{
      _phantom: Option<PhantomData<T>>
    }}

   impl<T> {}<T> {{
      pub fn new() -> Self {{
         {}::<T>{{
             _phantom: None
         }}
      }}

      {}

   }}


    impl<T: Clone + 'static> RPCMethod<T> for {}<T> {{
       fn call<'c>(&self, _plugin: &mut Plugin<T>, _request: &'c Value) -> Value {{
           {}
       }}
    }}
",
        args.rpc_name,
        args.rpc_name,
        args.rpc_name,
        item.to_string(),
        args.rpc_name,
        method_call.to_string(),
    )
    .to_owned()
}
