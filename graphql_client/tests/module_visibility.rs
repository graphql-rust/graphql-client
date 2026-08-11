mod inner {
    use graphql_client::*;

    graphql_queries!(
        query_path = "tests/module_visibility/query.graphql",
        schema_path = "tests/module_visibility/schema.graphql",
        module_visibility = "pub"
    );
}

#[test]
fn module_visibility() {
    let _ = inner::test_query::ResponseData { value: None };
}
