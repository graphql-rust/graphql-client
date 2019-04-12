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

const HEIGHTS_RESPONSE: &str = r##"{"mountainHeight": 224, "buildingHeight": 12}"##;
const ECHO_RESPONSE: &str = r##"{"echo": "tiramisù"}"##;

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

    let expected_echo = echo::ResponseData {
        echo: Some("tiramisù".to_string()),
    };

    let expected_heights = heights::ResponseData {
        mountain_height: Some(224),
        building_height: Some(12),
    };

    assert_eq!(expected_echo, echo_response_data);
    assert_eq!(expected_heights, heights_response_data);
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

    assert_eq!(Echo::build_query(echo_variables).operation_name, "Echo");

    assert_eq!(
        Heights::build_query(height_variables).operation_name,
        "Heights"
    );
}
