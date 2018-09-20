extern crate failure;
extern crate reqwest;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate graphql_client;
extern crate graphql_client_codegen;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT};
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/introspection_schema.graphql",
    query_path = "src/introspection_query.graphql",
    response_derives = "Serialize"
)]
struct IntrospectionQuery;

#[derive(StructOpt)]
enum Cli {
    #[structopt(name = "introspect-schema")]
    IntrospectSchema {
        /// The URL of a GraphQL endpoint to introspect.
        schema_location: String,
        /// Where to write the JSON for the introspected schema.
        #[structopt(parse(from_os_str))]
        #[structopt(long = "output")]
        output: Option<PathBuf>,
        // Set authorizaiton header.
        #[structopt(long = "authorization")]
        authorization: Option<String>,
    },
    #[structopt(name = "generate")]
    Generate {
        // should be a glob
        /// Path to graphql query file.
        #[structopt(parse(from_os_str))]
        query_path: PathBuf,
        /// Path to graphql schema file.
        #[structopt(parse(from_os_str))]
        schema_path: PathBuf,
        /// Name of struct that is implementation target.
        selected_operation: String,
        /// Additional derives that will be added to the generated structs and enums for the response and the variables.
        /// --additional-derives='Serialize,PartialEq'
        #[structopt(short = "a", long = "additional-derives")]
        additional_derives: Option<String>,
        /// You can choose deprecation strategy from allow, deny, or warn.
        /// Default value is warn.
        #[structopt(short = "d", long = "deprecation-strategy",)]
        deprecation_strategy: Option<String>,
        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },
}

fn main() -> Result<(), failure::Error> {
    let cli = Cli::from_args();
    match cli {
        Cli::IntrospectSchema {
            schema_location,
            output,
            authorization,
        } => introspect_schema(schema_location, output, authorization),
        Cli::Generate {
            query_path,
            schema_path,
            selected_operation,
            additional_derives,
            deprecation_strategy,
            output,
        } => generate_code(
            query_path,
            schema_path,
            selected_operation,
            additional_derives,
            deprecation_strategy,
            output,
        ),
    }
}

fn introspect_schema(
    location: String,
    output: Option<PathBuf>,
    authorization: Option<String>,
) -> Result<(), failure::Error> {
    use std::io::Write;

    let out: Box<Write> = match output {
        Some(path) => Box::new(::std::fs::File::create(path)?),
        None => Box::new(::std::io::stdout()),
    };

    let request_body: graphql_client::GraphQLQueryBody<()> = graphql_client::GraphQLQueryBody {
        variables: (),
        query: introspection_query::QUERY,
        operation_name: introspection_query::OPERATION_NAME,
    };

    let client = reqwest::Client::new();

    let mut req_builder = client.post(&location).headers(construct_headers());
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

fn generate_code(
    query_path: PathBuf,
    schema_path: PathBuf,
    selected_operation: String,
    additional_derives: Option<String>,
    deprecation_strategy: Option<String>,
    output: PathBuf,
) -> Result<(), failure::Error> {
    let deprecation_strategy = deprecation_strategy.as_ref().map(|s| s.as_str());
    let deprecation_strategy = match deprecation_strategy {
        Some("allow") => Some(graphql_client_codegen::deprecation::DeprecationStrategy::Allow),
        Some("deny") => Some(graphql_client_codegen::deprecation::DeprecationStrategy::Deny),
        Some("warn") => Some(graphql_client_codegen::deprecation::DeprecationStrategy::Warn),
        _ => None,
    };

    let options = graphql_client_codegen::GraphQLClientDeriveOptions {
        selected_operation,
        additional_derives: additional_derives,
        deprecation_strategy,
    };
    let gen = graphql_client_codegen::generate_module_token_stream(
        query_path,
        schema_path,
        Some(options),
    )?;
    let mut file = File::create(output)?;
    write!(file, "{}", gen.to_string());
    Ok(())
}
