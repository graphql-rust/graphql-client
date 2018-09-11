//! The top-level documentation resides on the [project README](https://github.com/tomhoule/graphql-client) at the moment.
//!
//! The main interface to this library is the custom derive that generates modules from a GraphQL query and schema. See the docs for the [`GraphQLQuery`] trait for a full example.

#![deny(warnings)]
#![deny(missing_docs)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
pub extern crate graphql_query_derive;

#[cfg_attr(test, macro_use)]
extern crate serde_json;

#[doc(hidden)]
pub use graphql_query_derive::*;

use std::collections::HashMap;

/// A convenience trait that can be used to build a GraphQL request body.
///
/// This will be implemented for you by codegen in the normal case. It is implemented on the struct you place the derive on.
///
/// Example:
///
/// ```
/// extern crate failure;
/// #[macro_use]
/// extern crate graphql_client;
/// #[macro_use]
/// extern crate serde_derive;
/// #[macro_use]
/// extern crate serde_json;
/// extern crate serde;
///
/// #[derive(GraphQLQuery)]
/// #[graphql(
///   query_path = "graphql_client_codegen/src/tests/star_wars_query.graphql",
///   schema_path = "graphql_client_codegen/src/tests/star_wars_schema.graphql"
/// )]
/// struct StarWarsQuery;
///
/// fn main() -> Result<(), failure::Error> {
///     use graphql_client::GraphQLQuery;
///
///     let variables = star_wars_query::Variables {
///         episode_for_hero: star_wars_query::Episode::NEWHOPE,
///     };
///
///     let expected_body = json!({
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
pub trait GraphQLQuery<'de> {
    /// The shape of the variables expected by the query. This should be a generated struct most of the time.
    type Variables: serde::Serialize;
    /// The top-level shape of the response data (the `data` field in the GraphQL response). In practice this should be generated, since it is hard to write by hand without error.
    type ResponseData: serde::Deserialize<'de>;

    /// Produce a GraphQL query struct that can be JSON serialized and sent to a GraphQL API.
    fn build_query(variables: Self::Variables) -> QueryBody<Self::Variables>;
}

/// The form in which queries are sent over HTTP in most implementations. This will be built using the [`GraphQLQuery`] trait normally.
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryBody<Variables>
where
    Variables: serde::Serialize,
{
    /// The values for the variables. They must match those declared in the queries. This should be the `Variables` struct from the generated module corresponding to the query.
    pub variables: Variables,
    /// The GraphQL query, as a string.
    pub query: &'static str,
}

/// Represents a location inside a query string. Used in errors. See [`Error`].
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Location {
    /// The line number in the query string where the error originated (starting from 1).
    pub line: i32,
    /// The column number in the query string where the error originated (starting from 1).
    pub column: i32,
}

/// Part of a path in a query. It can be an object key or an array index. See [`Error`].
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PathFragment {
    /// A key inside an object
    Key(String),
    /// An index inside an array
    Index(i32),
}

/// An element in the top-level `errors` array of a response body.
///
/// This tries to be as close to the spec as possible.
///
/// [Spec](https://github.com/facebook/graphql/blob/master/spec/Section%207%20--%20Response.md)
///
///
/// ```
/// # extern crate failure;
/// # #[macro_use]
/// # extern crate serde_json;
/// # extern crate graphql_client;
/// # #[macro_use]
/// # extern crate serde_derive;
/// #
/// # #[derive(Debug, Deserialize, PartialEq)]
/// # struct ResponseData {
/// #     something: i32
/// # }
/// #
/// # fn main() -> Result<(), failure::Error> {
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
///             "path": ["undeground", 20]
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
///                 PathFragment::Key("undeground".into()),
///                 PathFragment::Index(20),
///             ]),
///             extensions: None,
///         },
///     ]),
/// };
///
/// assert_eq!(body, expected);
///
/// #     Ok(())
/// # }
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

/// The generic shape taken by the responses of GraphQL APIs.
///
/// This will generally be used with the `ResponseData` struct from a derived module.
///
/// [Spec](https://github.com/facebook/graphql/blob/master/spec/Section%207%20--%20Response.md)
///
/// ```
/// # extern crate failure;
/// # #[macro_use]
/// # extern crate serde_json;
/// # extern crate graphql_client;
/// # #[macro_use]
/// # extern crate serde_derive;
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
/// # fn main() -> Result<(), failure::Error> {
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
/// };
///
/// assert_eq!(body, expected);
///
/// #     Ok(())
/// # }
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Response<Data> {
    /// The absent, partial or complete response data.
    pub data: Option<Data>,
    /// The top-level errors returned by the server.
    pub errors: Option<Vec<Error>>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
