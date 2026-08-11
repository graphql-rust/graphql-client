//! The top-level documentation resides on the [project
//! README](https://github.com/graphql-rust/graphql-client) at the moment.
//!
//! The main interface to this library is the custom derive that generates
//! modules from a GraphQL query and schema. See the docs for the
//! [`GraphQLQuery`] trait for a full example.
//!
//! ## Cargo features
//!
//! - `graphql_query_derive` (default: on): enables the `#[derive(GraphqlQuery)]` custom derive.
//! - `reqwest` (default: off): exposes the `graphql_client::reqwest::post_graphql()` function.
//! - `reqwest-blocking` (default: off): exposes the blocking version, `graphql_client::reqwest::post_graphql_blocking()`.

#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

#[cfg(feature = "graphql_query_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate graphql_query_derive;

#[cfg(feature = "graphql_query_derive")]
#[doc(hidden)]
pub use graphql_query_derive::*;

#[cfg(any(
    feature = "reqwest",
    feature = "reqwest-rustls",
    feature = "reqwest-blocking"
))]
pub mod reqwest;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Display, Write};

/// A convenience trait that can be used to build a GraphQL request body.
///
/// This will be implemented for you by codegen in the normal case. It is implemented on the struct you place the derive on.
///
/// Example:
///
/// ```
/// use graphql_client::*;
/// use serde_json::json;
/// use std::error::Error;
///
/// #[derive(GraphQLQuery)]
/// #[graphql(
///   query_path = "../graphql_client_codegen/src/tests/star_wars_query.graphql",
///   schema_path = "../graphql_client_codegen/src/tests/star_wars_schema.graphql"
/// )]
/// struct StarWarsQuery;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     use graphql_client::GraphQLQuery;
///
///     let variables = star_wars_query::Variables {
///         episode_for_hero: star_wars_query::Episode::NEWHOPE,
///     };
///
///     let expected_body = json!({
///         "operationName": star_wars_query::OPERATION_NAME,
///         "query": star_wars_query::QUERY,
///         "variables": {
///             "episodeForHero": "NEWHOPE"
///         },
///     });
///
///     let actual_body = serde_json::to_value(
///         StarWarsQuery::build_query(variables)
///     )?;
///
///     assert_eq!(actual_body, expected_body);
///
///     Ok(())
/// }
/// ```
///
/// # `#[graphql(...)]` attributes
///
/// The `GraphQLQuery` derive is configured with a `#[graphql(...)]`
/// attribute on the struct being derived.
///
/// Paths are relative to the directory containing the package's
/// `Cargo.toml`.
///
/// ```
/// use graphql_client::GraphQLQuery;
///
/// #[derive(GraphQLQuery)]
/// #[graphql(
///     query_path = "../graphql_client_codegen/src/tests/star_wars_query.graphql",
///     schema_path = "../graphql_client_codegen/src/tests/star_wars_schema.graphql",
///     variables_derives = "Debug, PartialEq",
///     response_derives = "Debug, PartialEq",
///     deprecated = "warn",
///     skip_serializing_none,
/// )]
/// struct StarWarsQuery;
/// ```
///
/// ## Required attributes
///
/// ### `query_path`
///
/// `query_path = "path/to/query.graphql"` specifies the GraphQL query
/// document. The document can contain multiple operations. The name of the
/// struct being derived selects the operation with the same name.
///
/// ### `schema_path`
///
/// `schema_path = "path/to/schema.graphql"` specifies the GraphQL schema.
/// Both GraphQL schema language files and JSON introspection responses are
/// supported.
///
/// ## Optional attributes
///
/// ### `variables_derives`
///
/// `variables_derives = "Debug, Clone"` adds comma-separated derive paths
/// to the generated `Variables` struct and generated input types. Generated
/// variable types always derive `serde::Serialize`.
///
/// ### `response_derives`
///
/// `response_derives = "Debug, PartialEq"` adds comma-separated derive paths
/// to the generated response structs and enums. Generated response types
/// always derive `serde::Deserialize`.
///
/// Paths can be used in both derive lists, for example
/// `response_derives = "Debug, std::cmp::PartialOrd"`.
///
/// ### `deprecated`
///
/// `deprecated` controls how fields marked with GraphQL's `@deprecated`
/// directive are generated. It accepts:
///
/// - `"allow"`: generate deprecated fields without a Rust deprecation
///   annotation.
/// - `"warn"`: generate deprecated fields with `#[deprecated]`.
/// - `"deny"`: omit deprecated fields from the generated response types.
///
/// The default is `"warn"`.
///
/// ### `normalization`
///
/// `normalization` controls the naming convention used for generated type,
/// operation, scalar, input, and enum-variant identifiers. It accepts:
///
/// - `"none"`: preserve names from the GraphQL document and schema.
/// - `"rust"`: convert generated identifiers to Rust-style `UpperCamelCase`.
///
/// The default is `"none"`. Rust field names are generated in `snake_case`
/// under either setting.
///
/// ### `custom_scalars_module`
///
/// `custom_scalars_module = "crate::scalars"` specifies a module containing
/// the Rust types for custom GraphQL scalars. The module must define a type
/// with the same generated name as each custom scalar used by the operation.
///
/// Without this attribute, custom scalar types are resolved from the scope
/// containing the struct being derived.
///
/// ### `extern_enums`
///
/// `extern_enums("Direction", "DistanceUnit")` prevents generation of the
/// listed GraphQL enums and uses existing Rust types instead. Each name must
/// exactly match an enum name in the GraphQL schema, and the corresponding
/// Rust type must be accessible from the scope containing the derived struct.
///
/// External enums must implement the serialization or deserialization traits
/// required by how they are used in the operation.
///
/// ### `fragments_other_variant`
///
/// `fragments_other_variant = "true"` adds an `Unknown` variant marked with
/// `#[serde(other)]` to enums generated for union and interface selections.
/// This permits deserializing a `__typename` value not covered by the query's
/// known variants.
///
/// The default is `"false"`.
///
/// ### `skip_serializing_none`
///
/// `skip_serializing_none` is a flag with no value. It adds
/// `#[serde(skip_serializing_if = "Option::is_none")]` to optional fields
/// on generated variable, input-object, and response types. The annotation
/// affects serialization only: absent request values are omitted instead of
/// being serialized as JSON `null`. On response types, it has an effect only
/// when `Serialize` is added through `response_derives`.
///
/// ### `variable_types`
///
/// `variable_types("crate::FirstInput", "crate::SecondInput")` uses existing
/// Rust types for GraphQL input-object variables instead of generating their
/// definitions.
///
/// The paths correspond positionally to the variables declared by the
/// operation. The supplied types must have the shape and serialization
/// behavior required by the GraphQL input types.
///
/// ### `response_type`
///
/// `response_type = "crate::ExistingResponse"` uses an existing Rust type for
/// the value returned by the operation's top-level field instead of generating
/// that field's response type.
///
/// This option requires the operation to select exactly one top-level field.
/// The supplied type must match the selected fields and implement
/// `serde::Deserialize`.
pub trait GraphQLQuery {
    /// The shape of the variables expected by the query. This should be a generated struct most of the time.
    type Variables: serde::Serialize;
    /// The top-level shape of the response data (the `data` field in the GraphQL response). In practice this should be generated, since it is hard to write by hand without error.
    type ResponseData: for<'de> serde::Deserialize<'de>;

