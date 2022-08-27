//! plugin error module implementation
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub struct PluginError {
    code: i32,
    #[serde(rename = "message")]
    mgs: String,
    data: Option<serde_json::Value>,
}

impl PluginError {
    #[allow(dead_code)]
    fn new(code: i32, msg: &str, data: &Option<serde_json::Value>) -> Self
    where
        Self: Sized,
    {
        return PluginError {
            code,
            mgs: msg.to_string(),
            data: data.to_owned(),
        };
    }
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code: {}, msg: {}", self.code, self.mgs,)
    }
}
