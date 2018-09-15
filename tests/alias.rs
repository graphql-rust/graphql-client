#[macro_use]
extern crate graphql_client;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/alias/query.graphql",
    schema_path = "tests/alias/schema.graphql"
)]
#[allow(dead_code)]
struct AliasQuery;

#[test]
fn alias() {
    let valid_response = json!({
        "alias": "127.0.1.2",
        "outer_alias": {
            "inner_alias": "inner value",
        },
    });

    let _type_name_test = alias_query::RustAliasQueryOuterAlias {
        inner_alias: None,
    };

    let valid_alias =
        serde_json::from_value::<alias_query::ResponseData>(valid_response).unwrap();

    assert_eq!(
        valid_alias.alias.unwrap(),
        "127.0.1.2"
    );
    assert_eq!(
        valid_alias.outer_alias.unwrap().inner_alias.unwrap(),
        "inner value"
    );
}
