#[macro_use]
extern crate graphql_query;
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[derive(GraphQLQuery)]
#[GraphQLQuery(
    query_path = "tests/star_wars_query_1.graphql", schema_path = "tests/star_wars_schema.graphql"
)]
struct StarWarsQuery1;

#[test]
fn star_wars_query_1_variables() {
    // let variables = star_wars_query_1::Variables { character: "Chewbacca" };
    // assert_eq!(StarWarsQuery1::build(&variables), "...");
    unimplemented!();
}

#[test]
fn star_wars_query_1_response() {
    unimplemented!();
}