    /// Produce a GraphQL query struct that can be JSON serialized and sent to a GraphQL API.
    fn build_query(variables: Self::Variables) -> QueryBody<Self::Variables>;
}

/// The form in which queries are sent over HTTP in most implementations. This will be built using the [`GraphQLQuery`] trait normally.
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryBody<Variables> {
    /// The values for the variables. They must match those declared in the queries. This should be the `Variables` struct from the generated module corresponding to the query.
    pub variables: Variables,
    /// The GraphQL query, as a string.
    pub query: &'static str,
    /// The GraphQL operation name, as a string.
    #[serde(rename = "operationName")]
    pub operation_name: &'static str,
}

/// Represents a location inside a query string. Used in errors. See [`Error`].
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Location {
    /// The line number in the query string where the error originated (starting from 1).
    pub line: i32,
    /// The column number in the query string where the error originated (starting from 1).
    pub column: i32,
}

/// Part of a path in a query. It can be an object key or an array index. See [`Error`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PathFragment {
    /// A key inside an object
    Key(String),
    /// An index inside an array
    Index(i32),
}

impl Display for PathFragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PathFragment::Key(ref key) => write!(f, "{}", key),
            PathFragment::Index(ref idx) => write!(f, "{}", idx),
        }
    }
}

/// An element in the top-level `errors` array of a response body.
///
/// This tries to be as close to the spec as possible.
///
/// [Spec](https://github.com/facebook/graphql/blob/master/spec/Section%207%20--%20Response.md)
///
///
/// ```
/// # use serde_json::json;
/// # use serde::Deserialize;
/// # use graphql_client::GraphQLQuery;
/// # use std::error::Error;
/// #
/// # #[derive(Debug, Deserialize, PartialEq)]
/// # struct ResponseData {
/// #     something: i32
/// # }
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use graphql_client::*;
///
/// let body: Response<ResponseData> = serde_json::from_value(json!({
///     "data": null,
///     "errors": [
///         {
///             "message": "The server crashed. Sorry.",
///             "locations": [{ "line": 1, "column": 1 }]
///         },
///         {
///             "message": "Seismic activity detected",
///             "path": ["underground", 20]
///         },
///      ],
/// }))?;
///
/// let expected: Response<ResponseData> = Response {
///     data: None,
///     errors: Some(vec![
///         Error {
///             message: "The server crashed. Sorry.".to_owned(),
///             locations: Some(vec![
///                 Location {
///                     line: 1,
///                     column: 1,
///                 }
///             ]),
///             path: None,
///             extensions: None,
///         },
///         Error {
///             message: "Seismic activity detected".to_owned(),
///             locations: None,
///             path: Some(vec![
///                 PathFragment::Key("underground".into()),
///                 PathFragment::Index(20),
///             ]),
///             extensions: None,
///         },
///     ]),
///     extensions: None,
/// };
///
/// assert_eq!(body, expected);
///
/// #     Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Error {
    /// The human-readable error message. This is the only required field.
    pub message: String,
    /// Which locations in the query the error applies to.
    pub locations: Option<Vec<Location>>,
    /// Which path in the query the error applies to, e.g. `["users", 0, "email"]`.
    pub path: Option<Vec<PathFragment>>,
    /// Additional errors. Their exact format is defined by the server.
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use `/` as a separator like JSON Pointer.
        let path = self
            .path
            .as_ref()
            .map(|fragments| {
                fragments
                    .iter()
                    .fold(String::new(), |mut acc, item| {
                        let _ = write!(acc, "{}/", item);
                        acc
                    })
                    .trim_end_matches('/')
                    .to_string()
            })
            .unwrap_or_else(|| "<query>".to_string());

        // Get the location of the error. We'll use just the first location for this.
        let loc = self
            .locations
            .as_ref()
            .and_then(|locations| locations.iter().next())
            .cloned()
            .unwrap_or_default();

        write!(f, "{}:{}:{}: {}", path, loc.line, loc.column, self.message)
    }
}

