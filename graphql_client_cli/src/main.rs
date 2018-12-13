extern crate env_logger;
extern crate log;
use env_logger::fmt::{Color, Style, StyledValue};
use log::Level;

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
#[macro_use]
extern crate syn;

#[cfg(feature = "rustfmt")]
extern crate rustfmt_nightly as rustfmt;

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
        /// Name of module.
        module_name: String,
        /// Name of target query. If you don't set this parameter, cli generate all queries in query file.
        #[structopt(short = "o", long = "selected-operation")]
        selected_operation: Option<String>,
        /// Additional derives that will be added to the generated structs and enums for the response and the variables.
        /// --additional-derives='Serialize,PartialEq'
        #[structopt(short = "a", long = "additional-derives")]
        additional_derives: Option<String>,
        /// You can choose deprecation strategy from allow, deny, or warn.
        /// Default value is warn.
        #[structopt(short = "d", long = "deprecation-strategy")]
        deprecation_strategy: Option<String>,
        /// If you don't want to execute rustfmt to generated code, set this option.
        /// Default value is false.
        /// Formating feature is disabled as default installation.
        #[structopt(long = "no-formatting")]
        no_formatting: bool,
        /// You can choose module and target struct visibility from pub and private.
        /// Default value is pub.
        #[structopt(short = "m", long = "module_visibility")]
        module_visibility: Option<String>,
        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },
}

fn main() -> Result<(), failure::Error> {
    set_env_logger();

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
            module_name,
            selected_operation,
            additional_derives,
            deprecation_strategy,
            no_formatting,
            module_visibility,
            output,
        } => generate::generate_code(
            query_path,
            schema_path,
            module_name,
            selected_operation,
            additional_derives,
            &deprecation_strategy,
            no_formatting,
            &module_visibility,
            &output,
        ),
    }
}

fn set_env_logger() {
    use std::io::Write;

    env_logger::Builder::from_default_env()
        .format(|f, record| {
            let mut style = f.style();
            let level = colored_level(&mut style, record.level());
            let mut style = f.style();
            let file = style.set_bold(true).value("file");
            let mut style = f.style();
            let module = style.set_bold(true).value("module");
            writeln!(
                f,
                "{} {}: {} {}: {}\n{}",
                level,
                file,
                record.file().unwrap(),
                module,
                record.target(),
                record.args()
            )
        })
        .init();
}

fn colored_level<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}
