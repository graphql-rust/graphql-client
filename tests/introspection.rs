#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[derive(GraphQLQuery)]
#[GraphQLQuery(
    query_path = "tests/introspection_query.graphql",
    schema_path = "tests/introspection_schema.graphql"
)]
struct IntrospectionQuery;

#[test]
fn introspection_schema() {
    ()
}
