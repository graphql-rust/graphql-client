use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/skip_serializing_none/schema.graphql",
    query_path = "tests/skip_serializing_none/query.graphql",
    skip_serializing_none
)]
pub struct SkipSerializingNoneMutation;

#[test]
fn skip_serializing_none() {
    use skip_serializing_none_mutation::*;

    let query = SkipSerializingNoneMutation::build_query(Variables {
        param: Some(Param {
            data: Author {
                name: "test".to_owned(),
                id: None,
            },
        }),
    });

    let stringified = serde_json::to_string(&query).expect("SkipSerializingNoneMutation is valid");

    println!("{}", stringified);

    assert!(stringified.contains(r#""data":{"name":"test"}"#));
}
