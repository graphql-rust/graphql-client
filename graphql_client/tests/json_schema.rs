use graphql_client::*;
use serde_json::json;

type Uuid = String;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/json_schema/query.graphql",
    schema_path = "tests/json_schema/schema_1.json",
    response_derives = "Debug,PartialEq"
)]
pub struct WithSchema1;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/json_schema/query_2.graphql",
    schema_path = "tests/json_schema/schema_2.json",
    response_derives = "Debug"
)]
pub struct WithSchema2;

#[test]
fn json_schemas_work_with_and_without_data_field() {
    let response = json!({
        "data": {
            "currentSession": null,
        },
    });

    let schema_1_result: graphql_client::Response<with_schema1::ResponseData> =
        serde_json::from_value(response.clone()).unwrap();
    let schema_2_result: graphql_client::Response<with_schema2::ResponseData> =
        serde_json::from_value(response).unwrap();

    assert_eq!(
        format!("{:?}", schema_1_result),
        format!("{:?}", schema_2_result)
    );
}
