#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// If you comment this out, it will not compile because the query is not valid. We need to investigate how we can make this a real test.
//
// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "tests/subscription/subscription_schema.graphql",
//     query_path = "tests/subscription/subscription_invalid_query.graphql"
// )]
// struct SubscriptionInvalidQuery;

#[test]
fn subscriptions_work() {
    unimplemented!("subscriptions test");
}
