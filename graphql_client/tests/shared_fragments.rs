use graphql_client::*;

graphql_queries!(
    query_path = "tests/shared_fragments/query.graphql",
    schema_path = "tests/shared_fragments/schema.graphql",
);

#[test]
fn shared_fragments() {
    let _: common::FragmentReference = a::ResponseData { in_fragment: None };
    let _: a::ResponseData = b::ResponseData { in_fragment: None };
}
