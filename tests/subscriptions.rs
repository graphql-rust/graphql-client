#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

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
)]
struct SubscriptionQuery;

#[test]
fn subscriptions_work() {
    let response_data: subscription_query::ResponseData = serde_json::from_str(RESPONSE).unwrap();

    let expected = r##"ResponseData { dog_birthdays: Some([RustBirthdaysDogBirthdays { name: Some("Maya") }, RustBirthdaysDogBirthdays { name: Some("Norbert") }, RustBirthdaysDogBirthdays { name: Some("Strelka") }, RustBirthdaysDogBirthdays { name: Some("Belka") }]) }"##;

    assert_eq!(format!("{:?}", response_data), expected);

    assert_eq!(
        response_data.dog_birthdays.map(|birthdays| birthdays.len()),
        Some(4)
    );
}
