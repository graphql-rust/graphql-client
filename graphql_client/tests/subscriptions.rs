use graphql_client::*;

const RESPONSE: &str = include_str!("subscription/subscription_query_response.json");

// If you uncomment this, it will not compile because the query is not valid. We need to investigate how we can make this a real test.
//
// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "tests/subscription/subscription_schema.graphql",
//     query_path = "tests/subscription/subscription_invalid_query.graphql"
// )]
// struct SubscriptionInvalidQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/subscription/subscription_schema.graphql",
    query_path = "tests/subscription/subscription_query.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct SubscriptionQuery;

#[test]
fn subscriptions_work() {
    let response_data: subscription_query::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    let expected = subscription_query::ResponseData {
        dog_birthdays: Some(vec![
            subscription_query::SubscriptionQueryDogBirthdays {
                name: Some("Maya".to_string()),
            },
            subscription_query::SubscriptionQueryDogBirthdays {
                name: Some("Norbert".to_string()),
            },
            subscription_query::SubscriptionQueryDogBirthdays {
                name: Some("Strelka".to_string()),
            },
            subscription_query::SubscriptionQueryDogBirthdays {
                name: Some("Belka".to_string()),
            },
        ]),
    };

    assert_eq!(response_data, expected);

    assert_eq!(
        response_data.dog_birthdays.map(|birthdays| birthdays.len()),
        Some(4)
    );
}
