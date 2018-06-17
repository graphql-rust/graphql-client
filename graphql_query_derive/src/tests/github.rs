use graphql_parser;
use schema::Schema;
use serde_json;

const SCHEMA_JSON: &'static str = include_str!("github_schema.json");
const SCHEMA_GRAPHQL: &'static str = include_str!("github_schema.graphql");

#[test]
fn ast_from_graphql_and_json_produce_the_same_schema() {
    let json: ::introspection_response::IntrospectionResponse =
        serde_json::from_str(SCHEMA_JSON).unwrap();
    let graphql_parser_schema = graphql_parser::parse_schema(SCHEMA_GRAPHQL).unwrap();
    assert_eq!(Schema::from(graphql_parser_schema), Schema::from(json),);
}
