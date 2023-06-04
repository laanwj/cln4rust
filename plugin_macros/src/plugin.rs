//! Resolve the procedural macro
//! code to declare a new plugin.
use kproc_parser::kparser::{DummyTracer, KParserError, KParserTracer};
use kproc_parser::kproc_macros::KTokenStream;
use kproc_parser::proc_macro::{TokenStream, TokenTree};
use kproc_parser::{build_error, check, trace};

#[derive(Debug)]
pub struct PluginDeclaration {
    pub state: Option<String>,
    pub dynamic: Option<TokenTree>,
    pub notificatios: Option<TokenStream>,
    pub hooks: Option<TokenStream>,
    pub rpc_methods: Option<TokenStream>,
}

impl std::fmt::Display for PluginDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = self.state.clone().unwrap_or("()".to_owned());
        writeln!(f, "{{")?;
        writeln!(
            f,
            "let mut plugin = Plugin::new({}, {});",
            state,
            self.dynamic
                .clone()
                .map_or(String::from("false"), |val| val.to_string())
        )?;
        if let Some(ref inner) = self.notificatios {
            let mut inner = KTokenStream::new(&inner);
            while !inner.is_end() {
                let notification = inner.advance();
                writeln!(f, "let call = {}();", notification)?;
                writeln!(
                    f,
                    "plugin.register_notification(&call.on_event.clone(), call);"
                )?;
                if let Err(err) = check!(",", inner.advance()) {
                    err.emit();
                    return Ok(());
                }
            }
        }
        if let Some(ref inner) = self.rpc_methods {
            let mut inner = KTokenStream::new(&inner);
            while !inner.is_end() {
                let rpc = inner.advance();
                writeln!(f, "let call = {}();", rpc)?;
                writeln!(f, "plugin.add_rpc_method(&call.name.clone(), &call.usage.clone(), &call.description.clone(), call);")?;
                if let Err(err) = check!(",", inner.advance()) {
                    err.emit();
                    return Ok(());
                }
            }
        }

        writeln!(f, "plugin\n }}")
    }
}

impl Default for PluginDeclaration {
    fn default() -> Self {
        Self {
            state: None,
            dynamic: None,
            notificatios: None,
            hooks: None,
            rpc_methods: None,
        }
    }
}

/// proc macro syntax is something like this
///
/// ```ignore
/// let plugin = plugin! {
///   state: State::new(),
///   dynamic: true,
///   notifications: [
///     on_rpc
///   ],
///   methods: [
///      foo_rpc,
///   ],
///   hooks: [],
/// };
/// plugin.start();
/// ```
pub(crate) fn parse(attr: TokenStream) -> TokenStream {
    let tracer = DummyTracer {};
    let module = KModuleParser::new().parse(attr, &tracer);
    if let Err(err) = module {
        err.emit();
        panic!();
    }
    let module = module.unwrap();
    trace!(tracer, "{module}");
    module.to_string().parse().unwrap()
}

/// Module parser able to parse a proc macro syntax
/// inspired from the linux kernel module macro.
pub struct KModuleParser;

impl KModuleParser {
    pub fn new() -> Self {
        KModuleParser
    }

    pub fn parse<L: KParserTracer>(
        &self,
        tokens: TokenStream,
        tracer: &L,
    ) -> Result<PluginDeclaration, KParserError> {
        trace!(tracer, "inputs stream: {tokens}");
        let mut stream = KTokenStream::new(&tokens);
        parse_stream(&mut stream, tracer)
    }
}

fn parse_stream<T: KParserTracer>(
    stream: &mut KTokenStream,
    tracer: &T,
) -> Result<PluginDeclaration, KParserError> {
    let mut dec = PluginDeclaration::default();
    while !stream.is_end() {
        let (key, mut value) = parse_key_value(stream, tracer)?;
        match key.to_string().as_str() {
            "state" => {
                let mut state = String::new();
                while !value.is_end() {
                    state += &value.advance().to_string();
                }
                dec.state = Some(state);
            }
            "dynamic" => dec.dynamic = Some(value.advance()),
            "notification" => {
                let value = value.advance();
                let TokenTree::Group(inner) = value else {
                    return Err(build_error!(value, "should be an array!"));
                };
                dec.notificatios = Some(inner.stream());
            }
            "methods" => {
                let value = value.advance();
                let TokenTree::Group(inner) = value else {
                    return Err(build_error!(value, "should be an array!"));
                };
                dec.rpc_methods = Some(inner.stream());
            }
            "hooks" => {
                let value = value.advance();
                let TokenTree::Group(inner) = value else {
                    return Err(build_error!(value, "should be an array!"));
                };
                dec.hooks = Some(inner.stream());
            }
            _ => {
                return Err(build_error!(
                    stream.peek().clone(),
                    "`{key}` not a plugin item!"
                ))
            }
        }
    }
    Ok(dec)
}
fn parse_key_value<T: KParserTracer>(
    stream: &mut KTokenStream,
    _: &T,
) -> Result<(TokenTree, KTokenStream), KParserError> {
    let key = stream.advance();
    check!(":", stream.advance())?;
    let mut values = String::new();
    while !stream.match_tok(",") {
        let value = stream.advance();
        values += &value.to_string();
    }
    check!(",", stream.advance())?;
    Ok((key, KTokenStream::new(&values.parse().unwrap())))
}