/// The generic shape taken by the responses of GraphQL APIs.
///
/// This will generally be used with the `ResponseData` struct from a derived module.
///
/// [Spec](https://github.com/facebook/graphql/blob/master/spec/Section%207%20--%20Response.md)
///
/// ```
/// # use serde_json::json;
/// # use serde::Deserialize;
/// # use graphql_client::GraphQLQuery;
/// # use std::error::Error;
/// #
/// # #[derive(Debug, Deserialize, PartialEq)]
/// # struct User {
/// #     id: i32,
/// # }
/// #
/// # #[derive(Debug, Deserialize, PartialEq)]
/// # struct Dog {
/// #     name: String
/// # }
/// #
/// # #[derive(Debug, Deserialize, PartialEq)]
/// # struct ResponseData {
/// #     users: Vec<User>,
/// #     dogs: Vec<Dog>,
/// # }
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use graphql_client::Response;
///
/// let body: Response<ResponseData> = serde_json::from_value(json!({
///     "data": {
///         "users": [{"id": 13}],
///         "dogs": [{"name": "Strelka"}],
///     },
///     "errors": [],
/// }))?;
///
/// let expected: Response<ResponseData> = Response {
///     data: Some(ResponseData {
///         users: vec![User { id: 13 }],
///         dogs: vec![Dog { name: "Strelka".to_owned() }],
///     }),
///     errors: Some(vec![]),
///     extensions: None,
/// };
///
/// assert_eq!(body, expected);
///
/// #     Ok(())
/// # }
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Response<Data> {
    /// The absent, partial or complete response data.
    pub data: Option<Data>,
    /// The top-level errors returned by the server.
    pub errors: Option<Vec<Error>>,
    /// Additional extensions. Their exact format is defined by the server.
    /// See [GraphQL Response Specification](https://github.com/graphql/graphql-spec/blob/main/spec/Section%207%20--%20Response.md#response-format)
    pub extensions: Option<HashMap<String, serde_json::Value>>,
}

