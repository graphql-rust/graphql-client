#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/interfaces/interface_with_type_refining_fragment_query.graphql",
    schema_path = "tests/interfaces/interface_schema.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct QueryOnInterface;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/unions/type_refining_fragment_on_union_query.graphql",
    schema_path = "tests/unions/union_schema.graphql",
    response_derives = "PartialEq, Debug"
)]
pub struct QueryOnUnion;

#[test]
fn type_refining_fragment_on_union() {
    const RESPONSE: &'static str = include_str!("unions/union_query_response.json");

    let response_data: query_on_union::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    let expected = query_on_union::ResponseData {
        names: Some(vec![
            query_on_union::RustMyQueryNames::Person(query_on_union::RustMyQueryNamesOnPerson {
                first_name: "Audrey".to_string(),
                last_name: Some("Lorde".to_string()),
            }),
            query_on_union::RustMyQueryNames::Dog(query_on_union::RustMyQueryNamesOnDog {
                name: "Laïka".to_string(),
            }),
            query_on_union::RustMyQueryNames::Organization(
                query_on_union::RustMyQueryNamesOnOrganization {
                    title: "Mozilla".to_string(),
                },
            ),
            query_on_union::RustMyQueryNames::Dog(query_on_union::RustMyQueryNamesOnDog {
                name: "Norbert".to_string(),
            }),
        ]),
    };

    assert_eq!(response_data, expected);
}

#[test]
fn type_refining_fragment_on_interface() {
    use query_on_interface::*;

    const RESPONSE: &'static str = include_str!("interfaces/interface_response.json");

    let response_data: query_on_interface::ResponseData = serde_json::from_str(RESPONSE).unwrap();

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
}
