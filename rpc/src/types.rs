#![allow(missing_docs)]
//! Common structures between requests and responses

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

/// Sub-structure for route in 'pay', 'getroute' and 'sendpay'
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RouteItem {
    pub id: String,
    pub channel: String,
    pub direction: Option<u64>,
    pub amount_msat: MSat,
    pub delay: i64,
    pub style: Option<String>,
    pub blinding: Option<String>,
    pub enctlv: Option<String>,
}

/// Type-safe millisatoshi wrapper
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MSat(pub u64);

impl Serialize for MSat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

struct MSatVisitor;

impl<'d> de::Visitor<'d> for MSatVisitor {
    type Value = MSat;

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if !s.ends_with("msat") {
            return Err(E::custom("missing msat suffix"));
        }

        let numpart = s
            .get(0..(s.len() - 4))
            .ok_or_else(|| E::custom("missing msat suffix"))?;

        let res = u64::from_str(numpart).map_err(|_| E::custom("not a number"))?;
        Ok(MSat(res))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(MSat(v))
    }

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a string ending with \"msat\" or an unsigned integer"
        )
    }
}

impl<'d> Deserialize<'d> for MSat {
    fn deserialize<D>(deserializer: D) -> Result<MSat, D::Error>
    where
        D: Deserializer<'d>,
    {
        deserializer.deserialize_any(MSatVisitor)
    }
}

impl fmt::Debug for MSat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}msat", self.0)
    }
}

impl fmt::Display for MSat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}msat", self.0)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::types::MSat;

    #[test]
    fn test_msat() {
        let v1: MSat = serde_json::from_value(json!(3)).unwrap();
        let v2: MSat = serde_json::from_value(json!("3msat")).unwrap();
        assert_eq!(v1, v2);
    }
}
