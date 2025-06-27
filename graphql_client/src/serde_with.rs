//! Helpers for overriding default serde implementations.

use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::Deserialize;
use std::fmt;
use std::marker::PhantomData;

/// Our own visitor trait that allows us to deserialize GraphQL IDs.
///
/// This is used by the codegen to enable String IDs to be deserialized from
/// either Strings or Integers even if they are nested in a list or optional.
///
/// We can't use the Visitor since we want to override the default deserialization
/// behavior for base types and automatic nesting support.
pub trait GraphQLVisitor<'de>: Sized {
    /// The name of the type that we are deserializing.
    fn type_name() -> &'static str;

    /// Visit an integer
    fn visit_i64<E>(v: i64) -> Result<Self, E>
    where
        E: de::Error,
    {
        Err(de::Error::invalid_type(
            de::Unexpected::Signed(v),
            &Self::type_name(),
        ))
    }

    /// Visit an integer
    fn visit_u64<E>(v: u64) -> Result<Self, E>
    where
        E: de::Error,
    {
        Err(de::Error::invalid_type(
            de::Unexpected::Unsigned(v),
            &Self::type_name(),
        ))
    }

    /// Visit a borrowed string
    fn visit_str<E>(v: &str) -> Result<Self, E>
    where
        E: de::Error,
    {
        Err(de::Error::invalid_type(
            de::Unexpected::Str(v),
            &Self::type_name(),
        ))
    }

    /// Visit a string
    fn visit_string<E>(v: String) -> Result<Self, E>
    where
        E: de::Error,
    {
        Err(de::Error::invalid_type(
            de::Unexpected::Str(&v),
            &Self::type_name(),
        ))
    }

    /// Visit a missing optional value
    fn visit_none<E>() -> Result<Self, E>
    where
        E: de::Error,
    {
        Err(de::Error::invalid_type(
            de::Unexpected::Option,
            &Self::type_name(),
        ))
    }

    /// Visit a null value
    fn visit_unit<E>() -> Result<Self, E>
    where
        E: de::Error,
    {
        Err(de::Error::invalid_type(
            de::Unexpected::Unit,
            &Self::type_name(),
        ))
    }

    /// Visit a sequence
    fn visit_seq<A>(seq: A) -> Result<Self, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let _ = seq;
        Err(de::Error::invalid_type(
            de::Unexpected::Seq,
            &Self::type_name(),
        ))
    }
}

impl GraphQLVisitor<'_> for String {
    fn type_name() -> &'static str {
        "an ID"
    }

    fn visit_i64<E>(v: i64) -> Result<Self, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_u64<E>(v: u64) -> Result<Self, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_str<E>(v: &str) -> Result<Self, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_string<E>(v: String) -> Result<Self, E>
    where
        E: de::Error,
    {
        Ok(v)
    }
}

impl<'de, T: GraphQLVisitor<'de>> GraphQLVisitor<'de> for Option<T> {
    fn type_name() -> &'static str {
        "an optional ID or sequence of IDs"
    }

    fn visit_i64<E>(v: i64) -> Result<Self, E>
    where
        E: de::Error,
    {
        T::visit_i64(v).map(Some)
    }

    fn visit_u64<E>(v: u64) -> Result<Self, E>
    where
        E: de::Error,
    {
        T::visit_u64(v).map(Some)
    }

    fn visit_str<E>(v: &str) -> Result<Self, E>
    where
        E: de::Error,
    {
        T::visit_str(v).map(Some)
    }

    fn visit_string<E>(v: String) -> Result<Self, E>
    where
        E: de::Error,
    {
        T::visit_string(v).map(Some)
    }

    fn visit_none<E>() -> Result<Self, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_unit<E>() -> Result<Self, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_seq<A>(seq: A) -> Result<Self, A::Error>
    where
        A: SeqAccess<'de>,
    {
        T::visit_seq(seq).map(Some)
    }
}

impl<'de, T: GraphQLVisitor<'de>> GraphQLVisitor<'de> for Vec<T> {
    fn type_name() -> &'static str {
        "a sequence of IDs"
    }

    fn visit_seq<A>(mut seq: A) -> Result<Self, A::Error>
    where
        A: SeqAccess<'de>,
    {
        struct Id<T>(T);

        impl<'de, T> Deserialize<'de> for Id<T>
        where
            T: GraphQLVisitor<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserialize_id(deserializer).map(Id)
            }
        }

        let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(Id(elem)) = seq.next_element()? {
            vec.push(elem);
        }
        Ok(vec)
    }
}

struct IdVisitor<T> {
    phantom: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for IdVisitor<T>
where
    T: GraphQLVisitor<'de>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string, integer, null, or a sequence of IDs")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::visit_i64(value)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::visit_u64(value)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::visit_str(value)
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::visit_str(&value)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::visit_none()
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::visit_unit()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        T::visit_seq(seq)
    }
}

