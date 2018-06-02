extern crate failure;
extern crate graphql_parser;
extern crate quote;

trait GraphQLQuery {
    type Variables;
    type ResponseData;

    fn build(variables: &Self::Variables) -> String;
}

pub struct GraphQLError {
    pub path: String,
}

pub struct GraphQLResponse<Data> {
    pub data: Option<Data>,
    pub errors: Option<Vec<GraphQLError>>,
}

/// This will output two things:
///
/// - a function that takes the necessary variables and outputs a JSON graphql query with the proper variables.
/// - a struct representing the shape of the `data` field of the resolved response
///
fn query_to_rs(query: &str, schema: &str) -> Result<quote::Tokens, failure::Error> {
    unimplemented!();
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
