use failure;
use graphql_client_codegen::{generate_module_token_stream, GraphQLClientCodegenOptions};
use std::fs::File;
use std::io::Write as _;
use std::path::PathBuf;
use syn;

pub(crate) struct CliCodegenParams {
    pub query_path: PathBuf,
    pub schema_path: PathBuf,
    pub selected_operation: Option<String>,
    pub additional_derives: Option<String>,
    pub deprecation_strategy: Option<String>,
    pub no_formatting: bool,
    pub module_visibility: Option<String>,
    pub output: PathBuf,
}

pub(crate) fn generate_code(params: CliCodegenParams) -> Result<(), failure::Error> {
    let deprecation_strategy = params
        .deprecation_strategy
        .as_ref()
        .and_then(|s| s.parse().ok());

    let mut options = GraphQLClientCodegenOptions::new_default();

    // options.set_module_name(module_name);
    options.set_module_visibility(
        syn::VisPublic {
            pub_token: <Token![pub]>::default(),
        }
        .into(),
    );

    if let Some(selected_operation) = params.selected_operation {
        options.set_operation_name(selected_operation);
    }

    if let Some(additional_derives) = params.additional_derives {
        options.set_additional_derives(additional_derives);
    }

    if let Some(deprecation_strategy) = deprecation_strategy {
        options.set_deprecation_strategy(deprecation_strategy);
    }

    let gen = generate_module_token_stream(params.query_path, &params.schema_path, options)?;

    let generated_code = gen.to_string();
    let generated_code = if cfg!(feature = "rustfmt") && !params.no_formatting {
        format(&generated_code)
    } else {
        generated_code
    };

    let mut file = File::create(params.output)?;

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
