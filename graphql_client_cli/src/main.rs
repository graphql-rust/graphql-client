mod error;
mod generate;
mod introspection_queries;
mod introspection_schema;

use clap::Parser;
use env_logger::fmt::style;
use log::Level;

use error::Error;

use std::path::PathBuf;
use Cli::Generate;

type CliResult<T> = Result<T, Error>;

#[derive(Parser)]
#[clap(author, about, version)]
enum Cli {
    /// Get the schema from a live GraphQL API. The schema is printed to stdout.
    #[clap(name = "introspect-schema")]
    IntrospectSchema {
        /// The URL of a GraphQL endpoint to introspect.
        schema_location: String,
        /// Where to write the JSON for the introspected schema.
        #[arg(long = "output")]
        output: Option<PathBuf>,
        /// Set the contents of the Authorization header.
        #[arg(long = "authorization")]
        authorization: Option<String>,
        /// Specify custom headers.
        /// --header 'X-Name: Value'
        #[arg(long = "header")]
        headers: Vec<introspection_schema::Header>,
        /// Disable ssl verification.
        /// Default value is false.
        #[clap(long = "no-ssl")]
        no_ssl: bool,
        /// Introspection Option: is-one-of will enable the @oneOf directive in the introspection query.
        /// This is an proposed feature and is not compatible with many GraphQL servers.
        /// Default value is false.
        #[clap(long = "is-one-of")]
        is_one_of: bool,
        /// Introspection Option: specify-by-url will enable the @specifiedByURL directive in the introspection query.
        /// This is an proposed feature and is not compatible with many GraphQL servers.
        /// Default value is false.
        #[clap(long = "specify-by-url")]
        specify_by_url: bool,
    },
    #[clap(name = "generate")]
    Generate {
        /// Path to GraphQL schema file (.json or .graphql).
        #[clap(short = 's', long = "schema-path")]
        schema_path: PathBuf,
        /// Path to the GraphQL query file.
        query_path: PathBuf,
        /// Name of target query. If you don't set this parameter, cli generate all queries in query file.
        #[clap(long = "selected-operation")]
        selected_operation: Option<String>,
        /// Additional derives that will be added to the generated structs and enums for the variables.
        /// --variables-derives='Serialize,PartialEq'
        #[clap(short = 'I', long = "variables-derives")]
        variables_derives: Option<String>,
        /// Additional derives that will be added to the generated structs and enums for the response.
        /// --response-derives='Serialize,PartialEq'
        #[clap(short = 'O', long = "response-derives")]
        response_derives: Option<String>,
        /// You can choose deprecation strategy from allow, deny, or warn.
        /// Default value is warn.
        #[clap(short = 'd', long = "deprecation-strategy")]
        deprecation_strategy: Option<String>,
        /// If you don't want to execute rustfmt to generated code, set this option.
        /// Default value is false.
        #[clap(long = "no-formatting")]
        no_formatting: bool,
        /// You can choose module and target struct visibility from pub and private.
        /// Default value is pub.
        #[clap(short = 'm', long = "module-visibility")]
        module_visibility: Option<String>,
        /// The directory in which the code will be generated.
        ///
        /// If this option is omitted, the code will be generated next to the .graphql
        /// file, with the same name and the .rs extension.
        #[clap(short = 'o', long = "output-directory")]
        output_directory: Option<PathBuf>,
        /// The module where the custom scalar definitions are located.
        /// --custom-scalars-module='crate::gql::custom_scalars'
        #[clap(short = 'p', long = "custom-scalars-module")]
        custom_scalars_module: Option<String>,
        /// A flag indicating if the enum representing the variants of a fragment union/interface should have a "other" variant
        /// --fragments-other-variant
        #[clap(long = "fragments-other-variant")]
        fragments_other_variant: bool,
        /// List of externally defined enum types. Type names must match those used in the schema exactly
        #[clap(long = "external-enums", num_args(0..), action(clap::ArgAction::Append))]
        external_enums: Option<Vec<String>>,
    },
}

fn main() -> CliResult<()> {
    set_env_logger();

    let cli = Cli::parse();
    match cli {
        Cli::IntrospectSchema {
            schema_location,
            output,
            authorization,
            headers,
            no_ssl,
            is_one_of,
            specify_by_url,
        } => introspection_schema::introspect_schema(
            &schema_location,
            output,
            authorization,
            headers,
            no_ssl,
            is_one_of,
            specify_by_url,
        ),
        Generate {
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
            fragments_other_variant,
            external_enums,
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
            fragments_other_variant,
            external_enums,
        }),
    }
}

fn set_env_logger() {
    use std::io::Write;

    env_logger::Builder::from_default_env()
        .format(|f, record | {
            let level_style = colored_level(record.level());
            let file_style = style::Style::new().bold();
            let module_style = style::Style::new().bold();
            writeln!(
                f,
                "{level_style}{}{level_style:#} {file_style}file{file_style:#}: {} {module_style}module{module_style:#}: {}\n{}",
                record.level(),
                record.file().unwrap(),
                record.target(),
                record.args()
            )
        }).init();
}

fn colored_level(level: Level) -> style::Style {
    match level {
        Level::Trace => style::Style::new().fg_color(Some(style::AnsiColor::Magenta.into())),
        Level::Debug => style::Style::new().fg_color(Some(style::AnsiColor::Blue.into())),
        Level::Info => style::Style::new().fg_color(Some(style::AnsiColor::Green.into())),
        Level::Warn => style::Style::new().fg_color(Some(style::AnsiColor::Yellow.into())),
        Level::Error => style::Style::new().fg_color(Some(style::AnsiColor::Red.into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::trace;
    use std::env;

    #[test]
    fn test_colored_level() {
        assert_eq!(
            colored_level(Level::Trace),
            style::Style::new().fg_color(Some(style::AnsiColor::Magenta.into()))
        );
        assert_eq!(
            colored_level(Level::Debug),
            style::Style::new().fg_color(Some(style::AnsiColor::Blue.into()))
        );
        assert_eq!(
            colored_level(Level::Info),
            style::Style::new().fg_color(Some(style::AnsiColor::Green.into()))
        );
        assert_eq!(
            colored_level(Level::Warn),
            style::Style::new().fg_color(Some(style::AnsiColor::Yellow.into()))
        );
        assert_eq!(
            colored_level(Level::Error),
            style::Style::new().fg_color(Some(style::AnsiColor::Red.into()))
        );
    }

    #[test]
    #[ignore]
    fn test_set_env_logger() {
        env::set_var("RUST_LOG", "trace");
        set_env_logger();
        trace!("this is trace")
    }
}
