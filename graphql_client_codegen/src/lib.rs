#![recursion_limit = "512"]

#[macro_use]
extern crate failure;
extern crate graphql_parser;
extern crate heck;
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

pub mod attributes;
pub mod codegen;
pub mod deprecation;
pub mod introspection_response;
pub mod query;
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

pub struct GraphQLClientDeriveOptions<'a> {
    pub input: &'a syn::DeriveInput,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FullResponse<T> {
    data: T,
}

pub fn generate_module_token_stream(
    query_path: std::path::PathBuf,
    schema_path: std::path::PathBuf,
    options: Option<GraphQLClientDeriveOptions>,
) -> Result<TokenStream, failure::Error> {
    let input = options.unwrap().input;

    let response_derives = attributes::extract_attr(input, "response_derives").ok();

    // The user can determine what to do about deprecations.
    let deprecation_strategy = deprecation::extract_deprecation_strategy(input)
        .unwrap_or(deprecation::DeprecationStrategy::Warn);

    // We need to qualify the query with the path to the crate it is part of
    let query_string = read_file(&query_path)?;
    let query = graphql_parser::parse_query(&query_string)?;

    // We need to qualify the schema with the path to the crate it is part of
    let schema_string = read_file(&schema_path)?;

    let extension = schema_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("INVALID");

    let schema = match extension {
        "graphql" | "gql" => {
            let s = graphql_parser::schema::parse_schema(&schema_string)?;
            schema::Schema::from(s)
        }
        "json" => {
            let parsed: FullResponse<introspection_response::IntrospectionResponse> = ::serde_json::from_str(&schema_string)?;
            schema::Schema::from(parsed.data)
        }
        extension => panic!("Unsupported extension for the GraphQL schema: {} (only .json and .graphql are supported)", extension)
    };

    let module_name = Ident::new(&input.ident.to_string().to_snake_case(), Span::call_site());
    let struct_name = &input.ident;
    let schema_output = codegen::response_for_query(
        schema,
        query,
        input.ident.to_string(),
        response_derives,
        deprecation_strategy,
    )?;

    let result = quote!(
        pub mod #module_name {
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(dead_code)]

            use serde;

            pub const QUERY: &'static str = #query_string;

            #schema_output
        }

        impl<'de> ::graphql_client::GraphQLQuery<'de> for #struct_name {
            type Variables = #module_name::Variables;
            type ResponseData = #module_name::ResponseData;

            fn build_query(variables: Self::Variables) -> ::graphql_client::GraphQLQueryBody<Self::Variables> {
                ::graphql_client::GraphQLQueryBody {
                    variables,
                    query: #module_name::QUERY,
                }

            }
        }
    );

    Ok(result)
}

fn read_file(
    path: impl AsRef<::std::path::Path> + ::std::fmt::Debug,
) -> Result<String, failure::Error> {
    use std::io::prelude::*;

    let mut out = String::new();
    let mut file = ::std::fs::File::open(&path).map_err(|io_err| {
        let err: failure::Error = io_err.into();
        err.context(format!(
            r#"
            Could not find file with path: {:?}
            Hint: file paths in the GraphQLQuery attribute are relative to the project root (location of the Cargo.toml). Example: query_path = "src/my_query.graphql".
            "#,
            path
        ))
    })?;
    file.read_to_string(&mut out)?;
    Ok(out)
}
