//! This file provides utility functions to handle JSON metadata.
//!
//! author: https://github.com/vincenzopalazzo
use serde::Serialize;
use serde_json::{json, Number};

/// return a server::json_value useful to initialize the JSON payload as an empty JSON object.
pub fn init_payload() -> serde_json::Value {
    json!({})
}

/// returns a success response.
pub fn init_success_response(id: &serde_json::Value) -> serde_json::Value {
    json!(
        {
            "id": id,
            "jsonrpc": "2.0",
        }
    )
}

/// adds an 64-bit integral value to the JSON payload.
pub fn add_number(payload: &mut serde_json::Value, key: &str, value: i64) {
    payload[key.to_string()] = serde_json::Value::Number(Number::from(value));
}

/// adds a string value to the JSON payload.
pub fn add_str(payload: &mut serde_json::Value, key: &str, value: &str) {
    payload[key.to_owned()] = serde_json::Value::String(value.to_owned());
}

/// This function adds a boolean value to the JSON payload.
pub fn add_bool(payload: &mut serde_json::Value, key: &str, value: bool) {
    payload[key.to_string()] = serde_json::Value::Bool(value);
}

/// This function adds a vector to the JSON payload.
pub fn add_vec<V: Serialize>(payload: &mut serde_json::Value, key: &str, value: Vec<V>) {
    let vec = json!(value);
    payload[key.to_string()] = serde_json::Value::Array(vec.as_array().unwrap().clone());
}