/// Hidden module for types used by the codegen crate.
#[doc(hidden)]
pub mod _private {
    pub use ::serde;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn graphql_error_works_with_just_message() {
        let err = json!({
            "message": "I accidentally your whole query"
        });

        let deserialized_error: Error = serde_json::from_value(err).unwrap();

        assert_eq!(
            deserialized_error,
            Error {
                message: "I accidentally your whole query".to_string(),
                locations: None,
                path: None,
                extensions: None,
            }
        )
    }

    #[test]
    fn full_graphql_error_deserialization() {
        let err = json!({
            "message": "I accidentally your whole query",
            "locations": [{ "line": 3, "column": 13}, {"line": 56, "column": 1}],
            "path": ["home", "alone", 3, "rating"]
        });

        let deserialized_error: Error = serde_json::from_value(err).unwrap();

        assert_eq!(
            deserialized_error,
            Error {
                message: "I accidentally your whole query".to_string(),
                locations: Some(vec![
                    Location {
                        line: 3,
                        column: 13,
                    },
                    Location {
                        line: 56,
                        column: 1,
                    },
                ]),
                path: Some(vec![
                    PathFragment::Key("home".to_owned()),
                    PathFragment::Key("alone".to_owned()),
                    PathFragment::Index(3),
                    PathFragment::Key("rating".to_owned()),
                ]),
                extensions: None,
            }
        )
    }

    #[test]
    fn full_graphql_error_with_extensions_deserialization() {
        let err = json!({
            "message": "I accidentally your whole query",
            "locations": [{ "line": 3, "column": 13}, {"line": 56, "column": 1}],
            "path": ["home", "alone", 3, "rating"],
            "extensions": {
                "code": "CAN_NOT_FETCH_BY_ID",
                "timestamp": "Fri Feb 9 14:33:09 UTC 2018"
            }
        });

        let deserialized_error: Error = serde_json::from_value(err).unwrap();

        let mut expected_extensions = HashMap::new();
        expected_extensions.insert("code".to_owned(), json!("CAN_NOT_FETCH_BY_ID"));
        expected_extensions.insert("timestamp".to_owned(), json!("Fri Feb 9 14:33:09 UTC 2018"));
        let expected_extensions = Some(expected_extensions);

        assert_eq!(
            deserialized_error,
            Error {
                message: "I accidentally your whole query".to_string(),
                locations: Some(vec![
                    Location {
                        line: 3,
                        column: 13,
                    },
                    Location {
                        line: 56,
                        column: 1,
                    },
                ]),
                path: Some(vec![
                    PathFragment::Key("home".to_owned()),
                    PathFragment::Key("alone".to_owned()),
                    PathFragment::Index(3),
                    PathFragment::Key("rating".to_owned()),
                ]),
                extensions: expected_extensions,
            }
        )
    }
}
