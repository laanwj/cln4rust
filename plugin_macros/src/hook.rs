//! Hook proc macro implementation
//!
//! Author: Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>
use std::process::abort;

use convert_case::{Case, Casing};

use kproc_parser::kparser::{DummyTracer, KParserTracer, Result};
use kproc_parser::kproc_macros::KTokenStream;
use kproc_parser::proc_macro::TokenStream;
use kproc_parser::rust::ast_nodes::MethodDeclToken;
use kproc_parser::rust::kparser::RustParser;

use crate::attr_parser::AttributeParser;

struct RPCHook {
    original_name: String,
    struct_name: String,
    fn_name: String,
}

struct HookMethodMacro {
    on: String,
}

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
    let hook = HookMethodMacro {
        on: parser.get("on").unwrap().to_owned(),
    };
    let meta = generate_hook_call(hook, fn_ast)
        .map_err(|err| {
            err.emit();
            abort();
        })
        .unwrap();
    generate_hook_method(meta, &tracer)
}

fn generate_hook_call(hook: HookMethodMacro, fun_dec: MethodDeclToken) -> Result<RPCHook> {
    let struct_name = format!("On{}", hook.on.as_str().to_case(Case::Pascal));
    let Some((_, ty)) = fun_dec.params.first() else {
        panic!("TODO: we need to return an error, but for now the list of params is empty");
    };
    let Some(ty) = ty.generics.clone().and_then(|gen| gen.first().cloned()) else {
        panic!("TODO: we need to return an error , but or now the inner generics is None")
    };
    Ok(RPCHook {
        original_name: hook.on,
        struct_name,
        fn_name: fun_dec.ident.to_string(),
    })
}

fn generate_hook_method<T: KParserTracer>(method_call: RPCHook, tracer: &T) -> TokenStream {
    "".parse().unwrap()
}
