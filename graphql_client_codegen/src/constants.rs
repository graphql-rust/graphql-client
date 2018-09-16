use deprecation::DeprecationStatus;
use field_type::FieldType;
use objects::GqlObjectField;

pub(crate) const TYPENAME_FIELD: &str = "__typename";

pub(crate) fn string_type() -> String {
    "String".to_string()
}

#[cfg(test)]
pub(crate) fn float_type() -> String {
    "Float".to_string()
}

pub(crate) fn typename_field() -> GqlObjectField {
    GqlObjectField {
        description: None,
        name: TYPENAME_FIELD.to_string(),
        /// Non-nullable, see spec:
        /// https://github.com/facebook/graphql/blob/master/spec/Section%204%20--%20Introspection.md
        type_: FieldType::Named(string_type()),
        deprecation: DeprecationStatus::Current,
    }
}

pub(crate) const MULTIPLE_SUBSCRIPTION_FIELDS_ERROR: &str = r##"
Multiple-field queries on the root subscription field are forbidden by the spec.

See: https://github.com/facebook/graphql/blob/master/spec/Section%205%20--%20Validation.md#subscription-operation-definitions
"##;

/// Error message when a selection set is the root of a query.
pub(crate) const SELECTION_SET_AT_ROOT: &str = r#"
Operations in queries must be named.

Instead of this:

{
  user {
    name
    repositories {
      name
      commits
    }
  }
}

Write this:

query UserRepositories {
  user {
    name
    repositories {
      name
      commits
    }
  }
}
"#;
