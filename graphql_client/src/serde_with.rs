//! Helpers for overriding default serde implementations.

use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(untagged)]
enum IntOrString {
    Int(i64),
    Str(String),
}

impl From<IntOrString> for String {
    fn from(value: IntOrString) -> Self {
        match value {
            IntOrString::Int(n) => n.to_string(),
            IntOrString::Str(s) => s,
        }
    }
}

/// Deserialize an optional ID type from either a String or an Integer representation.
///
/// This is used by the codegen to enable String IDs to be deserialized from
/// either Strings or Integers.
pub fn deserialize_option_id<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<IntOrString>::deserialize(deserializer).map(|opt| opt.map(String::from))
}

/// Deserialize an ID type from either a String or an Integer representation.
///
/// This is used by the codegen to enable String IDs to be deserialized from
/// either Strings or Integers.
pub fn deserialize_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    IntOrString::deserialize(deserializer).map(String::from)
}
