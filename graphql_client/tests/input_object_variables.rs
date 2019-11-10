use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/input_object_variables/input_object_variables_query.graphql",
    schema_path = "tests/input_object_variables/input_object_variables_schema.graphql",
    response_derives = "Debug"
)]
pub struct InputObjectVariablesQuery;

#[test]
fn input_object_variables_query_variables_struct() {
    let _ = input_object_variables_query::Variables {
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

// Custom scalars
type Email = String;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/input_object_variables/input_object_variables_query_defaults.graphql",
    schema_path = "tests/input_object_variables/input_object_variables_schema.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct DefaultInputObjectVariablesQuery;

#[test]
fn input_object_variables_default() {
    let variables = default_input_object_variables_query::Variables {
        msg: default_input_object_variables_query::Variables::default_msg(),
    };

    let out = serde_json::to_string(&variables).unwrap();

    assert_eq!(
        out,
        r#"{"msg":{"content":null,"to":{"category":null,"email":"rosa.luxemburg@example.com","name":null}}}"#,
    );
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/input_object_variables/input_object_variables_query.graphql",
    schema_path = "tests/input_object_variables/input_object_variables_schema.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct RecursiveInputQuery;

#[test]
fn recursive_input_objects_can_be_constructed() {
    use recursive_input_query::*;

    let _ = RecursiveInput {
        head: "hello".to_string(),
        tail: Box::new(None),
    };

    let _ = RecursiveInput {
        head: "hi".to_string(),
        tail: Box::new(Some(RecursiveInput {
            head: "this is crazy".to_string(),
            tail: Box::new(None),
        })),
    };
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/input_object_variables/input_object_variables_query.graphql",
    schema_path = "tests/input_object_variables/input_object_variables_schema.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct IndirectlyRecursiveInputQuery;

#[test]
fn indirectly_recursive_input_objects_can_be_constructed() {
    use indirectly_recursive_input_query::*;

    let _ = IndirectlyRecursiveInput {
        head: "hello".to_string(),
        tail: Box::new(None),
    };

    let _ = IndirectlyRecursiveInput {
        head: "hi".to_string(),
        tail: Box::new(Some(IndirectlyRecursiveInputTailPart {
            name: "this is crazy".to_string(),
            recursed_field: Box::new(None),
        })),
    };
}
