use graphql_client::*;

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
    const RESPONSE: &str = include_str!("unions/union_query_response.json");

    let response_data: query_on_union::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    let expected = query_on_union::ResponseData {
        names: Some(vec![
            query_on_union::QueryOnUnionNames::Person(query_on_union::QueryOnUnionNamesOnPerson {
                first_name: "Audrey".to_string(),
                last_name: Some("Lorde".to_string()),
            }),
            query_on_union::QueryOnUnionNames::Dog(query_on_union::QueryOnUnionNamesOnDog {
                dog_name: query_on_union::DogName {
                    name: "Laïka".to_string(),
                },
            }),
            query_on_union::QueryOnUnionNames::Organization(
                query_on_union::QueryOnUnionNamesOnOrganization {
                    title: "Mozilla".to_string(),
                },
            ),
            query_on_union::QueryOnUnionNames::Dog(query_on_union::QueryOnUnionNamesOnDog {
                dog_name: query_on_union::DogName {
                    name: "Norbert".to_string(),
                },
            }),
        ]),
    };

    assert_eq!(response_data, expected);
}

#[test]
fn type_refining_fragment_on_interface() {
    use crate::query_on_interface::*;

    const RESPONSE: &str = include_str!("interfaces/interface_response.json");

    let response_data: query_on_interface::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    let expected = ResponseData {
        everything: Some(vec![
            QueryOnInterfaceEverything {
                name: "Audrey Lorde".to_string(),
                on: QueryOnInterfaceEverythingOn::Person(QueryOnInterfaceEverythingOnPerson {
                    birthday_fragment: BirthdayFragment {
                        birthday: Some("1934-02-18".to_string()),
                    },
                }),
            },
            QueryOnInterfaceEverything {
                name: "Laïka".to_string(),
                on: QueryOnInterfaceEverythingOn::Dog(QueryOnInterfaceEverythingOnDog {
                    is_good_dog: true,
                }),
            },
            QueryOnInterfaceEverything {
                name: "Mozilla".to_string(),
                on: QueryOnInterfaceEverythingOn::Organization(
                    QueryOnInterfaceEverythingOnOrganization {
                        industry: Industry::OTHER,
                    },
                ),
            },
            QueryOnInterfaceEverything {
                name: "Norbert".to_string(),
                on: QueryOnInterfaceEverythingOn::Dog(QueryOnInterfaceEverythingOnDog {
                    is_good_dog: true,
                }),
            },
        ]),
    };

    assert_eq!(response_data, expected);
}
