//! The top-level documentation resides on the [project README](https://github.com/tomhoule/graphql-client) at the moment.
//!
//! The main interface to this library is the custom derive that generates modules from a GraphQL query and schema.

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate graphql_query_derive;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

#[doc(hidden)]
pub use graphql_query_derive::*;

/// A convenience trait that can be used to build a GraphQL request body.
pub trait GraphQLQuery<'de> {
    type Variables: serde::Serialize;
    type ResponseData: serde::Deserialize<'de>;

    /// Produce a GraphQL query struct that can be JSON serialized and sent to a GraphQL API.
    fn build_query(variables: Self::Variables) -> GraphQLQueryBody<Self::Variables>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLQueryBody<Variables>
where
    Variables: serde::Serialize,
{
    pub variables: Variables,
    pub query: &'static str,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Location {
    line: i32,
    column: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PathFragment {
    Key(String),
    Index(i32),
}

/// An element in the top-level `errors` array of a response body.
///
/// This tries to be as close to the spec as possible.
///
/// Spec: [https://github.com/facebook/graphql/blob/master/spec/Section%207%20--%20Response.md]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Option<Vec<Location>>,
    pub path: Option<Vec<PathFragment>>,
}

/// The generic shape taken by the responses of GraphQL APIs.
///
/// This will generally be used with the `ResponseData` struct from a derived module.
///
/// Spec: [https://github.com/facebook/graphql/blob/master/spec/Section%207%20--%20Response.md]
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLResponse<Data> {
    pub data: Option<Data>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graphql_error_works_with_just_message() {
        let err = json!({
            "message": "I accidentally your whole query"
        });

        let deserialized_error: GraphQLError = serde_json::from_value(err).unwrap();

        assert_eq!(
            deserialized_error,
            GraphQLError {
                message: "I accidentally your whole query".to_string(),
                locations: None,
                path: None,
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

        let deserialized_error: GraphQLError = serde_json::from_value(err).unwrap();

        assert_eq!(
            deserialized_error,
            GraphQLError {
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
            }
        )
    }
}
