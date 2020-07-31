// use futures::Future;
use graphql_client::{http::Client, GraphQLQuery};
// use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test_configure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn build_client() {
    // just to test it doesn't crash
    Client::new("https://example.com/graphql");
    Client::new("/graphql");
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/countries_schema.graphql",
    query_path = "tests/Germany.graphql"
)]
struct Germany;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_germany() {
    let response = Client::new("https://countries.trevorblades.com/")
        .call(Germany, germany::Variables)
        .await
        .unwrap();
    let continent_name = response
        .data
        .expect("response data is not null")
        .country
        .expect("country is not null")
        .continent
        .name;

    assert_eq!(continent_name, "Europe");
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/countries_schema.graphql",
    query_path = "tests/Germany.graphql"
)]
struct Country;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_country() {
    let response = Client::new("https://countries.trevorblades.com/")
        .call(
            Country,
            country::Variables {
                country_code: "CN".to_owned(),
            },
        )
        .await
        .unwrap();
    let continent_name = response
        .data
        .expect("response data is not null")
        .country
        .expect("country is not null")
        .continent
        .name;

    assert_eq!(continent_name, "Asia");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_bad_url() {
    let response = Client::new("https://example.com/non-existent/graphql/endpoint")
        .call(
            Country,
            country::Variables {
                country_code: "CN".to_owned(),
            },
        )
        .await;

    match response {
        Ok(_) => panic!("The API endpoint does not exist, this should not be called."),
        Err(_e) => {
            // TODO: What to assert here?
            // That url gives a response but it's not json, but in WASM it's blocked by cors
        }
    }
}
