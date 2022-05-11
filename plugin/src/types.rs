//! types
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
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
}
