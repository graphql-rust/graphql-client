use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/scalar_variables/scalar_variables_query.graphql",
    schema_path = "tests/scalar_variables/scalar_variables_schema.graphql"
)]
pub struct ScalarVariablesQuery;

#[test]
fn scalar_variables_query_variables_struct() {
    let _ = scalar_variables_query::Variables {
        msg: "hello".to_string(),
        reps: Some(32),
    };
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/scalar_variables/scalar_variables_query_defaults.graphql",
    schema_path = "tests/scalar_variables/scalar_variables_schema.graphql"
)]
pub struct DefaultScalarVariablesQuery;

#[test]
fn scalar_variables_default() {
    let variables = default_scalar_variables_query::Variables {
        msg: default_scalar_variables_query::Variables::default_msg(),
        reps: default_scalar_variables_query::Variables::default_reps(),
    };

    let out = serde_json::to_string(&variables).unwrap();

    assert_eq!(out, r#"{"msg":"o, hai","reps":3}"#);
}
