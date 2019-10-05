mod github;

#[test]
fn schema_with_keywords_works() {
    use crate::{
        codegen, generated_module, schema::Schema, CodegenMode, GraphQLClientCodegenOptions,
    };
    use graphql_parser;

    let query_string = include_str!("keywords_query.graphql");
    let query = graphql_parser::parse_query(query_string).expect("Parse keywords query");
    let schema = graphql_parser::parse_schema(include_str!("keywords_schema.graphql"))
        .expect("Parse keywords schema");
    let schema = Schema::from(&schema);

    let options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);
    let operations = codegen::all_operations(&query);
    for operation in &operations {
        let generated_tokens = generated_module::GeneratedModule {
            query_string,
            schema: &schema,
            query_document: &query,
            operation,
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
