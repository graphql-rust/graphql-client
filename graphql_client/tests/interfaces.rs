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

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/interfaces/interface_not_on_everything_query.graphql",
    schema_path = "tests/interfaces/interface_schema.graphql",
    response_derives = "Debug"
)]
pub struct InterfaceNotOnEverythingQuery;

const RESPONSE_NOT_ON_EVERYTHING: &'static str =
    include_str!("interfaces/interface_response_not_on_everything.json");

#[test]
fn interface_not_on_everything_deserialization() {
    println!("{:?}", RESPONSE);
    let response_data: interface_not_on_everything_query::ResponseData =
        serde_json::from_str(RESPONSE_NOT_ON_EVERYTHING).unwrap();

    println!("{:?}", response_data);

    let expected = r##"ResponseData { everything: Some([MyQueryEverything { name: "Audrey Lorde", on: Person(MyQueryEverythingOnPerson { birthday: Some("1934-02-18") }) }, MyQueryEverything { name: "Laïka", on: Dog }, MyQueryEverything { name: "Mozilla", on: Organization(MyQueryEverythingOnOrganization { industry: OTHER }) }, MyQueryEverything { name: "Norbert", on: Dog }]) }"##;

    assert_eq!(format!("{:?}", response_data), expected);

    assert_eq!(response_data.everything.map(|names| names.len()), Some(4));
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/interfaces/interface_with_fragment_query.graphql",
    schema_path = "tests/interfaces/interface_schema.graphql",
    response_derives = "Debug,PartialEq"
)]
pub struct InterfaceWithFragmentQuery;

const RESPONSE_FRAGMENT: &'static str =
    include_str!("interfaces/interface_with_fragment_response.json");

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
                        on: PublicStatusOn::Dog
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
