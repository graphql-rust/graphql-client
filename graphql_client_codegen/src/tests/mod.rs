use std::path::PathBuf;

use crate::{generate_module_token_stream_from_string, CodegenMode, GraphQLClientCodegenOptions};

const KEYWORDS_QUERY: &str = include_str!("keywords_query.graphql");
const KEYWORDS_SCHEMA_PATH: &str = "keywords_schema.graphql";

const FOOBARS_QUERY: &str = include_str!("foobars_query.graphql");
const FOOBARS_SCHEMA_PATH: &str = "foobars_schema.graphql";

fn build_schema_path(path: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("src/tests")
        .join(path)
}

#[test]
fn schema_with_keywords_works() {
    let query_string = KEYWORDS_QUERY;
    let schema_path = build_schema_path(KEYWORDS_SCHEMA_PATH);

    let options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);

    let generated_tokens =
        generate_module_token_stream_from_string(query_string, &schema_path, options)
            .expect("Generate keywords module");

    let generated_code = generated_tokens.to_string();

    // Parse generated code. All keywords should be correctly escaped.
    let r: syn::parse::Result<proc_macro2::TokenStream> = syn::parse2(generated_tokens);
    match r {
        Ok(_) => {
            // Rust keywords should be escaped / renamed now
            assert!(generated_code.contains("pub in_"));
            assert!(generated_code.contains("extern_"));
        }
        Err(e) => {
            panic!("Error: {}\n Generated content: {}\n", e, &generated_code);
        }
    };
}

#[test]
fn blended_custom_types_works() {
    let query_string = KEYWORDS_QUERY;
    let schema_path = build_schema_path(KEYWORDS_SCHEMA_PATH);

    let mut options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);
    options.set_custom_response_type("external_crate::Transaction".to_string());
    options.set_custom_variable_types(vec!["external_crate::ID".to_string()]);

    let generated_tokens =
        generate_module_token_stream_from_string(query_string, &schema_path, options)
            .expect("Generate keywords module");

    let generated_code = generated_tokens.to_string();
    eprintln!("{}", generated_code);

    // Parse generated code. Variables and returns should be replaced with custom types
    let r: syn::parse::Result<proc_macro2::TokenStream> = syn::parse2(generated_tokens);
    match r {
        Ok(_) => {
            // Variables and returns should be replaced with custom types
            assert!(generated_code.contains("pub type ResponseData = external_crate :: Transaction"));
            assert!(generated_code.contains("pub type extern_ = external_crate :: ID"));
        }
        Err(e) => {
            panic!("Error: {}\n Generated content: {}\n", e, &generated_code);
        }
    };
}

#[test]
fn fragments_other_variant_should_generate_unknown_other_variant() {
    let query_string = FOOBARS_QUERY;
    let schema_path = build_schema_path(FOOBARS_SCHEMA_PATH);

    let mut options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);

    options.set_fragments_other_variant(true);

    let generated_tokens =
        generate_module_token_stream_from_string(query_string, &schema_path, options)
            .expect("Generate foobars module");

    let generated_code = generated_tokens.to_string();

    let r: syn::parse::Result<proc_macro2::TokenStream> = syn::parse2(generated_tokens);
    match r {
        Ok(_) => {
            // Rust keywords should be escaped / renamed now
            assert!(generated_code.contains("# [serde (other)] Unknown"));
            assert!(generated_code.contains("Unknown"));
        }
        Err(e) => {
            panic!("Error: {}\n Generated content: {}\n", e, &generated_code);
        }
    };
}

#[test]
fn fragments_other_variant_false_should_not_generate_unknown_other_variant() {
    let query_string = FOOBARS_QUERY;
    let schema_path = build_schema_path(FOOBARS_SCHEMA_PATH);

    let mut options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);

    options.set_fragments_other_variant(false);

    let generated_tokens =
        generate_module_token_stream_from_string(query_string, &schema_path, options)
            .expect("Generate foobars module token stream");

    let generated_code = generated_tokens.to_string();

    let r: syn::parse::Result<proc_macro2::TokenStream> = syn::parse2(generated_tokens);
    match r {
        Ok(_) => {
            // Rust keywords should be escaped / renamed now
            assert!(!generated_code.contains("# [serde (other)] Unknown"));
            assert!(!generated_code.contains("Unknown"));
        }
        Err(e) => {
            panic!("Error: {}\n Generated content: {}\n", e, &generated_code);
        }
    };
}

#[test]
fn skip_serializing_none_should_generate_serde_skip_serializing() {
    let query_string = KEYWORDS_QUERY;
    let schema_path = build_schema_path(KEYWORDS_SCHEMA_PATH);

    let mut options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);

    options.set_skip_serializing_none(true);

    let generated_tokens =
        generate_module_token_stream_from_string(query_string, &schema_path, options)
            .expect("Generate foobars module");

    let generated_code = generated_tokens.to_string();

    let r: syn::parse::Result<proc_macro2::TokenStream> = syn::parse2(generated_tokens);

    match r {
        Ok(_) => {
            println!("{}", generated_code);
            assert!(generated_code.contains("skip_serializing_if"));
        }
        Err(e) => {
            panic!("Error: {}\n Generated content: {}\n", e, &generated_code);
        }
    };
}
