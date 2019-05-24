#![cfg(target_arch = "wasm32")]

use futures::Future;
use graphql_client::{web::Client, GraphQLQuery};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn build_client() {
    // just to test it doesn't crash
    Client::new("https://example.com/graphql");
    Client::new("/graphql");
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/countries_schema.json",
    query_path = "tests/Germany.graphql"
)]
struct Germany;

#[wasm_bindgen_test(async)]
fn test_germany() -> impl Future<Item = (), Error = JsValue> {
    Client::new("https://countries.trevorblades.com/")
        .call(Germany, germany::Variables)
        .map(|response| {
            let continent_name = response
                .data
                .expect("response data is not null")
                .country
                .expect("country is not null")
                .continent
                .expect("continent is not null")
                .name
                .expect("germany is on a continent");

            assert_eq!(continent_name, "Europe");
        })
        .map_err(|err| {
            panic!("{:?}", err);
        })
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/countries_schema.json",
    query_path = "tests/Germany.graphql"
)]
struct Country;

#[wasm_bindgen_test(async)]
fn test_country() -> impl Future<Item = (), Error = JsValue> {
    Client::new("https://countries.trevorblades.com/")
        .call(
            Country,
            country::Variables {
                country_code: "CN".to_owned(),
            },
        )
        .map(|response| {
            let continent_name = response
                .data
                .expect("response data is not null")
                .country
                .expect("country is not null")
                .continent
                .expect("continent is not null")
                .name
                .expect("country is on a continent");

            assert_eq!(continent_name, "Asia");
        })
        .map_err(|err| {
            panic!("{:?}", err);
        })
}

#[wasm_bindgen_test(async)]
fn test_bad_url() -> impl Future<Item = (), Error = JsValue> {
    Client::new("https://example.com/non-existent/graphql/endpoint")
        .call(
            Country,
            country::Variables {
                country_code: "CN".to_owned(),
            },
        )
        .map(|_response| panic!("The API endpoint does not exist, this should not be called."))
        .map_err(|err| {
            assert_eq!(
                err,
                graphql_client::web::ClientError::Network(
                    "NetworkError when attempting to fetch resource.".into()
                )
            );
        })
        .then(|_| Ok(()))
}
