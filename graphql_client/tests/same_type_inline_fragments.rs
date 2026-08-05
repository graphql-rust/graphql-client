use graphql_client::*;

const ONLY_INLINE_FRAGMENT_RESPONSE: &str =
    include_str!("same_type_inline_fragments/only_inline_fragment_response.json");
const INLINE_FRAGMENT_WITH_SIBLING_RESPONSE: &str =
    include_str!("same_type_inline_fragments/inline_fragment_with_sibling_response.json");

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/same_type_inline_fragments/query.graphql",
    schema_path = "tests/same_type_inline_fragments/schema.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct OnlyInlineFragment;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/same_type_inline_fragments/query.graphql",
    schema_path = "tests/same_type_inline_fragments/schema.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct InlineFragmentWithSibling;

#[test]
fn inline_fragment_on_enclosing_type_as_only_selection() {
    let response_data: only_inline_fragment::ResponseData =
        serde_json::from_str(ONLY_INLINE_FRAGMENT_RESPONSE).unwrap();

    let expected = only_inline_fragment::ResponseData {
        thing: only_inline_fragment::OnlyInlineFragmentThing {
            name: "Sputnik".to_string(),
        },
    };

    assert_eq!(response_data, expected);
}

#[test]
fn inline_fragment_on_enclosing_type_next_to_a_sibling_field() {
    let response_data: inline_fragment_with_sibling::ResponseData =
        serde_json::from_str(INLINE_FRAGMENT_WITH_SIBLING_RESPONSE).unwrap();

    let expected = inline_fragment_with_sibling::ResponseData {
        thing: inline_fragment_with_sibling::InlineFragmentWithSiblingThing {
            id: "1".to_string(),
            name: "Sputnik".to_string(),
        },
    };

    assert_eq!(response_data, expected);
}
