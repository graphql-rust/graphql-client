use crate::schema::Schema;
// use std::collections::HashSet;
// use std::iter::FromIterator;

const SCHEMA_JSON: &str = include_str!("github_schema.json");
const SCHEMA_GRAPHQL: &str = include_str!("github_schema.graphql");

#[test]
fn ast_from_graphql_and_json_produce_the_same_schema() {
    let json: graphql_introspection_query::introspection_response::IntrospectionResponse =
        serde_json::from_str(SCHEMA_JSON).unwrap();
    let graphql_parser_schema = graphql_parser::parse_schema(SCHEMA_GRAPHQL).unwrap();
    let json = Schema::from(json);
    let gql = Schema::from(graphql_parser_schema);

    assert!(vecs_match(&json.stored_scalars, &gql.stored_scalars));
    assert_eq!(
        json.stored_objects.len(),
        gql.stored_objects.len(),
        "Objects count matches."
    );
    assert!(
        vecs_match(&json.stored_objects, &gql.stored_objects),
        format!("{:?}\n{:?}", json.stored_objects, gql.stored_objects)
    );
    // for (json, gql) in json.unions.iter().zip(gql.unions.iter()) {
    //     assert_eq!(json, gql)
    // }
    // for (json, gql) in json.interfaces.iter().zip(gql.interfaces.iter()) {
    //     assert_eq!(json, gql)
    // }
    // assert_eq!(json.interfaces, gql.interfaces);
    // assert_eq!(json.query_type, gql.query_type);
    // assert_eq!(json.mutation_type, gql.mutation_type);
    // assert_eq!(json.subscription_type, gql.subscription_type);
    // for (json, gql) in json.inputs.iter().zip(gql.inputs.iter()) {
    //     assert_eq!(json, gql);
    // }
    // assert_eq!(json.inputs, gql.inputs, "inputs differ");
    // for ((json_name, json_value), (gql_name, gql_value)) in json.enums.iter().zip(gql.enums.iter())
    // {
    //     assert_eq!(json_name, gql_name);
    //     assert_eq!(
    //         HashSet::<&str>::from_iter(json_value.variants.iter().map(|v| v.name)),
    //         HashSet::<&str>::from_iter(gql_value.variants.iter().map(|v| v.name)),
    //     );
    // }
}

fn vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    a.len() == b.len() && a.iter().all(|a| b.iter().any(|b| a == b))
}
