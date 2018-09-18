#[macro_use]
extern crate graphql_client;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/fragments/query.graphql",
    schema_path = "tests/fragments/schema.graphql"
)]
#[allow(dead_code)]
struct FragmentReference;

#[test]
fn fragment_reference() {
    let valid_response = json!({
        "inFragment": "value",
    });

    let valid_fragment_reference =
        serde_json::from_value::<fragment_reference::ResponseData>(valid_response).unwrap();

    assert_eq!(
        valid_fragment_reference.fragment_reference.in_fragment.unwrap(),
        "value"
    );
}
