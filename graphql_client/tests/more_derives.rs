use graphql_client::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/more_derives/schema.graphql",
    query_path = "tests/more_derives/query.graphql",
    response_derives = "Debug, PartialEq, PartialOrd"
)]
pub struct MoreDerives;

#[test]
fn response_derives_can_be_added() {
    let response_data = more_derives::ResponseData {
        current_user: Some(more_derives::MoreDerivesCurrentUser {
            id: Some("abcd".to_owned()),
            name: Some("Angela Merkel".to_owned()),
        }),
    };

    let response_data_2 = more_derives::ResponseData {
        current_user: Some(more_derives::MoreDerivesCurrentUser {
            id: Some("ffff".to_owned()),
            name: Some("Winnie the Pooh".to_owned()),
        }),
    };

    assert_ne!(response_data, response_data_2);
    assert!(response_data < response_data_2);
}
