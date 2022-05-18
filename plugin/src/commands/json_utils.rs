use serde::Serialize;
use serde_json::{json, Number};

pub fn init_payload() -> serde_json::Value {
    json!({})
}

pub fn init_success_response(id: u64) -> serde_json::Value {
    json!(
        {
            "id": id,
            "jsonrpc": "2.0",
        }
    )
}

pub fn add_number(payload: &mut serde_json::Value, key: &str, value: i64) {
    payload[key.to_string()] = serde_json::Value::Number(Number::from(value));
}

pub fn add_str(payload: &mut serde_json::Value, key: &str, value: &str) {
    payload[key.to_owned()] = serde_json::Value::String(value.to_owned());
}

pub fn add_bool(payload: &mut serde_json::Value, key: &str, value: bool) {
    payload[key.to_string()] = serde_json::Value::Bool(value);
}

pub fn add_vec<V: Serialize>(payload: &mut serde_json::Value, key: &str, value: Vec<V>) {
    let vec = json!(value);
    payload[key.to_string()] = serde_json::Value::Array(vec.as_array().unwrap().clone());
}
