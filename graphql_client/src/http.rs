//! Some global

use crate::*;
use std::collections::HashMap;

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
    reqwest_client: reqwest::Client,
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
            reqwest_client: reqwest::Client::new(),
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
        // TODO: remove tests and test harness
        // TODO: custom headers
        let reqwest_response = self
            .reqwest_client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&Q::build_query(variables)).unwrap())
            .send()
            .await?;

        let text_response = reqwest_response.text().await?;

        Ok(serde_json::from_str(&text_response)?)
    }
}

/// TODO
#[derive(Debug)]
pub enum ClientError {
    /// TODO
    ReqwestError(reqwest::Error),
    /// TODO
    SerdeError(serde_json::Error),
}

impl From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> Self {
        ClientError::ReqwestError(e)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(e: serde_json::Error) -> Self {
        ClientError::SerdeError(e)
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
