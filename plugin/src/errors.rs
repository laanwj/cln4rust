//! plugin error JSONRPCv2.0 compliant module implementation
use std::fmt;

use serde::Serialize;

/// Type defining JSONRPCv2.0 compliant plugin errors defined here
/// https://www.jsonrpc.org/specification#error_object
#[derive(Debug, Clone, Serialize)]
pub struct PluginError {
    code: i32,
    #[serde(rename = "message")]
    msg: String,
    data: Option<serde_json::Value>,
}

impl PluginError {
    #[allow(dead_code)]
    pub fn new(code: i32, msg: &str, data: Option<serde_json::Value>) -> Self
    where
        Self: Sized,
    {
        PluginError {
            code,
            msg: msg.to_string(),
            data,
        }
    }
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code: {}, msg: {}", self.code, self.msg)
    }
}

impl From<serde_json::Error> for PluginError {
    fn from(e: serde_json::Error) -> Self {
        PluginError {
            code: -1,
            msg: format!("{e}"),
            data: None,
        }
    }
}
