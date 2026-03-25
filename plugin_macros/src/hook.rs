//! Crate to implement the hook proc macro.
use std::fmt::{Display, Error, Formatter};

use convert_case::{Case, Casing};

use kproc_parser::kparser::{DummyTracer, KParserTracer};
use kproc_parser::kproc_macros::KTokenStream;
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::ast_nodes::{MethodDeclToken, TyToken};
use kproc_parser::rust::kparser::RustParser;
use kproc_parser::trace;

use crate::attr_parser::AttributeParser;

/// Struct holding the parsed hook method information used for code generation.
struct HookCall {
    /// The hook name as registered with CLN (e.g. "htlc_accepted").
    original_name: String,
    /// The name of the generated struct in PascalCase.
    struct_name: String,
    /// The original function name.
    fn_name: String,
    /// The function parameters token stream.
    fn_params: TokenStream,
    /// The return type of the function.
    return_ty: TyToken,
    /// The function body token stream.
    fn_body: TokenStream,
    /// The plugin state type extracted from the first parameter.
    state_ty: String,
    /// Optional list of plugins that should run before this hook.
    before: Option<Vec<String>>,
    /// Optional list of plugins that should run after this hook.
    after: Option<Vec<String>>,
}

impl Display for HookCall {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        write!(formatter, "{}", self.fn_body)
    }
}

/// Parsed macro attributes for the hook.
struct HookMethodMacro {
    /// The hook name to register with CLN.
    hook_name: String,
    /// Optional list of plugins that should run before this hook.
    before: Option<Vec<String>>,
    /// Optional list of plugins that should run after this hook.
    after: Option<Vec<String>>,
}

/// Core parse function that takes the attribute and item TokenStreams
/// and returns the generated code.
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

    let before = attrs.get("before").map(|s| parse_list(s));
    let after = attrs.get("after").map(|s| parse_list(s));

    let attr = HookMethodMacro {
        hook_name: attrs.get("hook_name").unwrap().to_owned(),
        before,
        after,
    };
    let meta = generate_hook_call(attr, fun_ast);
    generate_hook_method(meta, &tracer)
}

/// Parse a comma-separated string list into a Vec<String>.
fn parse_list(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Build the HookCall struct from parsed macro attributes and the function AST.
fn generate_hook_call(hook: HookMethodMacro, fun_dec: MethodDeclToken) -> HookCall {
    let Some((_, ty)) = fun_dec.params.first() else {
        panic!("TODO: we need to return an error, but for now the list of params is empty");
    };
    let Some(ty) = ty.generics.clone().and_then(|gen| gen.first().cloned()) else {
        panic!("TODO: we should return an error, but for now the inner ty has no generics");
    };
    HookCall {
        original_name: hook.hook_name.to_owned(),
        struct_name: format!("Hook{}", hook.hook_name.as_str().to_case(Case::Pascal)),
        fn_name: fun_dec.ident.to_string(),
        fn_params: fun_dec.raw_params,
        return_ty: fun_dec.return_ty.unwrap(),
        fn_body: fun_dec.raw_body.unwrap(),
        state_ty: ty.to_string(),
        before: hook.before,
        after: hook.after,
    }
}

/// Generate the Rust code for the hook struct and its RPCCommand implementation.
fn generate_hook_method(method_call: HookCall, tracer: &dyn KParserTracer) -> TokenStream {
    let struct_name = &method_call.struct_name;
    let hook_name = &method_call.original_name;
    let fn_params = &method_call.fn_params;
    let fn_body = &method_call.fn_body;
    let return_ty = &method_call.return_ty;
    let state_ty = &method_call.state_ty;
    let fn_name = &method_call.fn_name;

    let before_expr = match &method_call.before {
        Some(list) => {
            let items: Vec<String> = list.iter().map(|s| format!("\"{s}\".to_string()")).collect();
            format!("Some(vec![{}])", items.join(", "))
        }
        None => "None".to_string(),
    };

    let after_expr = match &method_call.after {
        Some(list) => {
            let items: Vec<String> = list.iter().map(|s| format!("\"{s}\".to_string()")).collect();
            format!("Some(vec![{}])", items.join(", "))
        }
        None => "None".to_string(),
    };

    let code = format!(
        "
    #[derive(Clone, Default)]
    struct {struct_name} {{
      pub hook_name: String,
      pub before: Option<Vec<String>>,
      pub after: Option<Vec<String>>,
    }}

   impl {struct_name} {{
      pub fn new() -> Self {{
         Self{{
             hook_name: \"{hook_name}\".to_string(),
             before: {before_expr},
             after: {after_expr},
          }}
      }}
   }}

    impl RPCCommand<{state_ty}> for {struct_name} {{
       fn call<'c>(&self, {fn_params}) -> {return_ty} {{
           {fn_body}
       }}
    }}

   fn {fn_name}() -> {struct_name} {{
      {struct_name}::new()
   }}
"
    );

    trace!(tracer, "hook_method: {code}");

    code.parse().unwrap()
}
