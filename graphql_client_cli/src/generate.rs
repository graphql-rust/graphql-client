use anyhow::*;
use graphql_client_codegen::{
    generate_module_token_stream, CodegenMode, GraphQLClientCodegenOptions,
};
use std::fs::File;
use std::io::Write as _;
use std::path::PathBuf;
use syn::Token;

pub(crate) struct CliCodegenParams {
    pub query_path: PathBuf,
    pub schema_path: PathBuf,
    pub selected_operation: Option<String>,
    pub variables_derives: Option<String>,
    pub response_derives: Option<String>,
    pub deprecation_strategy: Option<String>,
    pub no_formatting: bool,
    pub module_visibility: Option<String>,
    pub output_directory: Option<PathBuf>,
}

pub(crate) fn generate_code(params: CliCodegenParams) -> Result<(), anyhow::Error> {
    let CliCodegenParams {
        variables_derives,
        response_derives,
        deprecation_strategy,
        no_formatting,
        output_directory,
        module_visibility: _module_visibility,
        query_path,
        schema_path,
        selected_operation,
    } = params;

    let deprecation_strategy = deprecation_strategy.as_ref().and_then(|s| s.parse().ok());

    let mut options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);

    options.set_module_visibility(
        syn::VisPublic {
            pub_token: <Token![pub]>::default(),
        }
        .into(),
    );

    if let Some(selected_operation) = selected_operation {
        options.set_operation_name(selected_operation);
    }

    if let Some(variables_derives) = variables_derives {
        options.set_variables_derives(variables_derives);
    }

    if let Some(response_derives) = response_derives {
        options.set_response_derives(response_derives);
    }

    if let Some(deprecation_strategy) = deprecation_strategy {
        options.set_deprecation_strategy(deprecation_strategy);
    }

    let gen = generate_module_token_stream(query_path.clone(), &schema_path, options)?;

    let generated_code = gen.to_string();
    let generated_code = if cfg!(feature = "rustfmt") && !no_formatting {
        format(&generated_code)
    } else {
        generated_code
    };

    let query_file_name: ::std::ffi::OsString = query_path
        .file_name()
        .map(ToOwned::to_owned)
        .ok_or_else(|| format_err!("Failed to find a file name in the provided query path."))?;

    let dest_file_path: PathBuf = output_directory
        .map(|output_dir| output_dir.join(query_file_name).with_extension("rs"))
        .unwrap_or_else(move || query_path.with_extension("rs"));

    let mut file = File::create(dest_file_path)?;
    write!(file, "{}", generated_code)?;

    Ok(())
}

#[allow(unused_variables)]
fn format(codes: &str) -> String {
    #[cfg(feature = "rustfmt")]
    {
        use rustfmt::{Config, Input, Session};

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
