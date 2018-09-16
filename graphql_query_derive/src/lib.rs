extern crate graphql_client_codegen;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
use graphql_client_codegen::*;

use proc_macro2::TokenStream;

#[proc_macro_derive(GraphQLQuery, attributes(graphql))]
pub fn graphql_query_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let ast = syn::parse2(input).expect("Derive input is well formed");
    let (query_path, schema_path) = build_query_and_schema_path(&ast);
    let option = GraphQLClientDeriveOptions { input: &ast };
    let gen = generate_module_token_stream(query_path, schema_path, Some(option)).unwrap();
    gen.into()
}

fn build_query_and_schema_path(
    input: &syn::DeriveInput,
) -> (std::path::PathBuf, std::path::PathBuf) {
    let cargo_manifest_dir =
        ::std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env variable is defined");

    let query_path = attributes::extract_attr(input, "query_path").unwrap();
    let query_path = format!("{}/{}", cargo_manifest_dir, query_path);
    let query_path = ::std::path::Path::new(&query_path).to_path_buf();
    let schema_path = attributes::extract_attr(input, "schema_path").unwrap();
    let schema_path = ::std::path::Path::new(&cargo_manifest_dir).join(schema_path);
    (query_path, schema_path)
}
