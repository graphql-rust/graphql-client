use graphql_client::*;
use opt_query::Param;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/default/query.graphql",
    schema_path = "tests/default/schema.graphql",
    variables_derives = "Default, Debug"
)]
struct OptQuery;

fn normalize_whitespace(s: &str) -> String {
    s.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn query_is_formatted_correctly() {
    let variables = opt_query::Variables {
        param: Some(Param::AUTHOR),
    };
    let query = OptQuery::build_query(variables);
    let debug_output = format!("{:#?}", query);

    let original_query = include_str!("default/query.graphql");

    // Normalize both for comparison
    let normalized_debug_output = normalize_whitespace(&debug_output);
    let normalized_original_query = normalize_whitespace(original_query);

    assert!(
        normalized_debug_output.contains(&normalized_original_query),
        "Debug output did not contain the expected query.\nDebug output:\n{}\n\nExpected query:\n{}",
        normalized_debug_output,
        normalized_original_query
    );
}
