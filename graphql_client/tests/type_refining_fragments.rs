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
            query_on_union::MyQueryNames::Person(query_on_union::MyQueryNamesOnPerson {
                first_name: "Audrey".to_string(),
                last_name: Some("Lorde".to_string()),
            }),
            query_on_union::MyQueryNames::Dog(query_on_union::MyQueryNamesOnDog {
                name: "Laïka".to_string(),
            }),
            query_on_union::MyQueryNames::Organization(
                query_on_union::MyQueryNamesOnOrganization {
                    title: "Mozilla".to_string(),
                },
            ),
            query_on_union::MyQueryNames::Dog(query_on_union::MyQueryNamesOnDog {
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
            MyQueryEverything {
                name: "Audrey Lorde".to_string(),
                on: MyQueryEverythingOn::Person(MyQueryEverythingOnPerson {
                    birthday: Some("1934-02-18".to_string()),
                }),
            },
            MyQueryEverything {
                name: "Laïka".to_string(),
                on: MyQueryEverythingOn::Dog(MyQueryEverythingOnDog { is_good_dog: true }),
            },
            MyQueryEverything {
                name: "Mozilla".to_string(),
                on: MyQueryEverythingOn::Organization(MyQueryEverythingOnOrganization {
                    industry: Industry::OTHER,
                }),
            },
            MyQueryEverything {
                name: "Norbert".to_string(),
                on: MyQueryEverythingOn::Dog(MyQueryEverythingOnDog { is_good_dog: true }),
            },
        ]),
    };

    assert_eq!(response_data, expected);
}
