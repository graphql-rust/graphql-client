extern crate failure;
extern crate graphql_parser;
extern crate quote;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate graphql_query_derive;
#[doc(hidden)]
pub use graphql_query_derive::*;

pub trait GraphQLQuery<'de> {
    type Variables: serde::Serialize;
    type ResponseData: serde::Deserialize<'de>;

    /// Produce a GraphQL query struct that can be JSON serialized and sent to a GraphQL API.
    fn build_query(variables: Self::Variables) -> GraphQLQueryBody<Self::Variables>;
}

#[derive(Serialize)]
pub struct GraphQLQueryBody<Variables>
where
    Variables: serde::Serialize,
{
    variables: Variables,
    query: &'static str,
}

pub struct GraphQLError {
    pub path: String,
}

pub struct GraphQLResponse<Data> {
    pub data: Option<Data>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[cfg(test)]
mod tests {

    macro_rules! assert_parses_to {
        ($query:expr, $schema:expr => $expected:tt) => {
            unimplemented!()
        };
    }

    macro_rules! assert_mismatch {
        ($query:expr, $schema:expr) => {
            unimplemented!()
        };
    }

    #[test]
    fn queries_parse_properly() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn invalid_queries_are_rejected() {
        unimplemented!();
    }
}
