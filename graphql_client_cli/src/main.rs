extern crate failure;
extern crate reqwest;
extern crate structopt;
#[macro_use]
extern crate graphql_client;
extern crate graphql_client_codegen;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod generate;
mod introspect_schema;
use std::path::PathBuf;
use structopt::StructOpt;

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
        } => introspect_schema::introspect_schema(&schema_location, output, authorization),
        Cli::Generate {
            query_path,
            schema_path,
            selected_operation,
            additional_derives,
            deprecation_strategy,
            output,
        } => generate::generate_code(
            query_path,
            schema_path,
            selected_operation,
            additional_derives,
            deprecation_strategy,
            output,
        ),
    }
}
