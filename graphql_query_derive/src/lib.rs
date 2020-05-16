extern crate proc_macro;

/// Derive-related code. This will be moved into graphql_query_derive.
mod attributes;

use graphql_client_codegen::{
    generate_module_token_stream, CodegenMode, GraphQLClientCodegenOptions,
};
use std::{
    env,
    fmt::Display,
    path::{Path, PathBuf},
};

use proc_macro2::TokenStream;

type BoxError = Box<dyn std::error::Error + 'static>;

#[derive(Debug)]
struct GeneralError(String);

impl Display for GeneralError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for GeneralError {}

#[proc_macro_derive(GraphQLQuery, attributes(graphql))]
pub fn derive_graphql_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match graphql_query_derive_inner(input) {
        Ok(ts) => ts,
        Err(err) => panic!("{:?}", err),
    }
}

fn graphql_query_derive_inner(
    input: proc_macro::TokenStream,
) -> Result<proc_macro::TokenStream, BoxError> {
    let input = TokenStream::from(input);
    let ast = syn::parse2(input)?;
    let (query_path, schema_path) = build_query_and_schema_path(&ast)?;
    let options = build_graphql_client_derive_options(&ast, query_path.clone())?;
    Ok(
        generate_module_token_stream(query_path, &schema_path, options)
            .map(Into::into)
            .map_err(|err| GeneralError(format!("Code generation failed: {}", err)))?,
    )
}

fn build_query_and_schema_path(input: &syn::DeriveInput) -> Result<(PathBuf, PathBuf), BoxError> {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").map_err(|_err| {
        GeneralError("Checking that the CARGO_MANIFEST_DIR env variable is defined.".into())
    })?;

    let query_path = attributes::extract_attr(input, "query_path")
        .map_err(|err| GeneralError(format!("Error extracting query path. {}", err)))?;
    let query_path = format!("{}/{}", cargo_manifest_dir, query_path);
    let query_path = Path::new(&query_path).to_path_buf();
    let schema_path = attributes::extract_attr(input, "schema_path")
        .map_err(|err| GeneralError(format!("Error extracting schema path. {}", err)))?;
    let schema_path = Path::new(&cargo_manifest_dir).join(schema_path);
    Ok((query_path, schema_path))
}

fn build_graphql_client_derive_options(
    input: &syn::DeriveInput,
    query_path: PathBuf,
) -> Result<GraphQLClientCodegenOptions, BoxError> {
    let variables_derives = attributes::extract_attr(input, "variables_derives").ok();
    let response_derives = attributes::extract_attr(input, "response_derives").ok();

    let mut options = GraphQLClientCodegenOptions::new(CodegenMode::Derive);
    options.set_query_file(query_path);

    if let Some(variables_derives) = variables_derives {
        options.set_variables_derives(variables_derives);
    };

    if let Some(response_derives) = response_derives {
        options.set_response_derives(response_derives);
    };

    // The user can determine what to do about deprecations.
    if let Ok(deprecation_strategy) = attributes::extract_deprecation_strategy(input) {
        options.set_deprecation_strategy(deprecation_strategy);
    };

    // The user can specify the normalization strategy.
    if let Ok(normalization) = attributes::extract_normalization(input) {
        options.set_normalization(normalization);
    };

    options.set_struct_ident(input.ident.clone());
    options.set_module_visibility(input.vis.clone());
    options.set_operation_name(input.ident.to_string());

    Ok(options)
}
