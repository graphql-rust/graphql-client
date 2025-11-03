//! A concrete client implementation over HTTP with reqwest.

use crate::GraphQLQuery;
use reqwest_crate as reqwest;

/// Use the provided reqwest::Client to post a GraphQL request.
#[cfg(any(feature = "reqwest", feature = "reqwest-rustls"))]
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
#[cfg(any(feature = "reqwest", feature = "reqwest-rustls"))]
pub async fn post_graphql_batch<Q: GraphQLQuery, U: reqwest::IntoUrl>(
    client: &reqwest::Client,
    url: U,
    variables: Vec<Q::Variables>,
) -> Result<Vec<crate::Response<Q::ResponseData>>, reqwest::Error> {
    let body = Q::build_batch_query(variables);
    let reqwest_response = client.post(url).json(&body).send().await?;

    reqwest_response.json().await
}

/// Use the provided reqwest::Client to post a GraphQL request.
#[cfg(feature = "reqwest-blocking")]
pub fn post_graphql_blocking<Q: GraphQLQuery, U: reqwest::IntoUrl>(
    client: &reqwest::blocking::Client,
    url: U,
    variables: Q::Variables,
) -> Result<crate::Response<Q::ResponseData>, reqwest::Error> {
    let body = Q::build_query(variables);
    let reqwest_response = client.post(url).json(&body).send()?;

    reqwest_response.json()
}

/// Use the provided reqwest::Client to post a GraphQL request.
#[cfg(feature = "reqwest-blocking")]
pub fn post_graphql_blocking_batch<Q: GraphQLQuery, U: reqwest::IntoUrl>(
    client: &reqwest::blocking::Client,
    url: U,
    variables: Vec<Q::Variables>,
) -> Result<Vec<crate::Response<Q::ResponseData>>, reqwest::Error> {
    let body = Q::build_batch_query(variables);
    let reqwest_response = client.post(url).json(&body).send()?;

    reqwest_response.json()
}
