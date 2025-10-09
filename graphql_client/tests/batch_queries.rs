use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/operation_selection/queries.graphql",
    schema_path = "tests/operation_selection/schema.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct Echo;

#[test]
fn batch_query() {
    let echo_variables = vec![
        echo::Variables {
            msg: Some("hi".to_string()),
        },
        echo::Variables {
            msg: Some("hello".to_string()),
        },
    ];
    let echo_batch_queries: serde_json::Value =
        serde_json::to_value(Echo::build_batch_query(echo_variables))
            .expect("Failed to serialize the query!");
    assert_eq!(
        echo_batch_queries.to_string(),
        r#"[{"operationName":"Echo","query":"query Heights($buildingId: ID!, $mountainName: String) {\n  mountainHeight(name: $mountainName)\n  buildingHeight(id: $buildingId)\n}\n\nquery Echo($msg: String) {\n  echo(msg: $msg)\n}\n","variables":{"msg":"hi"}},{"operationName":"Echo","query":"query Heights($buildingId: ID!, $mountainName: String) {\n  mountainHeight(name: $mountainName)\n  buildingHeight(id: $buildingId)\n}\n\nquery Echo($msg: String) {\n  echo(msg: $msg)\n}\n","variables":{"msg":"hello"}}]"#
    );
}
