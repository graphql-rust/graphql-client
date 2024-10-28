//! Helpers for overriding default serde implementations.

use serde::{Deserialize, Deserializer};

/// Deserialize an optional ID type from either a String or an Integer representation.
///
/// This is used by the codegen to enable String IDs to be deserialized from
/// either Strings or Integers.
pub fn deserialize_option_id<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum IntOrString {
        Int(i64),
        Str(String),
    }

    let res = Option::<IntOrString>::deserialize(deserializer)?;

    Ok(match res {
        None => None,
        Some(IntOrString::Int(n)) => Some(n.to_string()),
        Some(IntOrString::Str(s)) => Some(s),
    })
}
