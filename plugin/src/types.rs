//! types
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RpcOption {
    /// option name that is specified by the
    /// core lightning user, like --foo
    pub name: String,
    /// option type, that can be specified like the enum
    #[serde(rename = "type")]
    pub opt_typ: String,
    /// default value, that can be specified only as string
    /// and core lightning will convert it for you :) smart right?
    pub default: Option<String>,
    /// description of the option that is shows to the user
    /// when lightningd --help is typed
    pub description: String,
    /// if the filed is deprecated
    pub deprecated: bool,
    /// The value specified by the user
    pub value: Option<Value>,
}

impl RpcOption {
    pub fn value<T: for<'de> serde::de::Deserialize<'de>>(&self) -> T {
        let value: T = serde_json::from_value(
            self.value
                .to_owned()
                .expect(&format!("RPC value option is not present {:?}", self)),
        )
        .expect("serde json fails to build json::Value");
        value
    }
}

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

#[cfg(feature = "log")]
impl From<log::Level> for LogLevel {
    fn from(value: log::Level) -> Self {
        use log::Level;
        match value {
            Level::Debug => LogLevel::Debug,
            Level::Info => LogLevel::Info,
            Level::Warn => LogLevel::Warn,
            Level::Error => LogLevel::Error,
            Level::Trace => LogLevel::Debug,
        }
    }
}
