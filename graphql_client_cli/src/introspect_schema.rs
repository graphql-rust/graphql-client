use anyhow::format_err;
use graphql_client::GraphQLQuery;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/introspection_schema.graphql",
    query_path = "src/graphql/introspection_query.graphql",
    response_derives = "Serialize",
    variable_derives = "Deserialize"
)]
#[allow(dead_code)]
struct IntrospectionQuery;

pub fn introspect_schema(
    location: &str,
    output: Option<PathBuf>,
    authorization: Option<String>,
    headers: Vec<Header>,
) -> Result<(), anyhow::Error> {
    use std::io::Write;

    let out: Box<dyn Write> = match output {
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

    for custom_header in headers {
        req_builder = req_builder.header(custom_header.name.as_str(), custom_header.value.as_str());
    }

    if let Some(token) = authorization {
        req_builder = req_builder.bearer_auth(token.as_str());
    };

    let mut res = req_builder.json(&request_body).send()?;

    if res.status().is_success() {
        // do nothing
    } else if res.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", res.status());
    }

    let json: serde_json::Value = res.json()?;
    serde_json::to_writer_pretty(out, &json)?;
    Ok(())
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers
}

#[derive(Debug, PartialEq)]
pub struct Header {
    name: String,
    value: String,
}

impl FromStr for Header {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // error: colon required for name/value pair
        if !input.contains(':') {
            return Err(format_err!(
                "Invalid header input. A colon is required to separate the name and value. [{}]",
                input
            ));
        }

        // split on first colon and trim whitespace from name and value
        let name_value: Vec<&str> = input.splitn(2, ':').collect();
        let name = name_value[0].trim();
        let value = name_value[1].trim();

        // error: field name must be
        if name.is_empty() {
            return Err(format_err!(
                "Invalid header input. Field name is required before colon. [{}]",
                input
            ));
        }

        // error: no whitespace in field name
        if name.split_whitespace().count() > 1 {
            return Err(format_err!(
                "Invalid header input. Whitespace not allowed in field name. [{}]",
                input
            ));
        }

        Ok(Self {
            name: name.to_string(),
            value: value.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_errors_invalid_headers() {
        // https://tools.ietf.org/html/rfc7230#section-3.2

        for input in [
            "X-Name Value",   // error: colon required for name/value pair
            ": Value",        // error: field name must be
            "X Name: Value",  // error: no whitespace in field name
            "X\tName: Value", // error: no whitespace in field name (tab)
        ]
        .iter()
        {
            let header = Header::from_str(input);

            assert!(header.is_err(), "Expected error: [{}]", input);
        }
    }

    #[test]
    fn it_parses_valid_headers() {
        // https://tools.ietf.org/html/rfc7230#section-3.2

        let expected1 = Header {
            name: "X-Name".to_string(),
            value: "Value".to_string(),
        };
        let expected2 = Header {
            name: "X-Name".to_string(),
            value: "Value:".to_string(),
        };

        for (input, expected) in [
            ("X-Name: Value", &expected1),  // ideal
            ("X-Name:Value", &expected1),   // no optional whitespace
            ("X-Name: Value ", &expected1), // with optional whitespace
            ("X-Name:\tValue", &expected1), // with optional whitespace (tab)
            ("X-Name: Value:", &expected2), // with colon in value
            // not allowed per RFC, but we'll forgive
            ("X-Name : Value", &expected1),
            (" X-Name: Value", &expected1),
        ]
        .iter()
        {
            let header = Header::from_str(input);

            assert!(header.is_ok(), "Expected ok: [{}]", input);
            assert_eq!(
                header.unwrap(),
                **expected,
                "Expected equality: [{}]",
                input
            );
        }
    }
}
