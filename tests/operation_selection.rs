#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/operation_selection/queries.graphql",
    schema_path = "tests/operation_selection/schema.graphql"
)]
#[allow(dead_code)]
struct Heights;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/operation_selection/queries.graphql",
    schema_path = "tests/operation_selection/schema.graphql"
)]
#[allow(dead_code)]
struct Echo;

const HEIGHTS_RESPONSE: &'static str = r##"{"mountainHeight": 224, "buildingHeight": 12}"##;
const ECHO_RESPONSE: &'static str = r##"{"echo": "tiramisù"}"##;

#[test]
fn operation_selection_works() {
    let heights_response_data: heights::ResponseData =
        serde_json::from_str(HEIGHTS_RESPONSE).unwrap();
    let echo_response_data: echo::ResponseData = serde_json::from_str(ECHO_RESPONSE).unwrap();

    let _echo_variables = echo::Variables {
        msg: Some("hi".to_string()),
    };

    let _height_variables = heights::Variables {
        building_id: "12".to_string(),
        mountain_name: Some("canigou".to_string()),
    };

    let expected_echo = r##"ResponseData { echo: Some("tiramisù") }"##;
    let expected_heights =
        r##"ResponseData { mountain_height: Some(224), building_height: Some(12) }"##;

    assert_eq!(expected_echo, format!("{:?}", echo_response_data));
    assert_eq!(expected_heights, format!("{:?}", heights_response_data));
}
