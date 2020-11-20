//! Use graphql_client inside browsers with
//! [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen).

use crate::*;
use log::*;
use std::collections::HashMap;
use thiserror::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

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
#[derive(Debug, Error, PartialEq)]
pub enum ClientError {
    /// The body couldn't be built
    #[error("Request body is not a valid string")]
    Body,
    /// An error caused by window.fetch
    #[error("Network error")]
    Network(String),
    /// Error in a dynamic JS cast that should have worked
    #[error("JS casting error")]
    Cast,
    /// No window object could be retrieved
    #[error(
        "No Window object available - the client works only in a browser (non-worker) context"
    )]
    NoWindow,
    /// Response shape does not match the generated code
    #[error("Response shape error")]
    ResponseShape,
    /// Response could not be converted to text
    #[error("Response conversion to text failed (Response.text threw)")]
    ResponseText,
    /// Exception thrown when building the request
    #[error("Error building the request")]
    RequestError,
    /// Other JS exception
    #[error("Unexpected JS exception")]
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
            headers: HashMap::new(),
        }
    }

    /// Add a header to those sent with the requests. Can be used for things like authorization.
    pub fn add_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.into(), value.into());
    }

    /// Perform a query.
    ///
    // Lint disabled: We can pass by value because it's always an empty struct.
    #[allow(clippy::needless_pass_by_value)]
    pub async fn call<Q: GraphQLQuery + 'static>(
        &self,
        _query: Q,
        variables: Q::Variables,
    ) -> Result<crate::Response<Q::ResponseData>, ClientError> {
        let window = web_sys::window().ok_or(ClientError::NoWindow)?;
        let body =
            serde_json::to_string(&Q::build_query(variables)).map_err(|_| ClientError::Body)?;

        let mut request_init = web_sys::RequestInit::new();
        request_init
            .method("POST")
            .body(Some(&JsValue::from_str(&body)));

        let request = web_sys::Request::new_with_str_and_init(&self.endpoint, &request_init)
            .map_err(|_| ClientError::JsException)?;

        let headers = request.headers();
        headers
            .set("Content-Type", "application/json")
            .map_err(|_| ClientError::RequestError)?;
        headers
            .set("Accept", "application/json")
            .map_err(|_| ClientError::RequestError)?;
        for (header_name, header_value) in self.headers.iter() {
            headers
                .set(header_name, header_value)
                .map_err(|_| ClientError::RequestError)?;
        }

        let res = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|err| ClientError::Network(js_sys::Error::from(err).message().into()))?;
        debug!("response: {:?}", res);
        let cast_response = res
            .dyn_into::<web_sys::Response>()
            .map_err(|_| ClientError::Cast)?;

        let text_promise = cast_response
            .text()
            .map_err(|_| ClientError::ResponseText)?;
        let text = JsFuture::from(text_promise)
            .await
            .map_err(|_| ClientError::ResponseText)?;

        let response_text = text.as_string().unwrap_or_default();
        debug!("response text as string: {:?}", response_text);
        let response_data =
            serde_json::from_str(&response_text).map_err(|_| ClientError::ResponseShape)?;
        Ok(response_data)
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
