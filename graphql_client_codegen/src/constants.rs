pub(crate) const TYPENAME_FIELD: &str = "__typename";

pub(crate) const MULTIPLE_SUBSCRIPTION_FIELDS_ERROR: &str = r"
Multiple-field queries on the root subscription field are forbidden by the spec.

See: https://github.com/facebook/graphql/blob/master/spec/Section%205%20--%20Validation.md#subscription-operation-definitions
";

/// Error message when a selection set is the root of a query.
pub(crate) const SELECTION_SET_AT_ROOT: &str = r"
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
";
