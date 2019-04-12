use graphql_client::*;
pub mod custom_scalars;

use custom_scalars::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../github/src/schema.graphql",
    query_path = "../github/src/query_1.graphql",
    response_derives = "Debug"
)]
pub struct ExampleModule;
