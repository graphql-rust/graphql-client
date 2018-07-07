extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate graphql_client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../github/src/schema.graphql",
    query_path = "../github/src/query_1.graphql"
)]
pub struct ExampleModule;
