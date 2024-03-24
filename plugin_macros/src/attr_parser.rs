//! A custom parser for the attributes

use std::collections::HashMap;

use kproc_parser::{
    check,
    kparser::{KParserError, KParserTracer},
    kproc_macros::KTokenStream,
    trace,
};

pub(crate) struct AttributeParser {}

impl AttributeParser {
    pub fn parse(
        stream: &mut KTokenStream,
        tracer: &dyn KParserTracer,
    ) -> Result<HashMap<String, String>, KParserError> {
        parse_key_values(stream, tracer)
    }
}

fn parse_key_values(
    stream: &mut KTokenStream,
    tracer: &dyn KParserTracer,
) -> Result<HashMap<String, String>, KParserError> {
    let mut hash_map = HashMap::new();
    trace!(tracer, "start parsing key values");
    trace!(tracer, "start with tok {}", stream.peek());
    while !stream.is_end() {
        let key = stream.advance();
        check!("=", stream.peek())?;
        let _ = stream.advance();
        let value = stream.advance();
        if !stream.is_end() && stream.match_tok(",") {
            trace!(tracer, "removing the `,` tok");
            check!(",", stream.advance())?;
        }
        let value = value.to_string().replace('\"', "");
        trace!(tracer, "key {key} = value {value}");
        hash_map.insert(key.to_string(), value.to_string());
        trace!(tracer, "map is {:?}", hash_map);
    }
    Ok(hash_map)
}
