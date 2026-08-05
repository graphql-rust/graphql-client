use graphql_client::*;

const RESPONSE: &str =
    include_str!("typename_only_selection/typename_only_on_union_member_response.json");

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/typename_only_selection/query.graphql",
    schema_path = "tests/typename_only_selection/schema.graphql",
    response_derives = "Debug, PartialEq, Eq"
)]
pub struct TypenameOnlyOnUnionMember;

#[test]
fn typename_only_on_union_member_deserialization() {
    let response_data: typename_only_on_union_member::ResponseData =
        serde_json::from_str(RESPONSE).unwrap();

    let expected = typename_only_on_union_member::ResponseData {
        do_thing: typename_only_on_union_member::TypenameOnlyOnUnionMemberDoThing::DoThingSuccess(
            typename_only_on_union_member::TypenameOnlyOnUnionMemberDoThingOnDoThingSuccess {},
        ),
    };

    assert_eq!(response_data, expected);
}
