#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

const RESPONSE: &'static str = include_str!("unions/union_query_response.json");

#[derive(GraphQLQuery)]
#[gql(
    query_path = "tests/unions/union_query.graphql",
    schema_path = "tests/unions/union_schema.graphql"
)]
#[allow(dead_code)]
struct UnionQuery;

#[test]
fn union_query_deserialization() {
    let response_data: union_query::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    println!("{:?}", response_data);

    let expected = r##"ResponseData { names: Some([Person(RustMyQueryNamesOnPerson { first_name: "Audrey", last_name: Some("Lorde") }), Dog(RustMyQueryNamesOnDog { name: "Laïka" }), Organization(RustMyQueryNamesOnOrganization { title: "Mozilla" }), Dog(RustMyQueryNamesOnDog { name: "Norbert" })]) }"##;

    assert_eq!(format!("{:?}", response_data), expected);

    assert_eq!(response_data.names.map(|names| names.len()), Some(4));
}
