//! A concrete client implementation over HTTP with reqwest.

use crate::GraphQLQuery;

#[cfg(all(feature = "reqwest11-crate", not(feature = "reqwest12-crate")))]
use reqwest11_crate as reqwest;
#[cfg(feature = "reqwest12-crate")]
use reqwest12_crate as reqwest;

/// Use the provided reqwest::Client to post a GraphQL request.
#[cfg(any(
    feature = "reqwest11",
    feature = "reqwest11-rustls",
    feature = "reqwest12",
    feature = "reqwest12-rustls"
))]
pub async fn post_graphql<Q: GraphQLQuery, U: reqwest::IntoUrl>(
    client: &reqwest::Client,
    url: U,
    variables: Q::Variables,
) -> Result<crate::Response<Q::ResponseData>, reqwest::Error> {
    let body = Q::build_query(variables);
    let reqwest_response = client.post(url).json(&body).send().await?;

    reqwest_response.json().await
}

/// Use the provided reqwest::Client to post a GraphQL request.
#[cfg(any(feature = "reqwest11-blocking", feature = "reqwest12-blocking"))]
pub fn post_graphql_blocking<Q: GraphQLQuery, U: reqwest::IntoUrl>(
    client: &reqwest::blocking::Client,
    url: U,
    variables: Q::Variables,
) -> Result<crate::Response<Q::ResponseData>, reqwest::Error> {
    let body = Q::build_query(variables);
    let reqwest_response = client.post(url).json(&body).send()?;

    reqwest_response.json()
}
