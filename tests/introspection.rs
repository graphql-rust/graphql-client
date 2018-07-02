#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/introspection/introspection_query.graphql",
    schema_path = "tests/introspection/introspection_schema.graphql"
)]
#[allow(dead_code)]
struct IntrospectionQuery;

#[test]
fn introspection_schema() {
    ()
}
