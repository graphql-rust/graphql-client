use failure;
use graphql_client_codegen::*;
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::PathBuf;

pub fn generate_code(
    query_path: PathBuf,
    schema_path: PathBuf,
    selected_operation: String,
    additional_derives: Option<String>,
    deprecation_strategy: Option<String>,
    output: PathBuf,
) -> Result<(), failure::Error> {
    let deprecation_strategy = deprecation_strategy.as_ref().map(|s| s.as_str());
    let deprecation_strategy = match deprecation_strategy {
        Some("allow") => Some(deprecation::DeprecationStrategy::Allow),
        Some("deny") => Some(deprecation::DeprecationStrategy::Deny),
        Some("warn") => Some(deprecation::DeprecationStrategy::Warn),
        _ => None,
    };

    let options = GraphQLClientDeriveOptions {
        struct_name: selected_operation,
        additional_derives,
        deprecation_strategy,
    };
    let gen = generate_module_token_stream(query_path, schema_path, Some(options))?;
    let mut file = File::create(output)?;
    write!(file, "{}", gen.to_string());
    Ok(())
}
