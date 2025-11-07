use graphql_client::*;
use serde::Deserialize;

/*
 * Enums under test
 *
 * They rename the fields to use SCREAMING_SNAKE_CASE for deserialization, as it is the standard for GraphQL enums.
 */
#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DistanceUnit {
    Meter,
    Feet,
    SomethingElseWithMultipleWords,
}

/* Queries */

// Minimal setup using extern enum.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/extern_enums/schema.graphql",
    query_path = "tests/extern_enums/single_extern_enum_query.graphql",
    extern_enums("DistanceUnit")
)]
pub struct SingleExternEnumQuery;

// Tests using multiple externally defined enums. Also covers mixing with derived traits and with nullable GraphQL enum values.
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/extern_enums/schema.graphql",
    query_path = "tests/extern_enums/multiple_extern_enums_query.graphql",
    response_derives = "Debug, PartialEq, Eq",
    extern_enums("Direction", "DistanceUnit")
)]
pub struct MultipleExternEnumsQuery;

/* Tests */

#[test]
fn single_extern_enum() {
    const RESPONSE: &str = include_str!("extern_enums/single_extern_enum_response.json");

    println!("{RESPONSE:?}");
    let response_data: single_extern_enum_query::ResponseData =
        serde_json::from_str(RESPONSE).unwrap();

    println!("{:?}", response_data.unit);

    let expected = single_extern_enum_query::ResponseData {
        unit: DistanceUnit::Meter,
    };

    assert_eq!(response_data.unit, expected.unit);
}

#[test]
fn multiple_extern_enums() {
    const RESPONSE: &str = include_str!("extern_enums/multiple_extern_enums_response.json");

    println!("{RESPONSE:?}");
    let response_data: multiple_extern_enums_query::ResponseData =
        serde_json::from_str(RESPONSE).unwrap();

    println!("{response_data:?}");

    let expected = multiple_extern_enums_query::ResponseData {
        distance: 100,
        direction: Some(Direction::North),
        unit: DistanceUnit::SomethingElseWithMultipleWords,
    };

    assert_eq!(response_data, expected);
}