/// Generic deserializer for GraphQL ID types.
///
/// It can deserialize IDs from a string or an integer.
/// It supports optional values and lists of IDs.
pub fn deserialize_id<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: GraphQLVisitor<'de>,
{
    deserializer.deserialize_any(IdVisitor {
        phantom: PhantomData,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Deserialize)]
    struct Test {
        #[serde(deserialize_with = "deserialize_id")]
        pub id: String,
        #[serde(deserialize_with = "deserialize_id")]
        pub id_opt: Option<String>,
        #[serde(deserialize_with = "deserialize_id")]
        pub id_seq: Vec<String>,
        #[serde(deserialize_with = "deserialize_id")]
        pub id_opt_seq: Option<Vec<String>>,
        #[serde(deserialize_with = "deserialize_id")]
        pub id_opt_seq_opt: Option<Vec<Option<String>>>,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct NestedTest {
        #[serde(deserialize_with = "deserialize_id")]
        pub nested: Vec<Vec<String>>,
        #[serde(deserialize_with = "deserialize_id")]
        pub opt_nested: Option<Vec<Option<Vec<Option<String>>>>>,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct Test2 {
        #[serde(deserialize_with = "deserialize_id")]
        pub id_opt_seq_opt: Option<Vec<Option<String>>>,
    }

    #[test]
    fn test_deserialize_string() {
        let test = serde_json::from_str::<Test>(
            r#"{"id": "123", "id_opt": "123", "id_seq": ["123", "456"], "id_opt_seq": ["123"], "id_opt_seq_opt": ["123", "456"]}"#,
        ).unwrap();
        assert_eq!(test.id, "123".to_string());
        assert_eq!(test.id_opt, Some("123".to_string()));
        assert_eq!(test.id_seq, vec!["123".to_string(), "456".to_string()]);
        assert_eq!(test.id_opt_seq, Some(vec!["123".to_string()]));
        assert_eq!(
            test.id_opt_seq_opt,
            Some(vec![Some("123".to_string()), Some("456".to_string())])
        );
    }

    #[test]
    fn test_deserialize_integer() {
        let test = serde_json::from_str::<Test>(
            r#"{"id": 123, "id_opt": 123, "id_seq": [123, 456], "id_opt_seq": [123], "id_opt_seq_opt": [123, 456]}"#,
        ).unwrap();
        assert_eq!(test.id, "123".to_string());
        assert_eq!(test.id_opt, Some("123".to_string()));
        assert_eq!(test.id_seq, vec!["123".to_string(), "456".to_string()]);
        assert_eq!(test.id_opt_seq, Some(vec!["123".to_string()]));
        assert_eq!(
            test.id_opt_seq_opt,
            Some(vec![Some("123".to_string()), Some("456".to_string())])
        );
    }

    #[test]
    fn test_deserialize_mixed() {
        let test = serde_json::from_str::<Test>(
            r#"{"id": 123, "id_opt": null, "id_seq": [123, "456"], "id_opt_seq": null, "id_opt_seq_opt": [123, null, "456"]}"#,
        )
        .unwrap();
        assert_eq!(test.id, "123".to_string());
        assert_eq!(test.id_opt, None);
        assert_eq!(test.id_seq, vec!["123".to_string(), "456".to_string()]);
        assert_eq!(test.id_opt_seq, None);
        assert_eq!(
            test.id_opt_seq_opt,
            Some(vec![Some("123".to_string()), None, Some("456".to_string())])
        );
    }

    #[test]
    fn test_deserialize_unexpected_list_id_null() {
        let test = serde_json::from_str::<Test>(
            r#"{"id": "123", "id_opt": "123", "id_seq": ["123", null, "456"], "id_opt_seq": null, "id_opt_seq_opt": ["123", null, "456"]}"#,
        )
        .unwrap_err();
        assert_eq!(
            test.to_string(),
            "invalid type: null, expected an ID at line 1 column 53"
        );
    }

    #[test]
    fn test_deserialize_unexpected_list_null() {
        let test = serde_json::from_str::<Test>(
            r#"{"id": "123", "id_opt": "123", "id_seq": null, "id_opt_seq": null, "id_opt_seq_opt": ["123", null, "456"]}"#,
        )
        .unwrap_err();
        assert_eq!(
            test.to_string(),
            "invalid type: null, expected a sequence of IDs at line 1 column 45"
        );
    }

    #[test]
    fn test_deserialize_unexpected_id_null() {
        let test = serde_json::from_str::<Test>(
            r#"{"id": null, "id_opt": "123", "id_seq": null, "id_opt_seq": null, "id_opt_seq_opt": ["123", null, "456"]}"#,
        )
        .unwrap_err();
        assert_eq!(
            test.to_string(),
            "invalid type: null, expected an ID at line 1 column 11"
        );
    }

    #[test]
    fn test_deserialize_nested() {
        let test = serde_json::from_str::<NestedTest>(
            r#"{"nested": [["123", 789, "456"]], "opt_nested": [["123", null, "456"]]}"#,
        )
        .unwrap();
        assert_eq!(
            test.nested,
            vec![vec![
                "123".to_string(),
                "789".to_string(),
                "456".to_string()
            ]]
        );
        assert_eq!(
            test.opt_nested,
            Some(vec![Some(vec![
                Some("123".to_string()),
                None,
                Some("456".to_string())
            ])])
        );
    }
}
