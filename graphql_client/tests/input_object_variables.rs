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

    let out = serde_json::to_value(&variables).unwrap();

    let expected_default = serde_json::json!({
        "msg":{"content":null,"to":{"category":null,"email":"rosa.luxemburg@example.com","name":null}}
    });

    assert_eq!(out, expected_default);
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
pub struct InputCaseTestsQuery;

#[test]
fn input_objects_are_all_snake_case() {
    use input_case_tests_query::*;

    let _ = CaseTestInput {
        field_with_snake_case: "hello from".to_string(),
        other_field_with_camel_case: "the other side".to_string(),
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

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/input_object_variables/input_object_variables_query.graphql",
    schema_path = "tests/input_object_variables/input_object_variables_schema.graphql",
    variables_derives = "Default",
    response_derives = "Debug, PartialEq"
)]
pub struct RustNameQuery;

#[test]
fn rust_name_correctly_mapped() {
    use rust_name_query::*;
    let value = serde_json::to_value(&Variables {
        extern_: Some("hello".to_owned()),
        msg: <_>::default(),
    })
    .unwrap();
    assert_eq!(
        value
            .as_object()
            .unwrap()
            .get("extern")
            .unwrap()
            .as_str()
            .unwrap(),
        "hello"
    );
}
