#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![allow(clippy::option_option)]

//! Crate for internal use by other graphql-client crates, for code generation.
//!
//! It is not meant to be used directly by users of the library.

use lazy_static::*;
use proc_macro2::TokenStream;
use quote::*;

mod codegen;
mod codegen_options;
/// Deprecation-related code
pub mod deprecation;
/// Contains the [Schema] type and its implementation.
pub mod schema;

mod constants;
mod generated_module;
/// Normalization-related code
pub mod normalization;
mod query;
mod type_qualifiers;

#[cfg(test)]
mod tests;

pub use crate::codegen_options::{CodegenMode, GraphQLClientCodegenOptions};

use std::{collections::HashMap, io};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}")]
struct GeneralError(String);

type BoxError = Box<dyn std::error::Error + 'static>;
type CacheMap<T> = std::sync::Mutex<HashMap<std::path::PathBuf, T>>;

lazy_static! {
    static ref SCHEMA_CACHE: CacheMap<schema::Schema> = CacheMap::default();
    static ref QUERY_CACHE: CacheMap<(String, graphql_parser::query::Document)> =
        CacheMap::default();
}

/// Generates Rust code given a query document, a schema and options.
pub fn generate_module_token_stream(
    query_path: std::path::PathBuf,
    schema_path: &std::path::Path,
    options: GraphQLClientCodegenOptions,
) -> Result<TokenStream, BoxError> {
    use std::collections::hash_map;

    let schema_extension = schema_path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("INVALID");

    // Check the schema cache.
    let schema: schema::Schema = {
        let mut lock = SCHEMA_CACHE.lock().expect("schema cache is poisoned");
        match lock.entry(schema_path.to_path_buf()) {
            hash_map::Entry::Occupied(o) => o.get().clone(),
            hash_map::Entry::Vacant(v) => {
                let schema_string = read_file(v.key())?;
                let schema = match schema_extension {
                    "graphql" | "gql" => {
                        let s = graphql_parser::schema::parse_schema(&schema_string).map_err(|parser_error| GeneralError(format!("Parser error: {}", parser_error)))?;
                        schema::Schema::from(s)
                    }
                    "json" => {
                        let parsed: graphql_introspection_query::introspection_response::IntrospectionResponse = serde_json::from_str(&schema_string)?;
                        schema::Schema::from(parsed)
                    }
                    extension => return Err(GeneralError(format!("Unsupported extension for the GraphQL schema: {} (only .json and .graphql are supported)", extension)).into())
                };

                v.insert(schema).clone()
            }
        }
    };

    // We need to qualify the query with the path to the crate it is part of
    let (query_string, query) = {
        let mut lock = QUERY_CACHE.lock().expect("query cache is poisoned");
        match lock.entry(query_path) {
            hash_map::Entry::Occupied(o) => o.get().clone(),
            hash_map::Entry::Vacant(v) => {
                let query_string = read_file(v.key())?;
                let query = graphql_parser::parse_query(&query_string)
                    .map_err(|err| GeneralError(format!("Query parser error: {}", err)))?;
                v.insert((query_string, query)).clone()
            }
        }
    };

    let query = crate::query::resolve(&schema, &query)?;

    // Determine which operation we are generating code for. This will be used in operationName.
    let operations = options
        .operation_name
        .as_ref()
        .and_then(|operation_name| query.select_operation(operation_name, *options.normalization()))
        .map(|op| vec![op]);

    let operations = match (operations, &options.mode) {
        (Some(ops), _) => ops,
        (None, &CodegenMode::Cli) => query.operations().collect(),
        (None, &CodegenMode::Derive) => {
            return Err(GeneralError(derive_operation_not_found_error(
                options.struct_ident(),
                &query,
            ))
            .into());
        }
    };

    // The generated modules.
    let mut modules = Vec::with_capacity(operations.len());

    for operation in &operations {
        let generated = generated_module::GeneratedModule {
            query_string: query_string.as_str(),
            schema: &schema,
            resolved_query: &query,
            operation: &operation.1.name,
            options: &options,
        }
        .to_token_stream()?;
        modules.push(generated);
    }

    let modules = quote! { #(#modules)* };

    Ok(modules)
}

#[derive(Debug, Error)]
enum ReadFileError {
    #[error(
        "Could not find file with path: {}\
        Hint: file paths in the GraphQLQuery attribute are relative to the project root (location of the Cargo.toml). Example: query_path = \"src/my_query.graphql\".",
        path
    )]
    FileNotFound {
        path: String,
        #[source]
        io_error: io::Error,
    },
    #[error("Error reading file at: {}", path)]
    ReadError {
        path: String,
        #[source]
        io_error: io::Error,
    },
}

fn read_file(path: &std::path::Path) -> Result<String, ReadFileError> {
    use std::fs;
    use std::io::prelude::*;

    let mut out = String::new();
    let mut file = fs::File::open(path).map_err(|io_error| ReadFileError::FileNotFound {
        io_error,
        path: path.display().to_string(),
    })?;

    file.read_to_string(&mut out)
        .map_err(|io_error| ReadFileError::ReadError {
            io_error,
            path: path.display().to_string(),
        })?;
    Ok(out)
}

/// In derive mode, build an error when the operation with the same name as the struct is not found.
fn derive_operation_not_found_error(
    ident: Option<&proc_macro2::Ident>,
    query: &crate::query::Query,
) -> String {
    let operation_name = ident.map(ToString::to_string);
    let struct_ident = operation_name.as_deref().unwrap_or("");

    let available_operations: Vec<&str> = query
        .operations()
        .map(|(_id, op)| op.name.as_str())
        .collect();
    let available_operations: String = available_operations.join(", ");

    return format!(
        "The struct name does not match any defined operation in the query file.\nStruct name: {}\nDefined operations: {}",
        struct_ident,
        available_operations,
    );
}
