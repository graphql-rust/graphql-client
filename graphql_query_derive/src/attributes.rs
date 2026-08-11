use proc_macro2::{TokenStream, TokenTree};
use std::str::FromStr;
use syn::Meta;

use graphql_client_codegen::deprecation::DeprecationStrategy;
use graphql_client_codegen::normalization::Normalization;

const DEPRECATION_ERROR: &str = "deprecated must be one of 'allow', 'deny', or 'warn'";
const NORMALIZATION_ERROR: &str = "normalization must be one of 'none' or 'rust'";

/// Extract a token stream of the `graphql` attribute.
pub fn extract_tokens(ast: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let attribute = ast
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("graphql"))
        .ok_or_else(|| syn::Error::new_spanned(ast, "The graphql attribute is missing"))?;

    match &attribute.meta {
        Meta::List(list) => Ok(list.tokens.clone()),
        _ => Err(syn::Error::new_spanned(
            ast,
            "Unable to parse the graphql attribute",
        )),
    }
}

pub fn ident_exists(tokens: &TokenStream, ident: &str) -> Result<(), syn::Error> {
    for item in tokens.clone().into_iter() {
        if let TokenTree::Ident(ident_) = item {
            if ident_ == ident {
                return Ok(());
            }
        }
    }

    Err(syn::Error::new_spanned(
        tokens,
        format!("Ident `{}` not found", ident),
    ))
}

/// Extract an configuration parameter specified in the `graphql` attribute.
pub fn extract_attr(tokens: &TokenStream, attr: &str) -> Result<String, syn::Error> {
    let mut iter = tokens.clone().into_iter();
    while let Some(item) = iter.next() {
        if let TokenTree::Ident(ident) = item {
            if ident == attr {
                iter.next();
                if let Some(TokenTree::Literal(lit)) = iter.next() {
                    let lit_str: syn::LitStr = syn::parse_str(&lit.to_string())?;
                    return Ok(lit_str.value());
                }
            }
        }
    }

    Err(syn::Error::new_spanned(
        tokens,
        format!("Attribute `{}` not found", attr),
    ))
}

/// Extract a list of configuration parameter values specified in the `graphql` attribute.
pub fn extract_attr_list(tokens: &TokenStream, attr: &str) -> Result<Vec<String>, syn::Error> {
    let mut result = Vec::new();

    let mut iter = tokens.clone().into_iter();
    while let Some(item) = iter.next() {
        if let TokenTree::Ident(ident) = item {
            if ident == attr {
                if let Some(TokenTree::Group(group)) = iter.next() {
                    for token in group.stream() {
                        if let TokenTree::Literal(lit) = token {
                            let lit_str: syn::LitStr = syn::parse_str(&lit.to_string())?;
                            result.push(lit_str.value());
                        }
                    }
                    return Ok(result);
                }
            }
        }
    }

    if result.is_empty() {
        Err(syn::Error::new_spanned(
            tokens,
            format!("Attribute list `{}` not found or empty", attr),
        ))
    } else {
        Ok(result)
    }
}

/// Get the deprecation from a struct attribute in the derive case.
pub fn extract_deprecation_strategy(
    tokens: &TokenStream,
) -> Result<DeprecationStrategy, syn::Error> {
    extract_attr(tokens, "deprecated")?
        .to_lowercase()
        .as_str()
        .parse()
        .map_err(|_| syn::Error::new_spanned(tokens, DEPRECATION_ERROR.to_owned()))
}

/// Get the deprecation from a struct attribute in the derive case.
pub fn extract_normalization(tokens: &TokenStream) -> Result<Normalization, syn::Error> {
    extract_attr(tokens, "normalization")?
        .to_lowercase()
        .as_str()
        .parse()
        .map_err(|_| syn::Error::new_spanned(tokens, NORMALIZATION_ERROR))
}

pub fn extract_fragments_other_variant(tokens: &TokenStream) -> bool {
    extract_attr(tokens, "fragments_other_variant")
        .ok()
        .and_then(|s| FromStr::from_str(s.as_str()).ok())
        .unwrap_or(false)
}

