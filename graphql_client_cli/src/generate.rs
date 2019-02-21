use failure;
use graphql_client_codegen::{
    deprecation, generate_module_token_stream, GraphQLClientCodegenOptions,
};
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use syn;

#[allow(clippy::too_many_arguments)]
pub fn generate_code(
    query_path: PathBuf,
    schema_path: &Path,
    module_name: String,
    selected_operation: Option<String>,
    additional_derives: Option<String>,
    deprecation_strategy: Option<&str>,
    no_formatting: bool,
    module_visibility: Option<&str>,
    output: &Path,
) -> Result<(), failure::Error> {
    let deprecation_strategy = match deprecation_strategy {
        Some("allow") => Some(deprecation::DeprecationStrategy::Allow),
        Some("deny") => Some(deprecation::DeprecationStrategy::Deny),
        Some("warn") => Some(deprecation::DeprecationStrategy::Warn),
        _ => None,
    };

    let module_visibility = match module_visibility {
        Some("pub") => syn::VisPublic {
            pub_token: <Token![pub]>::default(),
        }
        .into(),
        Some("private") => syn::Visibility::Inherited {},
        _ => syn::VisPublic {
            pub_token: <Token![pub]>::default(),
        }
        .into(),
    };

    let mut options = GraphQLClientCodegenOptions::new_default();

    options.set_module_name(module_name);
    options.set_module_visibility(module_visibility);

    if let Some(selected_operation) = selected_operation {
        options.set_operation_name(selected_operation);
    }

    if let Some(additional_derives) = additional_derives {
        options.set_additional_derives(additional_derives);
    }

    if let Some(deprecation_strategy) = deprecation_strategy {
        options.set_deprecation_strategy(deprecation_strategy);
    }

    let gen = generate_module_token_stream(query_path, &schema_path, options)?;

    let generated_code = gen.to_string();
    let generated_code = if cfg!(feature = "rustfmt") && !no_formatting {
        format(&generated_code)
    } else {
        generated_code
    };

    let mut file = File::create(output)?;

    write!(file, "{}", generated_code)?;

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
