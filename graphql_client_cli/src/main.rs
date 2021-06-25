use env_logger::fmt::{Color, Style, StyledValue};
use log::Level;

#[cfg(feature = "rustfmt")]
extern crate rustfmt_nightly as rustfmt;

mod generate;
mod introspect_schema;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(author, about)]
enum Cli {
    /// Get the schema from a live GraphQL API. The schema is printed to stdout.
    #[structopt(name = "introspect-schema")]
    IntrospectSchema {
        /// The URL of a GraphQL endpoint to introspect.
        schema_location: String,
        /// Where to write the JSON for the introspected schema.
        #[structopt(parse(from_os_str))]
        #[structopt(long = "output")]
        output: Option<PathBuf>,
        /// Set the contents of the Authorizaiton header.
        #[structopt(long = "authorization")]
        authorization: Option<String>,
        /// Specify custom headers.
        /// --header 'X-Name: Value'
        #[structopt(long = "header")]
        headers: Vec<introspect_schema::Header>,
        /// Disable ssl verification.
        /// Default value is false.
        #[structopt(long = "no-ssl")]
        no_ssl: bool,
    },
    #[structopt(name = "generate")]
    Generate {
        /// Path to GraphQL schema file (.json or .graphql).
        #[structopt(short = "s", long = "schema-path")]
        schema_path: PathBuf,
        /// Path to the GraphQL query file.
        query_path: PathBuf,
        /// Name of target query. If you don't set this parameter, cli generate all queries in query file.
        #[structopt(long = "selected-operation")]
        selected_operation: Option<String>,
        /// Additional derives that will be added to the generated structs and enums for the variables.
        /// --variables-derives='Serialize,PartialEq'
        #[structopt(short = "I", long = "variables-derives")]
        variables_derives: Option<String>,
        /// Additional derives that will be added to the generated structs and enums for the response.
        /// --output-derives='Serialize,PartialEq'
        #[structopt(short = "O", long = "response-derives")]
        response_derives: Option<String>,
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
        #[structopt(short = "m", long = "module-visibility")]
        module_visibility: Option<String>,
        /// The directory in which the code will be generated.
        ///
        /// If this option is omitted, the code will be generated next to the .graphql
        /// file, with the same name and the .rs extension.
        #[structopt(short = "o", long = "output-directory")]
        output_directory: Option<PathBuf>,
        /// The module where the custom scalar definitions are located.
        /// --custom-scalars-module='crate::gql::custom_scalars'
        #[structopt(short = "p", long = "custom-scalars-module")]
        custom_scalars_module: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    set_env_logger();

    let cli = Cli::from_args();
    match cli {
        Cli::IntrospectSchema {
            schema_location,
            output,
            authorization,
            headers,
            no_ssl,
        } => introspect_schema::introspect_schema(
            &schema_location,
            output,
            authorization,
            headers,
            no_ssl,
        ),
        Cli::Generate {
            variables_derives,
            response_derives,
            deprecation_strategy,
            module_visibility,
            no_formatting,
            output_directory,
            query_path,
            schema_path,
            selected_operation,
            custom_scalars_module,
        } => generate::generate_code(generate::CliCodegenParams {
            query_path,
            schema_path,
            selected_operation,
            variables_derives,
            response_derives,
            deprecation_strategy,
            no_formatting,
            module_visibility,
            output_directory,
            custom_scalars_module,
        }),
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
