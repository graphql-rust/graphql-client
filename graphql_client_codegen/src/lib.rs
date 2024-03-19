#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![allow(clippy::option_option)]

//! Crate for Rust code generation from a GraphQL query, schema, and options.

use lazy_static::*;
use proc_macro2::TokenStream;
use quote::*;
use schema::Schema;

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

use std::{collections::BTreeMap, fmt::Display, io};

#[derive(Debug)]
struct GeneralError(String);

impl Display for GeneralError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for GeneralError {}

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type CacheMap<T> = std::sync::Mutex<BTreeMap<std::path::PathBuf, T>>;
type QueryDocument = graphql_parser::query::Document<'static, String>;

lazy_static! {
    static ref SCHEMA_CACHE: CacheMap<Schema> = CacheMap::default();
    static ref QUERY_CACHE: CacheMap<(String, QueryDocument)> = CacheMap::default();
}

fn get_set_cached<T: Clone>(
    cache: &CacheMap<T>,
    key: &std::path::Path,
    value_func: impl FnOnce() -> T,
) -> T {
    let mut lock = cache.lock().expect("cache is poisoned");
    lock.entry(key.into()).or_insert_with(value_func).clone()
}

fn query_document(query_string: &str) -> Result<QueryDocument, BoxError> {
    let document = graphql_parser::parse_query(query_string)
        .map_err(|err| GeneralError(format!("Query parser error: {}", err)))?
        .into_static();
    Ok(document)
}

fn get_set_query_from_file(query_path: &std::path::Path) -> (String, QueryDocument) {
    get_set_cached(&QUERY_CACHE, query_path, || {
        let query_string = read_file(query_path).unwrap();
        let query_document = query_document(&query_string).unwrap();
        (query_string, query_document)
    })
}

fn get_set_schema_from_file(schema_path: &std::path::Path) -> Schema {
    get_set_cached(&SCHEMA_CACHE, schema_path, || {
        let schema_extension = schema_path
            .extension()
            .map(|ext| ext.to_str().expect("Path must be valid UTF-8"))
            .unwrap_or("<no extension>");
        let schema_string = read_file(schema_path).unwrap();
        match schema_extension {
            "graphql" | "graphqls"| "gql" => {
                let s = graphql_parser::schema::parse_schema::<&str>(&schema_string).map_err(|parser_error| GeneralError(format!("Parser error: {}", parser_error))).unwrap();
                Schema::from(s)
            }
            "json" => {
                let parsed: graphql_introspection_query::introspection_response::IntrospectionResponse = serde_json::from_str(&schema_string).unwrap();
                Schema::from(parsed)
            }
            extension => panic!("Unsupported extension for the GraphQL schema: {} (only .json, .graphql, .graphqls and .gql are supported)", extension)
        }
    })
}

/// Generates Rust code given a path to a query file, a path to a schema file, and options.
pub fn generate_module_token_stream(
    query_path: std::path::PathBuf,
    schema_path: &std::path::Path,
    options: GraphQLClientCodegenOptions,
) -> Result<TokenStream, BoxError> {
    let query = get_set_query_from_file(query_path.as_path());
    let schema = get_set_schema_from_file(schema_path);

    generate_module_token_stream_inner(&query, &schema, options)
}

/// Generates Rust code given a query string, a path to a schema file, and options.
pub fn generate_module_token_stream_from_string(
    query_string: &str,
    schema_path: &std::path::Path,
    options: GraphQLClientCodegenOptions,
) -> Result<TokenStream, BoxError> {
    let query = (query_string.to_string(), query_document(query_string)?);
    let schema = get_set_schema_from_file(schema_path);

    generate_module_token_stream_inner(&query, &schema, options)
}

/// Generates Rust code given a query string and query document, a schema, and options.
fn generate_module_token_stream_inner(
    query: &(String, QueryDocument),
    schema: &Schema,
    options: GraphQLClientCodegenOptions,
) -> Result<TokenStream, BoxError> {
    let (query_string, query_document) = query;

    // We need to qualify the query with the path to the crate it is part of
    let query = crate::query::resolve(schema, query_document)?;

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
            schema,
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

#[derive(Debug)]
enum ReadFileError {
    FileNotFound { path: String, io_error: io::Error },
    ReadError { path: String, io_error: io::Error },
}

impl Display for ReadFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadFileError::FileNotFound { path, .. } => {
                write!(f, "Could not find file with path: {}\n
                Hint: file paths in the GraphQLQuery attribute are relative to the project root (location of the Cargo.toml). Example: query_path = \"src/my_query.graphql\".", path)
            }
            ReadFileError::ReadError { path, .. } => {
                f.write_str("Error reading file at: ")?;
                f.write_str(path)
            }
        }
    }
}

impl std::error::Error for ReadFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReadFileError::FileNotFound { io_error, .. }
            | ReadFileError::ReadError { io_error, .. } => Some(io_error),
        }
    }
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

    format!(
        "The struct name does not match any defined operation in the query file.\nStruct name: {}\nDefined operations: {}",
        struct_ident,
        available_operations,
    )
}
