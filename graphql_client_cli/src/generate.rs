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
    no_formatting: bool,
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

    let mut file = File::create(output.clone())?;

    let codes = gen.to_string();

    if cfg!(feature = "rustfmt") && !no_formatting {
        let codes = format(&codes);
        write!(file, "{}", codes);
    } else {
        write!(file, "{}", codes);
    }

    Ok(())
}

#[allow(unused_variables)]
fn format(codes: &str) -> String {
    #[cfg(feature = "rustfmt")]
    {
        use rustfmt::{Config, Input, Session};
        use std::default::Default;

        let mut config = Config::default();

        config.set().emit_mode(rustfmt_nightly::EmitMode::Stdout);
        config.set().verbose(rustfmt_nightly::Verbosity::Quiet);

        let mut out = Vec::with_capacity(codes.len() * 2);

        Session::new(config, Some(&mut out))
            .format(Input::Text(codes.to_string()))
            .unwrap_or_else(|err| panic!("rustfmt error: {}", err));

        return String::from_utf8(out).unwrap();
    }
    #[cfg(not(feature = "rustfmt"))]
    unreachable!()
}
