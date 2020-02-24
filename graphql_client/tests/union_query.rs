use graphql_client::*;

const RESPONSE: &str = include_str!("unions/union_query_response.json");

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/unions/union_query.graphql",
    schema_path = "tests/unions/union_schema.graphql",
    response_derives = "PartialEq, Debug"
)]
pub struct UnionQuery;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/unions/union_query.graphql",
    schema_path = "tests/unions/union_schema.graphql",
    response_derives = "PartialEq, Debug"
)]
pub struct FragmentOnUnion;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/unions/union_query.graphql",
    schema_path = "tests/unions/union_schema.graphql",
    response_derives = "PartialEq, Debug"
)]
pub struct FragmentAndMoreOnUnion;

#[test]
fn union_query_deserialization() {
    let response_data: union_query::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    let expected = union_query::ResponseData {
        names: Some(vec![
            union_query::UnionQueryNames {
                typename: "Person".into(),
                on: union_query::UnionQueryNamesOn::Person(union_query::UnionQueryNamesOnPerson {
                    first_name: "Audrey".to_string(),
                    last_name: Some("Lorde".to_string()),
                }),
            },
            union_query::UnionQueryNames {
                typename: "Dog".into(),
                on: union_query::UnionQueryNamesOn::Dog(union_query::UnionQueryNamesOnDog {
                    name: "Laïka".to_string(),
                }),
            },
            union_query::UnionQueryNames {
                typename: "Organization".into(),
                on: union_query::UnionQueryNamesOn::Organization(
                    union_query::UnionQueryNamesOnOrganization {
                        title: "Mozilla".to_string(),
                    },
                ),
            },
            union_query::UnionQueryNames {
                typename: "Dog".into(),
                on: union_query::UnionQueryNamesOn::Dog(union_query::UnionQueryNamesOnDog {
                    name: "Norbert".to_string(),
                }),
            },
        ]),
    };

    assert_eq!(response_data, expected);

    assert_eq!(response_data.names.map(|names| names.len()), Some(4));
}

#[test]
fn fragment_on_union() {
    let response_data: fragment_on_union::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    let expected = fragment_on_union::ResponseData {
        names: Some(vec![
            fragment_on_union::FragmentOnUnionNames {
                names_fragment: fragment_on_union::NamesFragment::Person(
                    fragment_on_union::NamesFragmentOnPerson {
                        first_name: "Audrey".to_string(),
                    },
                ),
            },
            fragment_on_union::FragmentOnUnionNames {
                names_fragment: fragment_on_union::NamesFragment::Dog(
                    fragment_on_union::NamesFragmentOnDog {
                        name: "Laïka".to_string(),
                    },
                ),
            },
            fragment_on_union::FragmentOnUnionNames {
                names_fragment: fragment_on_union::NamesFragment::Organization(
                    fragment_on_union::NamesFragmentOnOrganization {
                        title: "Mozilla".to_string(),
                    },
                ),
            },
            fragment_on_union::FragmentOnUnionNames {
                names_fragment: fragment_on_union::NamesFragment::Dog(
                    fragment_on_union::NamesFragmentOnDog {
                        name: "Norbert".to_string(),
                    },
                ),
            },
        ]),
    };

    assert_eq!(response_data, expected);
}

#[test]
fn fragment_and_more_on_union() {
    todo!();
    // let _expected = fragment_and_more_on_union::ResponseData {
    //     names: Some(vec![
    //         fragment_and_more_on_union::FragmentAndMoreOnUnionNames::Person {
    //             first_name: "Audrey".to_string(),
    //             last_name: Some("Lorde".to_string()),
    //         },
    //         fragment_and_more_on_union::FragmentAndMoreOnUnionNames::Dog {
    //             name: "Laïka".to_string(),
    //         },
    //         fragment_and_more_on_union::FragmentAndMoreOnUnionNames::Organization {
    //             title: "Mozilla".to_string(),
    //         },
    //         fragment_and_more_on_union::FragmentAndMoreOnUnionNames::Dog {
    //             name: "Norbert".to_string(),
    //         },
    //     ]),
    // };
}
