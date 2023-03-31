//! Crate that to implement the notification proc macro.
use std::fmt::{Display, Error, Formatter};
use std::process::abort;

use convert_case::{Case, Casing};

use kproc_parser::kparser::{DummyTracer, KParserTracer};
use kproc_parser::kproc_macros::KTokenStream;
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::ast_nodes::MethodDeclToken;
use kproc_parser::rust::kparser::RustParser;
use kproc_parser::trace;

use crate::attr_parser::AttributeParser;

/// The struct where the tools darling store the information
/// the macros regardng the notification, so we implement the struct where we want to unmarshall
/// the user information.
struct NotificationMethodMacro {
    /// rpc_name is the name of the plugin that the user want to use
    /// regarding the RPC method registered by the plugin.
    on: String,
}

/// RPC notification struct contains all the information to generate a struct
struct RPCNotification {
    /// The name of the notification specified by the user
    original_name: String,
    /// the original function name
    /// where the macro is operating on.
    fn_name: String,
    /// struct name for the command struct
    struct_name: String,
    /// function parameters defined in the user function
    fn_params: TokenStream,
    /// function body defined by the user
    fn_body: TokenStream,
    /// Plugin state defined by the user and pass
    /// as first method parameter.
    state_ty: String,
}

/// implementation fo the Display method of the rpc method that give
/// that convert the struct in the function call in a valid rust syntax.
impl Display for RPCNotification {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        write!(formatter, "{}", self.fn_body)
    }
}

/// Core function to parse token stream and return
/// the generate token stream
pub(crate) fn parse(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tracer = DummyTracer;
    let parser = RustParser::with_tracer(&tracer);
    let fn_ast = parser.parse_fn(&item);

    let mut attr = KTokenStream::new(&attr);
    let parser = AttributeParser::parse(&mut attr, &tracer);
    if let Err(err) = parser {
        err.emit();
        abort();
    }
    let parser = parser.unwrap();
    let attr = NotificationMethodMacro {
        on: parser.get("on").unwrap().to_owned(),
    };
    let meta = generate_notification_call(&attr, fn_ast);
    generate_notification_method(meta, &tracer)
}

// helper method to generator the RPCCall struct and make the code more readable and cleaner.
fn generate_notification_call(
    notification: &NotificationMethodMacro,
    fun_dec: MethodDeclToken,
) -> RPCNotification {
    let struct_name = format!("On{}", notification.on.as_str().to_case(Case::Pascal));
    let Some((_, ty)) = fun_dec.params.first() else {
        panic!("TODO: we need to return an error, but for now the list of params is empty");
    };
    let Some(ty) = ty.generics.clone().and_then(|gen| gen.first().cloned()) else {
        panic!("TODO: we need to return an error , but or now the inner generics is None")
    };
    RPCNotification {
        original_name: notification.on.to_owned(),
        // FIXME: append some suffix
        struct_name,
        fn_name: fun_dec.ident.to_string(),
        fn_params: fun_dec.raw_params,
        fn_body: fun_dec.raw_body.unwrap(),
        state_ty: ty.to_string(),
    }
}

/// helper method to generate the necessary Rust code to implement
fn generate_notification_method(
    method_call: RPCNotification,
    tracer: &dyn KParserTracer,
) -> TokenStream {
    let struct_name = method_call.struct_name;
    let event_name = method_call.original_name;
    let fn_params = method_call.fn_params;
    let fn_body = method_call.fn_body;
    let state_ty = method_call.state_ty;
    let fn_name = method_call.fn_name;
    let result = format!(
        "
    #[derive(Clone, Default)]
    struct {struct_name} {{
      // keep the information added in the macros to
      // help future macros to register the plugin.
      pub on_event: String,
    }}

   impl {struct_name} {{
      pub fn new() -> Self {{
         Self{{
             on_event: \"{event_name}\".to_string(),
          }}
      }}
   }}

   // FIXME: we should have the possibility to take the
   // rag type and put in the `RPCCommand<State>`
   impl RPCCommand<{state_ty}> for {struct_name} {{
       fn call_void<'c>(&self,{fn_params}) {{
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

    trace!(tracer, "notification method callback {result}");

    result.parse().unwrap()
}
