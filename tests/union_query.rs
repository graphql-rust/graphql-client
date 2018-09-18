#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

const RESPONSE: &'static str = include_str!("unions/union_query_response.json");

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/unions/union_query.graphql",
    schema_path = "tests/unions/union_schema.graphql",
    response_derives = "PartialEq, Debug",
)]
struct UnionQuery;

#[test]
fn union_query_deserialization() {
    let response_data: union_query::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    let expected = union_query::ResponseData {
        names: Some(vec![
            union_query::RustMyQueryNames::Person(union_query::RustMyQueryNamesOnPerson {
                first_name: "Audrey".to_string(),
                last_name: Some("Lorde".to_string()),
            }),
            union_query::RustMyQueryNames::Dog(union_query::RustMyQueryNamesOnDog {
                name: "Laïka".to_string(),
            }),
            union_query::RustMyQueryNames::Organization(
                union_query::RustMyQueryNamesOnOrganization {
                    title: "Mozilla".to_string(),
                },
            ),
            union_query::RustMyQueryNames::Dog(union_query::RustMyQueryNamesOnDog {
                name: "Norbert".to_string(),
            }),
        ]),
    };

    assert_eq!(response_data, expected);

    assert_eq!(response_data.names.map(|names| names.len()), Some(4));
}
