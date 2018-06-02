#![recursion_limit = "128"]

#[macro_use]
extern crate failure;
extern crate graphql_parser;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;
use std::collections::BTreeMap;

mod enums;
mod field_type;
mod inputs;
mod interfaces;
mod objects;
mod query;
mod schema;

use schema::Schema;

struct DeriveContext {
    schema: Schema,
    structs: BTreeMap<String, TokenStream>,
    enums: BTreeMap<String, TokenStream>,
    inputs: BTreeMap<String, TokenStream>,
}

#[proc_macro_derive(GraphQLQuery)]
pub fn hello_world_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let ast = syn::parse2(input).expect("Derive input is well formed");
    let gen = impl_gql_query(&ast).unwrap();
    gen.into()
}

fn impl_gql_query(input: &syn::DeriveInput) -> Result<TokenStream, failure::Error> {
    use std::io::prelude::*;

    let query_path = extract_attr(input, "query_path")?;
    let schema_path = extract_attr(input, "schema_path")?;
    let mut query = String::new();
    ::std::fs::File::open(query_path)?.read_to_string(&mut query)?;
    let query = graphql_parser::parse_query(&query)?;

    let mut schema = String::new();
    ::std::fs::File::open(schema_path)?.read_to_string(&mut schema)?;
    let schema = graphql_parser::schema::parse_schema(&schema)?;
    let schema = schema::Schema::from(schema);

    Ok(quote!())
}

fn extract_attr(ast: &syn::DeriveInput, attr: &str) -> Result<String, failure::Error> {
    let attributes = &ast.attrs;
    let attribute = attributes
        .iter()
        .find(|attr| quote!(attr.path).to_string() == "GraphQLQuery")
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
