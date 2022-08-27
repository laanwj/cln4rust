//! plugin error module implementation
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
pub struct PluginError {
    code: i32,
    mgs: String,
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "code: {}, msg: {}", self.code, self.mgs)
    }
}
