use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/default/query.graphql",
    schema_path = "tests/default/schema.graphql",
    variables_derives = "Default"
)]
struct OptQuery;

#[test]
fn variables_can_derive_default() {
    let _: <OptQuery as GraphQLQuery>::Variables = Default::default();
}
