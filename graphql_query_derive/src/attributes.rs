use std::str::FromStr;

use graphql_client_codegen::deprecation::DeprecationStrategy;
use graphql_client_codegen::normalization::Normalization;

const DEPRECATION_ERROR: &str = "deprecated must be one of 'allow', 'deny', or 'warn'";
const NORMALIZATION_ERROR: &str = "normalization must be one of 'none' or 'rust'";

/// The `graphql` attribute as a `syn::Path`.
fn path_to_match() -> syn::Path {
    syn::parse_str("graphql").expect("`graphql` is a valid path")
}

pub fn ident_exists(ast: &syn::DeriveInput, ident: &str) -> Result<(), syn::Error> {
    let graphql_path = path_to_match();
    let attribute = ast
        .attrs
        .iter()
        .find(|attr| attr.path == graphql_path)
        .ok_or_else(|| syn::Error::new_spanned(ast, "The graphql attribute is missing"))?;

    if let syn::Meta::List(items) = &attribute.parse_meta().expect("Attribute is well formatted") {
        for item in items.nested.iter() {
            if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = item {
                if let Some(ident_) = path.get_ident() {
                    if ident_ == ident {
                        return Ok(());
                    }
                }
            }
        }
    }

    Err(syn::Error::new_spanned(
        &ast,
        format!("Ident `{}` not found", ident),
    ))
}

/// Extract an configuration parameter specified in the `graphql` attribute.
pub fn extract_attr(ast: &syn::DeriveInput, attr: &str) -> Result<String, syn::Error> {
    let attributes = &ast.attrs;
    let graphql_path = path_to_match();
    let attribute = attributes
        .iter()
        .find(|attr| attr.path == graphql_path)
        .ok_or_else(|| syn::Error::new_spanned(ast, "The graphql attribute is missing"))?;
    if let syn::Meta::List(items) = &attribute.parse_meta().expect("Attribute is well formatted") {
        for item in items.nested.iter() {
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = item {
                let syn::MetaNameValue { path, lit, .. } = name_value;
                if let Some(ident) = path.get_ident() {
                    if ident == attr {
                        if let syn::Lit::Str(lit) = lit {
                            return Ok(lit.value());
                        }
                    }
                }
            }
        }
    }

    Err(syn::Error::new_spanned(
        &ast,
        format!("Attribute `{}` not found", attr),
    ))
}

/// Extract a list of configuration parameter values specified in the `graphql` attribute.
pub fn extract_attr_list(ast: &syn::DeriveInput, attr: &str) -> Result<Vec<String>, syn::Error> {
    let attributes = &ast.attrs;
    let graphql_path = path_to_match();
    let attribute = attributes
        .iter()
        .find(|attr| attr.path == graphql_path)
        .ok_or_else(|| syn::Error::new_spanned(ast, "The graphql attribute is missing"))?;
    if let syn::Meta::List(items) = &attribute.parse_meta().expect("Attribute is well formatted") {
        for item in items.nested.iter() {
            if let syn::NestedMeta::Meta(syn::Meta::List(value_list)) = item {
                if let Some(ident) = value_list.path.get_ident() {
                    if ident == attr {
                        return value_list
                            .nested
                            .iter()
                            .map(|lit| {
                                if let syn::NestedMeta::Lit(syn::Lit::Str(lit)) = lit {
                                    Ok(lit.value())
                                } else {
                                    Err(syn::Error::new_spanned(
                                        lit,
                                        "Attribute inside value list must be a literal",
                                    ))
                                }
                            })
                            .collect();
                    }
                }
            }
        }
    }

    Err(syn::Error::new_spanned(ast, "Attribute not found"))
}

/// Get the deprecation from a struct attribute in the derive case.
pub fn extract_deprecation_strategy(
    ast: &syn::DeriveInput,
) -> Result<DeprecationStrategy, syn::Error> {
    extract_attr(ast, "deprecated")?
        .to_lowercase()
        .as_str()
        .parse()
        .map_err(|_| syn::Error::new_spanned(ast, DEPRECATION_ERROR.to_owned()))
}

/// Get the deprecation from a struct attribute in the derive case.
pub fn extract_normalization(ast: &syn::DeriveInput) -> Result<Normalization, syn::Error> {
    extract_attr(ast, "normalization")?
        .to_lowercase()
        .as_str()
        .parse()
        .map_err(|_| syn::Error::new_spanned(ast, NORMALIZATION_ERROR))
}

pub fn extract_fragments_other_variant(ast: &syn::DeriveInput) -> bool {
    extract_attr(ast, "fragments_other_variant")
        .ok()
        .and_then(|s| FromStr::from_str(s.as_str()).ok())
        .unwrap_or(false)
}

pub fn extract_skip_serializing_none(ast: &syn::DeriveInput) -> bool {
    ident_exists(ast, "skip_serializing_none").is_ok()
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
        assert!(!extract_skip_serializing_none(&parsed));
    }
}
