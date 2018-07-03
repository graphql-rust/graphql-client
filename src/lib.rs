///! The top-level documentation resides on the [project README](https://github.com/tomhoule/graphql-client) at the moment.
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate graphql_query_derive;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLError {
    pub path: String,
}

/// The generic shape taken by the responses of GraphQL APIs.
///
/// This will generally be used with the `ResponseData` struct from a derived module.
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLResponse<Data> {
    pub data: Option<Data>,
    pub errors: Option<Vec<GraphQLError>>,
}
