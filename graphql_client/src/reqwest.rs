//! A concrete client implementation over HTTP with reqwest.

use crate::GraphQLQuery;

/// Use the provided reqwest::Client to post a GraphQL request.
pub async fn post_graphql<Q: GraphQLQuery, U: reqwest::IntoUrl>(
    client: &reqwest::Client,
    url: U,
    variables: Q::Variables,
) -> Result<crate::Response<Q::ResponseData>, reqwest::Error> {
    let body = Q::build_query(variables);
    let reqwest_response = client.post(url).json(&body).send().await?;

    Ok(reqwest_response.json().await?)
}
