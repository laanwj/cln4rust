//! Crate to keep the code of the
//! rpc_method proc macro
use std::fmt::{Display, Error, Formatter};

use convert_case::{Case, Casing};

use kproc_parser::kparser::{DummyTracer, KParserTracer};
use kproc_parser::kproc_macros::KTokenStream;
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::ast_nodes::{MethodDeclToken, TyToken};
use kproc_parser::rust::kparser::RustParser;
use kproc_parser::trace;

use crate::attr_parser::AttributeParser;

/// Method to generate the RPC call in a string format
struct RPCCall {
    /// original name of the rpc method specified
    /// by the user
    original_name: String,
    /// the description of the rpc method
    description: String,
    /// the usage tips that is use by core lightning to give tips to the user
    usage: String,
    /// the name of the struct that will be created by the
    /// macros by write in the camel case the original_name
    struct_name: String,
    /// the function name where the macro
    /// is operating on
    fn_name: String,
    /// the function params of the method specified by the user
    fn_params: TokenStream,
    /// the return type of the function defined.
    return_ty: TyToken,
    /// the function body of the method specified by the user
    /// in the function.
    fn_body: TokenStream,
    /// plugin state defined by the user
    state_ty: String,
}

/// implementation fo the Display method of the rpc method that give
/// that convert the struct in the function call in a valid rust syntax.
impl Display for RPCCall {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        write!(formatter, "{}", self.fn_body)
    }
}

/// The struct where the tools darling store the information
/// the macros, so we implement the struct where we want to unmarshall
/// the user information.
struct RPCMethodMacro {
    /// rpc_name is the name of the plugin that the user want to use
    /// regarding the RPC method registered by the plugin.
    rpc_name: String,
    /// description is the short description that the user want to add
    /// to the rpc method.
    description: String,
    /// usage is some tips to give the user on how t use the rpc method
    usage: String,
    // FIXME: add the long description
}

/// core parse function that take in input the TokenStream and return
/// in output the generated token stream.
pub(crate) fn parse(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tracer = DummyTracer {};
    let parser = RustParser::new();
    let fun_ast = parser.parse_fn(&item);

    trace!(tracer, "attrs: {:?}", attr);
    let mut attr = KTokenStream::new(&attr);
    let parser = AttributeParser::parse(&mut attr, &tracer);
    if let Err(err) = parser {
        err.emit();
        panic!();
    }

    let attrs = parser.unwrap();

    let attr = RPCMethodMacro {
        rpc_name: attrs.get("rpc_name").unwrap().to_owned(),
        description: attrs.get("description").unwrap().to_owned(),
        usage: "".to_owned(),
    };
    let meta = generate_method_call(attr, fun_ast);
    generate_rpc_method(meta, &tracer)
}

/// helper method to generator the RPCCall struct and make the code more readable and cleaner.
fn generate_method_call(rpc: RPCMethodMacro, fun_dec: MethodDeclToken) -> RPCCall {
    let Some((_, ty)) = fun_dec.params.first() else {
        panic!("TODO: we need to return an error, but for now the list of params is empity");
    };
    let Some(ty) = ty.generics.clone().and_then(|gen| gen.first().cloned()) else {
        panic!("TODO: we should return an error, but for now the inner ty has no generics");
    };
    RPCCall {
        original_name: rpc.rpc_name.to_owned(),
        description: rpc.description.to_string(),
        usage: rpc.usage.to_string(),
        // FIXME: I can use the function name istead?
        struct_name: rpc.rpc_name.as_str().to_case(Case::Pascal),
        fn_name: fun_dec.ident.to_string(),
        fn_params: fun_dec.raw_params,
        return_ty: fun_dec.return_ty.unwrap(),
        fn_body: fun_dec.raw_body.unwrap(),
        state_ty: ty.to_string(),
    }
}

/// helper function to generate the RPC Generator over a generic type
/// to make sure that the user can use the plugin state to build the RPC method.
fn generate_rpc_method(method_call: RPCCall, tracer: &dyn KParserTracer) -> TokenStream {
    let struct_name = method_call.struct_name;
    let description = method_call.description;
    let usage = method_call.usage;
    let rpc_name = method_call.original_name;
    let fn_params = method_call.fn_params;
    let fn_body = method_call.fn_body;
    let return_ty = method_call.return_ty;
    let state_ty = method_call.state_ty;
    let fn_name = method_call.fn_name;

    let code = format!(
        "
    #[derive(Clone, Default)]
    struct {struct_name} {{
      // keep the information added in the macros to
      // help future macros to register the plugin.
      pub name: String,
      pub description: String,
      pub long_description: String,
      pub usage: String,
    }}

   impl {struct_name} {{
      pub fn new() -> Self {{
         Self{{
             name: \"{rpc_name}\".to_string(),
             description: \"{description}\".to_string(),
             long_description: \"{description}\".to_string(),
             usage: \"{usage}\".to_string(),
          }}
      }}
   }}


    impl RPCCommand<{state_ty}> for {struct_name} {{
       fn call<'c>(&self, {fn_params}) -> {return_ty} {{
           {fn_body}
       }}
    }}


   /// now the original function will e the builder function
   /// than under the hook call the `new` method of the
   /// struct just defined.
   fn {fn_name}() -> {struct_name} {{
      {struct_name}::new()
   }}
"
    );

    trace!(tracer, "rpc_method: {code}");

    code.parse().unwrap()
}
