use crate::{generated_module, schema::Schema, CodegenMode, GraphQLClientCodegenOptions};

#[test]
fn schema_with_keywords_works() {
    let query_string = include_str!("keywords_query.graphql");
    let query = graphql_parser::parse_query(query_string).expect("Parse keywords query");
    let schema = graphql_parser::parse_schema(include_str!("keywords_schema.graphql"))
        .expect("Parse keywords schema");
    let schema = Schema::from(schema);

    let options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);
    let query = crate::query::resolve(&schema, &query).unwrap();

    for (_id, operation) in query.operations() {
        let generated_tokens = generated_module::GeneratedModule {
            query_string,
            schema: &schema,
            operation: &operation.name,
            resolved_query: &query,
            options: &options,
        }
        .to_token_stream()
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
}

#[test]
fn fragments_other_variant_should_generate_unknown_other_variant() {
    let query_string = include_str!("foobars_query.graphql");
    let query = graphql_parser::parse_query(query_string).expect("Parse foobars query");
    let schema = graphql_parser::parse_schema(include_str!("foobars_schema.graphql"))
        .expect("Parse foobars schema");
    let schema = Schema::from(schema);

    let mut options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);

    options.set_fragments_other_variant(true);
    let query = crate::query::resolve(&schema, &query).unwrap();

    for (_id, operation) in query.operations() {
        let generated_tokens = generated_module::GeneratedModule {
            query_string,
            schema: &schema,
            operation: &operation.name,
            resolved_query: &query,
            options: &options,
        }
        .to_token_stream()
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
}

#[test]
fn fragments_other_variant_false_should_not_generate_unknown_other_variant() {
    let query_string = include_str!("foobars_query.graphql");
    let query = graphql_parser::parse_query(query_string).expect("Parse foobars query");
    let schema = graphql_parser::parse_schema(include_str!("foobars_schema.graphql"))
        .expect("Parse foobars schema");
    let schema = Schema::from(schema);

    let options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);

    let query = crate::query::resolve(&schema, &query).unwrap();

    for (_id, operation) in query.operations() {
        let generated_tokens = generated_module::GeneratedModule {
            query_string,
            schema: &schema,
            operation: &operation.name,
            resolved_query: &query,
            options: &options,
        }
        .to_token_stream()
        .expect("Generate foobars module");
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
}
