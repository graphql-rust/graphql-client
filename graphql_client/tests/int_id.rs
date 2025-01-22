use graphql_client::*;
use serde_json::json;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/more_derives/schema.graphql",
    query_path = "tests/more_derives/query.graphql",
    response_derives = "Debug, PartialEq, Eq, std::cmp::PartialOrd"
)]
pub struct MoreDerives;

#[test]
fn int_id() {
    let response1 = json!({
        "currentUser": {
            "id": 1,
            "name": "Don Draper",
        }
    });

    let response2 = json!({
        "currentUser": {
            "id": "2",
            "name": "Peggy Olson",
        }
    });

    let res1 = serde_json::from_value::<more_derives::ResponseData>(response1)
        .expect("should deserialize");
    assert_eq!(
        res1.current_user.expect("res1 current user").id,
        Some("1".into())
    );

    let res2 = serde_json::from_value::<more_derives::ResponseData>(response2)
        .expect("should deserialize");
    assert_eq!(
        res2.current_user.expect("res2 current user").id,
        Some("2".into())
    );
}
