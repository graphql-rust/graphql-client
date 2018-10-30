#![recursion_limit = "512"]
#![deny(missing_docs)]

//! Crate for internal use by other graphql-client crates, for code generation.
//!
//! It is not meant to be used directly by users of the library.

#[macro_use]
extern crate failure;
extern crate graphql_parser;
extern crate heck;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
extern crate proc_macro2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;
use syn::Visibility;

/// Derive-related code. This will be moved into graphql_query_derive.
pub mod attributes;
mod codegen;
/// Deprecation-related code
pub mod deprecation;
mod introspection_response;
mod query;
/// Contains the [Schema] type and its implementation.
pub mod schema;

mod constants;
mod enums;
mod field_type;
mod fragments;
mod inputs;
mod interfaces;
mod objects;
mod operations;
mod scalars;
mod selection;
mod shared;
mod unions;
mod variables;

use heck::SnakeCase;

#[cfg(test)]
mod tests;
use proc_macro2::{Ident, Span};

type CacheMap<T> =
    ::std::sync::Mutex<::std::collections::hash_map::HashMap<::std::path::PathBuf, T>>;

lazy_static! {
    static ref SCHEMA_CACHE: CacheMap<schema::Schema> = CacheMap::default();
    static ref QUERY_CACHE: CacheMap<(String, graphql_parser::query::Document)> =
        CacheMap::default();
}

/// Used to configure code generation.
pub struct GraphQLClientDeriveOptions {
    /// Name of the operation we want to generate code for. If it does not match, we default to the first one.
    pub struct_name: String,
    /// Comma-separated list of additional traits we want to derive.
    pub additional_derives: Option<String>,
    /// The deprecation strategy to adopt.
    pub deprecation_strategy: Option<deprecation::DeprecationStrategy>,
    /// target struct visibility.
    pub module_visibility: Visibility,
}

/// Generates the code for a Rust module given a query, a schema and options.
pub fn generate_module_token_stream(
    query_path: std::path::PathBuf,
    schema_path: std::path::PathBuf,
    options: Option<GraphQLClientDeriveOptions>,
) -> Result<TokenStream, failure::Error> {
    let options = options.unwrap();

    let module_visibility = options.module_visibility;
    let response_derives = options.additional_derives;

    // The user can determine what to do about deprecations.
    let deprecation_strategy = options.deprecation_strategy.unwrap_or_default();

    // We need to qualify the query with the path to the crate it is part of
    let (query_string, query) = {
        let mut lock = QUERY_CACHE.lock().expect("query cache is poisoned");
        match lock.entry(query_path) {
            ::std::collections::hash_map::Entry::Occupied(o) => o.get().clone(),
            ::std::collections::hash_map::Entry::Vacant(v) => {
                let query_string = read_file(v.key())?;
                let query = graphql_parser::parse_query(&query_string)?;
                v.insert((query_string, query)).clone()
            }
        }
    };

    // Determine which operation we are generating code for. This will be used in operationName.

    let operation = if let Some(op) = codegen::select_operation(&query, &options.struct_name) {
        op
    } else {
        panic!("Query document defines no operation.")
    };

    let operation_name_literal = &operation.name;

    // Check the schema cache.
    let schema = {
        let mut lock = SCHEMA_CACHE.lock().expect("schema cache is poisoned");
        match lock.entry(schema_path) {
            ::std::collections::hash_map::Entry::Occupied(o) => o.get().clone(),
            ::std::collections::hash_map::Entry::Vacant(v) => {
                let schema_string = read_file(v.key())?;
                let schema = {
                    let extension = v
                        .key()
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("INVALID");

                    match extension {
                        "graphql" | "gql" => {
                            let s = graphql_parser::schema::parse_schema(&schema_string)?;
                            schema::Schema::from(s)
                        }
                        "json" => {
                            let parsed: introspection_response::IntrospectionResponse = ::serde_json::from_str(&schema_string)?;
                            schema::Schema::from(parsed)
                        }
                        extension => panic!("Unsupported extension for the GraphQL schema: {} (only .json and .graphql are supported)", extension)
                    }
                };

                v.insert(schema).clone()
            }
        }
    };

    let module_name = Ident::new(
        options.struct_name.to_snake_case().as_str(),
        Span::call_site(),
    );
    let struct_name = Ident::new(options.struct_name.as_str(), Span::call_site());
    let schema_output = codegen::response_for_query(
        schema,
        query,
        &operation,
        response_derives,
        deprecation_strategy,
    )?;

    let result = quote!(
        #module_visibility mod #module_name {
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(dead_code)]

            use serde;

            pub const QUERY: &'static str = #query_string;
            pub const OPERATION_NAME: &'static str = #operation_name_literal;

            #schema_output
        }

        impl ::graphql_client::GraphQLQuery for #struct_name {
            type Variables = #module_name::Variables;
            type ResponseData = #module_name::ResponseData;

            fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
                ::graphql_client::QueryBody {
                    variables,
                    query: #module_name::QUERY,
                    operation_name: #module_name::OPERATION_NAME,
                }

            }
        }
    );

    Ok(result)
}

fn read_file(path: &::std::path::Path) -> Result<String, failure::Error> {
    use std::io::prelude::*;

    let mut out = String::new();
    let mut file = ::std::fs::File::open(path).map_err(|io_err| {
        let err: failure::Error = io_err.into();
        err.context(format!(
            r#"
            Could not find file with path: {}
            Hint: file paths in the GraphQLQuery attribute are relative to the project root (location of the Cargo.toml). Example: query_path = "src/my_query.graphql".
            "#,
            path.display()
        ))
    })?;
    file.read_to_string(&mut out)?;
    Ok(out)
}
