//! Use graphql_client inside browsers with [wasm-bindgen].
//!
//! This crate reexports all you need from graphql-client, so you do not need any other explicit dependencies.

// #![deny(warnings)]
#![deny(missing_docs)]
#![feature(non_exhaustive)]

#[macro_use]
pub extern crate graphql_client;
#[macro_use]
extern crate wasm_bindgen;

pub use graphql_client::GraphQLQuery;
use failure::*;
use futures::{Future, IntoFuture};
use log::*;
use std::collections::HashMap;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::{JsCast, JsValue};

/// The main interface to the library.
///
/// The workflow is the following:
///
/// - create a client
/// - (optionally) configure it
/// - use it to perform queries with the [call] method
pub struct Client {
    endpoint: String,
    headers: HashMap<String, String>,
}

/// All the ways a request can go wrong.
///
/// not exhaustive
#[derive(Debug, Fail)]
#[non_exhaustive]
pub enum ClientError {
    /// The body couldn't be built
    #[fail(display = "Request body is not a valid string")]
    Body,
    /// An error caused by window.fetch
    #[fail(display = "Network error")]
    Network,
    /// Error in a dynamic JS cast that should have worked
    #[fail(display = "JS casting error")]
    Cast,
    /// No window object could be retrieved
    #[fail(
        display = "No Window object available - the client works only in a browser (non-worker) context"
    )]
    NoWindow,
    /// Response shape does not match the generated code
    #[fail(display = "Response shape error",)]
    ResponseShape,
    /// Response could not be converted to text
    #[fail(display = "Response conversion to text failed (Response.text threw)")]
    ResponseText,
    /// Exception thrown when building the request
    #[fail(display = "Error building the request",)]
    RequestError,
    /// Other JS exception
    #[fail(display = "Unexpected JS exception")]
    JsException,
}

impl Client {
    /// Initialize a client. The `endpoint` parameter is the URI of the GraphQL API.
    pub fn new<Endpoint>(endpoint: Endpoint) -> Client
    where
        Endpoint: Into<String>,
    {
        Client {
            endpoint: endpoint.into(),
        }
    }

    /// Perform a query.
    pub fn call<Q: GraphQLQuery + 'static>(
        &self,
        _query: Q,
        variables: Q::Variables,
    ) -> impl Future<Item = graphql_client::Response<Q::ResponseData>, Error = ClientError> + 'static
    {
        // this can be removed when we convert to async/await
        let endpoint = self.endpoint.clone();

        web_sys::window()
            .ok_or_else(|| ClientError::NoWindow)
            .into_future()
            .and_then(move |window| {
                serde_json::to_string(&Q::build_query(variables))
                    .map_err(|_| ClientError::Body)
                    .map(move |body| (window, body))
            }).and_then(move |(window, body)| {
                let mut request_init = web_sys::RequestInit::new();
                request_init
                    .method("POST")
                    .body(Some(&JsValue::from_str(&body)));

                web_sys::Request::new_with_str_and_init(&endpoint, &request_init)
                    .map_err(|_| ClientError::JsException)
                    .map(|request| (window, request))
                // "Request constructor threw");
            }).and_then(move |(window, request)| {
                let request: Result<web_sys::Request, _> = request
                    .headers()
                    .set("Content-Type", "application/json")
                    .map_err(|_| ClientError::RequestError)
                    .map(|_| request);

                let request: Result<web_sys::Request, _> = request.and_then(|req| {
                    req.headers()
                        .set("Accept", "application/json")
                        .map_err(|_| ClientError::RequestError)
                        .map(|_| req)
                });

                request.map(move |request| (window, request))
            }).and_then(move |(window, request)| {
                JsFuture::from(window.fetch_with_request(&request))
                    .map_err(|_| ClientError::Network)
            }).and_then(move |res| {
                debug!("response: {:?}", res);
                res.dyn_into::<web_sys::Response>()
                    .map_err(|_| ClientError::Cast)
            }).and_then(move |cast_response| {
                cast_response.text().map_err(|_| ClientError::ResponseText)
            }).and_then(move |text_promise| {
                JsFuture::from(text_promise).map_err(|_| ClientError::ResponseText)
            }).and_then(|text| {
                let response_text = text.as_string().unwrap_or_else(|| String::new());
                debug!("response text as string: {:?}", response_text);
                serde_json::from_str(&response_text).map_err(|_| ClientError::ResponseShape)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_new() {
        Client::new("https://example.com/graphql");
        Client::new("/graphql");
    }
}
