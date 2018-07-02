#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/input_object_variables/input_object_variables_query.graphql",
    schema_path = "tests/input_object_variables/input_object_variables_schema.graphql"
)]
#[allow(dead_code)]
struct InputObjectVariablesQuery;

#[test]
fn input_object_variables_query_variables_struct() {
    input_object_variables_query::Variables {
        msg: Some(input_object_variables_query::Message {
            content: None,
            to: Some(input_object_variables_query::Recipient {
                email: "sarah.connor@example.com".to_string(),
                category: None,
                name: Some("Sarah Connor".to_string()),
            }),
        }),
    };
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/input_object_variables/input_object_variables_query_defaults.graphql",
    schema_path = "tests/input_object_variables/input_object_variables_schema.graphql"
)]
#[allow(dead_code)]
struct DefaultInputObjectVariablesQuery;

#[test]
fn input_object_variables_default() {
    let variables = default_input_object_variables_query::Variables {
        msg: default_input_object_variables_query::Variables::default_msg(),
    };

    let out = serde_json::to_string(&variables).unwrap();

    assert_eq!(out, r#"{"msg":{"content":null,"to":{"category":null,"email":"rosa.luxemburg@example.com","name":null}}}"#);
}
