extern crate failure;
extern crate reqwest;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate graphql_client;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/introspection_schema.graphql",
    query_path = "src/introspection_query.graphql",
    response_derives = "Serialize",
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
    },
    #[structopt(name = "generate")]
    Generate {
        // should be a glob
        paths: String,
        #[structopt(parse(from_os_str))]
        schema: PathBuf,
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
        } => introspect_schema(schema_location, output),
        Cli::Generate {
            paths: _,
            schema: _,
            output: _,
        } => unimplemented!(),
    }
}

fn introspect_schema(location: String, output: Option<PathBuf>) -> Result<(), failure::Error> {
    use reqwest::header::*;
    use reqwest::mime;
    use std::io::Write;

    let mut out: Box<Write> = match output {
        Some(path) => Box::new(::std::fs::File::create(path)?),
        None => Box::new(::std::io::stdout()),
    };

    let request_body: graphql_client::GraphQLQueryBody<()> = graphql_client::GraphQLQueryBody {
        variables: (),
        query: introspection_query::QUERY,
    };

    let client = reqwest::Client::new();
    let mut res = client
        .post(&location)
        .header(Accept(vec![qitem(mime::APPLICATION_JSON)]))
        .json(&request_body)
        .send()?;

    if res.status().is_success() {
    } else if res.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", res.status());
    }

    let json: graphql_client::GraphQLResponse<introspection_query::ResponseData> = res.json()?;
    let json = serde_json::to_string(&json)?;

    write!(out, "{}", json)?;

    Ok(())
}
