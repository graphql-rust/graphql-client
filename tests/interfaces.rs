#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

const RESPONSE: &'static str = include_str!("interfaces/interface_response.json");

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/interfaces/interface_query.graphql",
    schema_path = "tests/interfaces/interface_schema.graphql",
    response_derives = "Debug, PartialEq",
)]
struct InterfaceQuery;

#[test]
fn interface_deserialization() {
    use interface_query::*;

    println!("{:?}", RESPONSE);
    let response_data: interface_query::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    println!("{:?}", response_data);

    let expected = ResponseData {
        everything: Some(vec![
            RustMyQueryEverything {
                name: "Audrey Lorde".to_string(),
                on: RustMyQueryEverythingOn::Person(RustMyQueryEverythingOnPerson {
                    birthday: Some("1934-02-18".to_string()),
                }),
            },
            RustMyQueryEverything {
                name: "Laïka".to_string(),
                on: RustMyQueryEverythingOn::Dog(RustMyQueryEverythingOnDog { is_good_dog: true }),
            },
            RustMyQueryEverything {
                name: "Mozilla".to_string(),
                on: RustMyQueryEverythingOn::Organization(RustMyQueryEverythingOnOrganization {
                    industry: Industry::OTHER,
                }),
            },
            RustMyQueryEverything {
                name: "Norbert".to_string(),
                on: RustMyQueryEverythingOn::Dog(RustMyQueryEverythingOnDog { is_good_dog: true }),
            },
        ]),
    };

    assert_eq!(response_data, expected);

    assert_eq!(response_data.everything.map(|names| names.len()), Some(4));
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/interfaces/interface_not_on_everything_query.graphql",
    schema_path = "tests/interfaces/interface_schema.graphql",
    response_derives = "Debug",
)]
struct InterfaceNotOnEverythingQuery;

const RESPONSE_NOT_ON_EVERYTHING: &'static str =
    include_str!("interfaces/interface_response_not_on_everything.json");

#[test]
fn interface_not_on_everything_deserialization() {
    println!("{:?}", RESPONSE);
    let response_data: interface_not_on_everything_query::ResponseData =
        serde_json::from_str(RESPONSE_NOT_ON_EVERYTHING).unwrap();

    println!("{:?}", response_data);

    let expected = r##"ResponseData { everything: Some([RustMyQueryEverything { name: "Audrey Lorde", on: Person(RustMyQueryEverythingOnPerson { birthday: Some("1934-02-18") }) }, RustMyQueryEverything { name: "Laïka", on: Dog }, RustMyQueryEverything { name: "Mozilla", on: Organization(RustMyQueryEverythingOnOrganization { industry: OTHER }) }, RustMyQueryEverything { name: "Norbert", on: Dog }]) }"##;

    assert_eq!(format!("{:?}", response_data), expected);

    assert_eq!(response_data.everything.map(|names| names.len()), Some(4));
}
