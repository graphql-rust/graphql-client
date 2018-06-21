#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[derive(GraphQLQuery)]
#[GraphQLQuery(
    query_path = "tests/scalar_variables_query.graphql",
    schema_path = "tests/scalar_variables_schema.graphql"
)]
#[allow(dead_code)]
struct ScalarVariablesQuery;

#[test]
fn scalar_variables_query_variables_struct() {
    scalar_variables_query::Variables {
        msg: "hello".to_string(),
        reps: Some(32),
    };
}
