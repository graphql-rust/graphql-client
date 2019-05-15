use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/deprecation/schema.graphql",
    query_path = "tests/deprecation/query.graphql",
    deprecated = "allow"
)]
pub struct AllowDeprecation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/deprecation/schema.graphql",
    query_path = "tests/deprecation/query.graphql",
    deprecated = "deny"
)]
pub struct DenyDeprecation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/deprecation/schema.graphql",
    query_path = "tests/deprecation/query.graphql",
    deprecated = "warn"
)]
pub struct WarnDeprecation;

#[test]
fn deprecation_allow() {
    // Make any deprecations be a compile error.
    // Under `allow`, even deprecated fields aren't marked as such.
    // Thus this is checking that the deprecated fields exist and aren't marked
    // as deprecated.
    #![deny(deprecated)]
    let _ = allow_deprecation::ResponseData {
        current_user: Some(allow_deprecation::AllowDeprecationCurrentUser {
            id: Some("abcd".to_owned()),
            name: Some("Angela Merkel".to_owned()),
            deprecated_with_reason: Some("foo".to_owned()),
            deprecated_no_reason: Some("bar".to_owned()),
        }),
    };
}

#[test]
fn deprecation_deny() {
    let _ = deny_deprecation::ResponseData {
        current_user: Some(deny_deprecation::DenyDeprecationCurrentUser {
            id: Some("abcd".to_owned()),
            name: Some("Angela Merkel".to_owned()),
            // Notice the deprecated fields are not included here.
            // If they were generated, not using them would be a compile error.
            // Thus this is checking that the depreacted fields are not
            // generated under the `deny` scheme.
        }),
    };
}

#[test]
fn deprecation_warn() {
    #![allow(deprecated)]
    let _ = warn_deprecation::ResponseData {
        current_user: Some(warn_deprecation::WarnDeprecationCurrentUser {
            id: Some("abcd".to_owned()),
            name: Some("Angela Merkel".to_owned()),
            deprecated_with_reason: Some("foo".to_owned()),
            deprecated_no_reason: Some("bar".to_owned()),
        }),
    };
}
