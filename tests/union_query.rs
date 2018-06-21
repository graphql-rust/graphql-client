#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

const RESPONSE: &'static str = include_str!("union_query_response.json");

#[derive(GraphQLQuery)]
#[GraphQLQuery(
    query_path = "tests/union_query.graphql",
    schema_path = "tests/union_schema.graphql",
)]
struct UnionQuery;

#[test]
fn union_query_deserialization() {
    let response_data: union_query::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    println!("{:?}", response_data);

    let expected = r##"ResponseData { names: Some([Some(Person(RustMyQueryNamesOnPerson { firstName: Some("Audrey"), lastName: Some("Lorde") })), Some(Dog(RustMyQueryNamesOnDog { name: Some("Laïka") })), Some(Organization(RustMyQueryNamesOnOrganization { title: Some("Mozilla") })), Some(Dog(RustMyQueryNamesOnDog { name: Some("Norbert") }))]) }"##;

    assert_eq!(format!("{:?}", response_data), expected);

    assert_eq!(response_data.names.map(|names| names.len()), Some(4));

}
