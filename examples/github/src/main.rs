extern crate failure;
#[macro_use]
extern crate graphql_query;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use graphql_query::*;

#[derive(GraphQLQuery)]
#[GraphQLQuery(schema_path = "src/schema.graphql", query_path = "src/query_1.graphql")]
struct Query1;

fn main() -> Result<(), failure::Error> {
    let _q = Query1::build_query(query1::Variables);
    Ok(())
}
