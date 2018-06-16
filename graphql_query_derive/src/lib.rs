#![recursion_limit = "128"]
#![feature(nll)]

#[macro_use]
extern crate failure;
extern crate graphql_parser;
extern crate heck;
extern crate proc_macro;
extern crate proc_macro2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;

mod enums;
mod field_type;
mod fragments;
mod inputs;
mod interfaces;
mod introspection_response;
mod objects;
mod query;
mod schema;
mod shared;
mod unions;

use heck::*;
use proc_macro2::{Ident, Span};

#[proc_macro_derive(GraphQLQuery, attributes(GraphQLQuery))]
pub fn graphql_query_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let ast = syn::parse2(input).expect("Derive input is well formed");
    let gen = impl_gql_query(&ast).unwrap();
    gen.into()
}

fn impl_gql_query(input: &syn::DeriveInput) -> Result<TokenStream, failure::Error> {
    use std::io::prelude::*;

    let cargo_manifest_dir =
        ::std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env variable is defined");

    let query_path = extract_attr(input, "query_path")?;
    let schema_path = extract_attr(input, "schema_path")?;

    // We need to qualify the query with the path to the crate it is part of
    let query_path = format!("{}/{}", cargo_manifest_dir, query_path);
    let mut query_string = String::new();
    ::std::fs::File::open(query_path)?.read_to_string(&mut query_string)?;
    let query = graphql_parser::parse_query(&query_string)?;

    // We need to qualify the schema with the path to the crate it is part of
    let schema_path = format!("{}/{}", cargo_manifest_dir, schema_path);
    let mut schema_string = String::new();
    ::std::fs::File::open(schema_path)?.read_to_string(&mut schema_string)?;
    let schema = graphql_parser::schema::parse_schema(&schema_string)?;
    let schema = schema::Schema::from(schema);

    let module_name = Ident::new(&input.ident.to_string().to_snake_case(), Span::call_site());
    let struct_name = &input.ident;
    let schema_output = schema.response_for_query(query)?;

    let result = quote!(
        mod #module_name {
            #![allow(non_camel_case_types)]
            #![allow(non_snake_case)]
            #![allow(dead_code)]

            pub const QUERY: &'static str = #query_string;

            #schema_output
        }

        impl<'de> ::graphql_query::GraphQLQuery<'de> for #struct_name {
            type Variables = #module_name::Variables;
            type ResponseData = #module_name::ResponseData;

            fn build_query(variables: Self::Variables) -> ::graphql_query::GraphQLQueryBody<Self::Variables> {
                ::graphql_query::GraphQLQueryBody {
                    variables,
                    query: #module_name::QUERY,
                }

            }
        }
    );

    Ok(result)
}

fn extract_attr(ast: &syn::DeriveInput, attr: &str) -> Result<String, failure::Error> {
    let attributes = &ast.attrs;
    let attribute = attributes
        .iter()
        .find(|attr| {
            let path = &attr.path;
            quote!(#path).to_string() == "GraphQLQuery"
        })
        .ok_or(format_err!("The GraphQLQuery attribute is missing"))?;
    if let syn::Meta::List(items) = &attribute
        .interpret_meta()
        .expect("Attribute is well formatted")
    {
        for item in items.nested.iter() {
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = item {
                let syn::MetaNameValue {
                    ident,
                    eq_token: _,
                    lit,
                } = name_value;
                if ident == &attr {
                    if let syn::Lit::Str(lit) = lit {
                        return Ok(lit.value());
                    }
                }
            }
        }
    }

    Err(format_err!("attribute not found"))?
}
