use graphql_client::*;

const RESPONSE: &str = include_str!("interfaces/interface_response.json");

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/interfaces/interface_query.graphql",
    schema_path = "tests/interfaces/interface_schema.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct InterfaceQuery;

#[test]
fn interface_deserialization() {
    use interface_query::*;

    println!("{:?}", RESPONSE);
    let response_data: interface_query::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    println!("{:?}", response_data);

    let expected = ResponseData {
        everything: Some(vec![
            InterfaceQueryEverything {
                name: "Audrey Lorde".to_string(),
                on: InterfaceQueryEverythingOn::Person(InterfaceQueryEverythingOnPerson {
                    birthday: Some("1934-02-18".to_string()),
                }),
            },
            InterfaceQueryEverything {
                name: "Laïka".to_string(),
                on: InterfaceQueryEverythingOn::Dog(InterfaceQueryEverythingOnDog {
                    is_good_dog: true,
                }),
            },
            InterfaceQueryEverything {
                name: "Mozilla".to_string(),
                on: InterfaceQueryEverythingOn::Organization(
                    InterfaceQueryEverythingOnOrganization {
                        industry: Industry::OTHER,
                    },
                ),
            },
            InterfaceQueryEverything {
                name: "Norbert".to_string(),
                on: InterfaceQueryEverythingOn::Dog(InterfaceQueryEverythingOnDog {
                    is_good_dog: true,
                }),
            },
        ]),
    };

    assert_eq!(response_data, expected);
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/interfaces/interface_not_on_everything_query.graphql",
    schema_path = "tests/interfaces/interface_schema.graphql",
    response_derives = "Debug,PartialEq"
)]
pub struct InterfaceNotOnEverythingQuery;

const RESPONSE_NOT_ON_EVERYTHING: &str =
    include_str!("interfaces/interface_response_not_on_everything.json");

#[test]
fn interface_not_on_everything_deserialization() {
    use interface_not_on_everything_query::*;

    let response_data: interface_not_on_everything_query::ResponseData =
        serde_json::from_str(RESPONSE_NOT_ON_EVERYTHING).unwrap();

    let expected = ResponseData {
        everything: Some(vec![
            InterfaceNotOnEverythingQueryEverything {
                name: "Audrey Lorde".to_string(),
                on: InterfaceNotOnEverythingQueryEverythingOn::Person(
                    InterfaceNotOnEverythingQueryEverythingOnPerson {
                        birthday: Some("1934-02-18".to_string()),
                    },
                ),
            },
            InterfaceNotOnEverythingQueryEverything {
                name: "Laïka".to_string(),
                on: InterfaceNotOnEverythingQueryEverythingOn::Dog,
            },
            InterfaceNotOnEverythingQueryEverything {
                name: "Mozilla".to_string(),
                on: InterfaceNotOnEverythingQueryEverythingOn::Organization(
                    InterfaceNotOnEverythingQueryEverythingOnOrganization {
                        industry: Industry::OTHER,
                    },
                ),
            },
            InterfaceNotOnEverythingQueryEverything {
                name: "Norbert".to_string(),
                on: InterfaceNotOnEverythingQueryEverythingOn::Dog,
            },
        ]),
    };

    // let expected = r##"ResponseData { everything: Some([InterfaceQueryEverything { name: "Audrey Lorde", on: Person(InterfaceQueryEverythingOnPerson { birthday: Some("1934-02-18") }) }, InterfaceQueryEverything { name: "Laïka", on: Dog }, InterfaceQueryEverything { name: "Mozilla", on: Organization(InterfaceQueryEverythingOnOrganization { industry: OTHER }) }, InterfaceQueryEverything { name: "Norbert", on: Dog }]) }"##;

    assert_eq!(response_data, expected);

    assert_eq!(response_data.everything.map(|names| names.len()), Some(4));
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/interfaces/interface_with_fragment_query.graphql",
    schema_path = "tests/interfaces/interface_schema.graphql",
    response_derives = "Debug,PartialEq"
)]
pub struct InterfaceWithFragmentQuery;

const RESPONSE_FRAGMENT: &str = include_str!("interfaces/interface_with_fragment_response.json");

#[test]
fn fragment_in_interface() {
    use interface_with_fragment_query::*;
    let response_data: ResponseData =
        serde_json::from_str(RESPONSE_FRAGMENT).expect("RESPONSE_FRAGMENT did not deserialize");

    assert_eq!(
        response_data,
        ResponseData {
            everything: Some(vec![
                InterfaceWithFragmentQueryEverything {
                    name: "Audrey Lorde".to_string(),
                    public_status: PublicStatus {
                        display_name: false,
                        on: PublicStatusOn::Person(PublicStatusOnPerson {
                            birthday: Some("1934-02-18".to_string()),
                            age: Some(84),
                        }),
                    },
                    on: InterfaceWithFragmentQueryEverythingOn::Person(
                        InterfaceWithFragmentQueryEverythingOnPerson {
                            birthday: Some("1934-02-18".to_string()),
                        }
                    )
                },
                InterfaceWithFragmentQueryEverything {
                    name: "Laïka".to_string(),
                    public_status: PublicStatus {
                        display_name: true,
                        on: PublicStatusOn::OTHER
                    },
                    on: InterfaceWithFragmentQueryEverythingOn::Dog(
                        InterfaceWithFragmentQueryEverythingOnDog { is_good_dog: true }
                    )
                },
                InterfaceWithFragmentQueryEverything {
                    name: "Mozilla".to_string(),
                    public_status: PublicStatus {
                        display_name: false,
                        on: PublicStatusOn::Organization(PublicStatusOnOrganization {
                            industry: Industry::CAT_FOOD,
                        })
                    },
                    on: InterfaceWithFragmentQueryEverythingOn::Organization,
                },
                InterfaceWithFragmentQueryEverything {
                    name: "Norbert".to_string(),
                    public_status: PublicStatus {
                        display_name: true,
                        on: PublicStatusOn::Dog
                    },
                    on: InterfaceWithFragmentQueryEverythingOn::Dog(
                        InterfaceWithFragmentQueryEverythingOnDog { is_good_dog: true }
                    ),
                },
            ])
        }
    )
}
