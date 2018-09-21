use failure;
use graphql_client;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde_json;
use std::path::PathBuf;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection_schema.graphql",
    query_path = "src/graphql/introspection_query.graphql",
    response_derives = "Serialize"
)]
#[allow(dead_code)]
struct IntrospectionQuery;

pub fn introspect_schema(
    location: &str,
    output: Option<PathBuf>,
    authorization: Option<String>,
) -> Result<(), failure::Error> {
    use std::io::Write;

    let out: Box<Write> = match output {
        Some(path) => Box::new(::std::fs::File::create(path)?),
        None => Box::new(::std::io::stdout()),
    };

    let request_body: graphql_client::QueryBody<()> = graphql_client::QueryBody {
        variables: (),
        query: introspection_query::QUERY,
        operation_name: introspection_query::OPERATION_NAME,
    };

    let client = reqwest::Client::new();

    let mut req_builder = client.post(location).headers(construct_headers());
    if let Some(token) = authorization {
        req_builder = req_builder.bearer_auth(token.as_str());
    };

    let mut res = req_builder.json(&request_body).send()?;

    if res.status().is_success() {
    } else if res.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", res.status());
    }

    let json: serde_json::Value = res.json()?;
    Ok(serde_json::to_writer_pretty(out, &json)?)
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers
}