pub fn extract_skip_serializing_none(tokens: &TokenStream) -> bool {
    ident_exists(tokens, "skip_serializing_none").is_ok()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deprecation_strategy() {
        let input = "
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = \"x\",
            query_path = \"x\",
            deprecated = \"warn\",
        )]
        struct MyQuery;
        ";
        let parsed = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();
        assert_eq!(
            extract_deprecation_strategy(&parsed).unwrap(),
            DeprecationStrategy::Warn
        );
    }

    #[test]
    fn test_deprecation_strategy_is_case_insensitive() {
        let input = "
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = \"x\",
            query_path = \"x\",
            deprecated = \"DeNy\",
        )]
        struct MyQuery;
        ";
        let parsed = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();
        assert_eq!(
            extract_deprecation_strategy(&parsed).unwrap(),
            DeprecationStrategy::Deny
        );
    }

    #[test]
    fn test_invalid_deprecation_strategy() {
        let input = "
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = \"x\",
            query_path = \"x\",
            deprecated = \"foo\",
        )]
        struct MyQuery;
        ";
        let parsed = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();
        match extract_deprecation_strategy(&parsed) {
            Ok(_) => panic!("parsed unexpectedly"),
            Err(e) => assert_eq!(&format!("{}", e), DEPRECATION_ERROR),
        };
    }

    #[test]
    fn test_fragments_other_variant_set_to_true() {
        let input = "
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = \"x\",
            query_path = \"x\",
            fragments_other_variant = \"true\",
        )]
        struct MyQuery;
        ";
        let parsed = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();
        assert!(extract_fragments_other_variant(&parsed));
    }

    #[test]
    fn test_fragments_other_variant_set_to_false() {
        let input = "
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = \"x\",
            query_path = \"x\",
            fragments_other_variant = \"false\",
        )]
        struct MyQuery;
        ";
        let parsed = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();
        assert!(!extract_fragments_other_variant(&parsed));
    }

    #[test]
    fn test_fragments_other_variant_set_to_invalid() {
        let input = "
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = \"x\",
            query_path = \"x\",
            fragments_other_variant = \"invalid\",
        )]
        struct MyQuery;
        ";
        let parsed = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();
        assert!(!extract_fragments_other_variant(&parsed));
    }

    #[test]
    fn test_fragments_other_variant_unset() {
        let input = "
        #[derive(GraphQLQuery)]
        #[graphql(
            schema_path = \"x\",
            query_path = \"x\",
        )]
        struct MyQuery;
        ";
        let parsed = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();
        assert!(!extract_fragments_other_variant(&parsed));
    }

    #[test]
    fn test_skip_serializing_none_set() {
        let input = r#"
            #[derive(GraphQLQuery)]
            #[graphql(
                schema_path = "x",
                query_path = "x",
                skip_serializing_none
            )]
            struct MyQuery;
        "#;
        let parsed = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();
        assert!(extract_skip_serializing_none(&parsed));
    }

    #[test]
    fn test_skip_serializing_none_unset() {
        let input = r#"
            #[derive(GraphQLQuery)]
            #[graphql(
                schema_path = "x",
                query_path = "x",
            )]
            struct MyQuery;
        "#;
        let parsed = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();
        assert!(!extract_skip_serializing_none(&parsed));
    }

    #[test]
    fn test_external_enums() {
        let input = r#"
            #[derive(Serialize, Deserialize, Debug)]
            #[derive(GraphQLQuery)]
            #[graphql(
                schema_path = "x",
                query_path = "x",
                extern_enums("Direction", "DistanceUnit"),
            )]
            struct MyQuery;
        "#;
        let parsed: syn::DeriveInput = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();

        assert_eq!(
            extract_attr_list(&parsed, "extern_enums").ok().unwrap(),
            vec!["Direction", "DistanceUnit"],
        );
    }

    #[test]
    fn test_custom_variable_types() {
        let input = r#"
            #[derive(Serialize, Deserialize, Debug)]
            #[derive(GraphQLQuery)]
            #[graphql(
                schema_path = "x",
                query_path = "x",
                variable_types("extern_crate::Var1", "extern_crate::Var2"),
            )]
            struct MyQuery;
        "#;
        let parsed: syn::DeriveInput = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();

        assert_eq!(
            extract_attr_list(&parsed, "variable_types").ok().unwrap(),
            vec!["extern_crate::Var1", "extern_crate::Var2"],
        );
    }

    #[test]
    fn test_custom_response_type() {
        let input = r#"
            #[derive(Serialize, Deserialize, Debug)]
            #[derive(GraphQLQuery)]
            #[graphql(
                schema_path = "x",
                query_path = "x",
                response_type = "extern_crate::Resp",
            )]
            struct MyQuery;
        "#;
        let parsed: syn::DeriveInput = syn::parse_str(input).unwrap();
        let parsed = extract_tokens(&parsed).unwrap();

        assert_eq!(
            extract_attr(&parsed, "response_type").ok().unwrap(),
            "extern_crate::Resp",
        );
    }
}
