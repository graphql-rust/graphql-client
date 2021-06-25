use graphql_client_codegen::{
    generate_module_token_stream, CodegenMode, GraphQLClientCodegenOptions,
};
use std::fs::File;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::Stdio;
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
    pub custom_scalars_module: Option<String>,
}

pub(crate) fn generate_code(params: CliCodegenParams) {
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
        custom_scalars_module,
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

    if let Some(custom_scalars_module) = custom_scalars_module {
        let custom_scalars_module =
            syn::parse_str(&custom_scalars_module).expect("Invalid custom scalar module path");

        options.set_custom_scalars_module(custom_scalars_module);
    }

    let gen = generate_module_token_stream(query_path.clone(), &schema_path, options).unwrap();

    let generated_code = gen.to_string();
    let generated_code = if !no_formatting {
        format(&generated_code)
    } else {
        generated_code
    };

    let query_file_name: ::std::ffi::OsString = query_path
        .file_name()
        .map(ToOwned::to_owned)
        .ok_or_else(|| "Failed to find a file name in the provided query path.".to_owned())
        .unwrap();

    let dest_file_path: PathBuf = output_directory
        .map(|output_dir| output_dir.join(query_file_name).with_extension("rs"))
        .unwrap_or_else(move || query_path.with_extension("rs"));

    log::info!("Writing generated query to {:?}", dest_file_path);

    let mut file = File::create(dest_file_path).unwrap();
    write!(file, "{}", generated_code).unwrap();
}

fn format(code: &str) -> String {
    let binary = "rustfmt";

    let mut child = std::process::Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let child_stdin = child.stdin.as_mut().unwrap();
    write!(child_stdin, "{}", code).unwrap();

    let output = child.wait_with_output().unwrap();

    if !output.status.success() {
        panic!(
            "rustfmt error\n\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8(output.stdout).unwrap()
}
