#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/operation_selection/queries.graphql",
    schema_path = "tests/operation_selection/schema.graphql",
    response_derives = "Debug,PartialEq"
)]
pub struct Heights;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/operation_selection/queries.graphql",
    schema_path = "tests/operation_selection/schema.graphql",
    response_derives = "Debug,PartialEq"
)]
pub struct Echo;

// The default is the first operation so this should be the same as Heights
#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/operation_selection/queries.graphql",
    schema_path = "tests/operation_selection/schema.graphql",
    response_derives = "Debug,PartialEq"
)]
pub struct Unrelated;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/operation_selection/queries.graphql",
    schema_path = "tests/operation_selection/schema.graphql",
    response_derives = "Debug,PartialEq",
    selected_operation = "Echo"
)]
pub struct SelectedOperation;

const HEIGHTS_RESPONSE: &'static str = r##"{"mountainHeight": 224, "buildingHeight": 12}"##;
const ECHO_RESPONSE: &'static str = r##"{"echo": "tiramisù"}"##;

#[test]
fn operation_selection_works() {
    let heights_response_data: heights::ResponseData =
        serde_json::from_str(HEIGHTS_RESPONSE).unwrap();
    let heights_unrelated_response_data: unrelated::ResponseData =
        serde_json::from_str(HEIGHTS_RESPONSE).unwrap();
    let echo_response_data: echo::ResponseData = serde_json::from_str(ECHO_RESPONSE).unwrap();
    let selected_operation_response_data: selected_operation::ResponseData =
        serde_json::from_str(ECHO_RESPONSE).unwrap();

    let _echo_variables = echo::Variables {
        msg: Some("hi".to_string()),
    };

    let _height_variables = heights::Variables {
        building_id: "12".to_string(),
        mountain_name: Some("canigou".to_string()),
    };
    let _unrelated_variables = unrelated::Variables {
        building_id: "12".to_string(),
        mountain_name: Some("canigou".to_string()),
    };

    let _selected_operation_variables = selected_operation::Variables {
        msg: Some("hi".to_string()),
    };

    let expected_echo = echo::ResponseData {
        echo: Some("tiramisù".to_string()),
    };

    let expected_heights = heights::ResponseData {
        mountain_height: Some(224),
        building_height: Some(12),
    };

    let expected_heights_unrelated = unrelated::ResponseData {
        mountain_height: Some(224),
        building_height: Some(12),
    };

    let expected_selected_operation = selected_operation::ResponseData {
        echo: Some("tiramisù".to_string()),
    };

    assert_eq!(expected_echo, echo_response_data);
    assert_eq!(expected_heights, heights_response_data);
    assert_eq!(expected_heights_unrelated, heights_unrelated_response_data);
    assert_eq!(
        expected_selected_operation,
        selected_operation_response_data
    );
}

#[test]
fn operation_name_is_correct() {
    let echo_variables = echo::Variables {
        msg: Some("hi".to_string()),
    };

    let height_variables = heights::Variables {
        building_id: "12".to_string(),
        mountain_name: Some("canigou".to_string()),
    };
    let unrelated_variables = unrelated::Variables {
        building_id: "12".to_string(),
        mountain_name: Some("canigou".to_string()),
    };

    assert_eq!(Echo::build_query(echo_variables).operation_name, "Echo");
    assert_eq!(
        Heights::build_query(height_variables).operation_name,
        "Heights"
    );
    assert_eq!(
        Unrelated::build_query(unrelated_variables).operation_name,
        "Heights"
    );
}
