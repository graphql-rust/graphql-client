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
        optional_int: None,
        optional_list: None,
        non_optional_int: 1337,
        non_optional_list: vec![],
        param: Some(Param {
            data: Author {
                name: "test".to_owned(),
                id: None,
            },
        }),
    });

    let stringified = serde_json::to_string(&query).expect("SkipSerializingNoneMutation is valid");

    println!("{}", stringified);

    assert!(stringified.contains(r#""param":{"data":{"name":"test"}}"#));
    assert!(stringified.contains(r#""nonOptionalInt":1337"#));
    assert!(stringified.contains(r#""nonOptionalList":[]"#));
    assert!(!stringified.contains(r#""optionalInt""#));
    assert!(!stringified.contains(r#""optionalList""#));

    let query = SkipSerializingNoneMutation::build_query(Variables {
        optional_int: Some(42),
        optional_list: Some(vec![]),
        non_optional_int: 1337,
        non_optional_list: vec![],
        param: Some(Param {
            data: Author {
                name: "test".to_owned(),
                id: None,
            },
        }),
    });
    let stringified = serde_json::to_string(&query).expect("SkipSerializingNoneMutation is valid");
    println!("{}", stringified);
    assert!(stringified.contains(r#""param":{"data":{"name":"test"}}"#));
    assert!(stringified.contains(r#""nonOptionalInt":1337"#));
    assert!(stringified.contains(r#""nonOptionalList":[]"#));
    assert!(stringified.contains(r#""optionalInt":42"#));
    assert!(stringified.contains(r#""optionalList":[]"#));
}
