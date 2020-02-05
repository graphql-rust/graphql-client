use anyhow::*;
use graphql_client_codegen::deprecation::DeprecationStrategy;
use graphql_client_codegen::normalization::Normalization;

const DEPRECATION_ERROR: &str = "deprecated must be one of 'allow', 'deny', or 'warn'";
const NORMALIZATION_ERROR: &str = "normalization must be one of 'none' or 'rust'";

/// The `graphql` attribute as a `syn::Path`.
fn path_to_match() -> syn::Path {
    syn::parse_str("graphql").expect("`graphql` is a valid path")
}

/// Extract an configuration parameter specified in the `graphql` attribute.
pub fn extract_attr(ast: &syn::DeriveInput, attr: &str) -> Result<String, anyhow::Error> {
    let attributes = &ast.attrs;
    let graphql_path = path_to_match();
    let attribute = attributes
        .iter()
        .find(|attr| attr.path == graphql_path)
        .ok_or_else(|| format_err!("The graphql attribute is missing"))?;
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

    Err(format_err!("attribute not found"))
}

/// Get the deprecation from a struct attribute in the derive case.
pub fn extract_deprecation_strategy(
    ast: &syn::DeriveInput,
) -> Result<DeprecationStrategy, anyhow::Error> {
    extract_attr(&ast, "deprecated")?
        .to_lowercase()
        .as_str()
        .parse()
        .map_err(|_| format_err!("{}", DEPRECATION_ERROR))
}

/// Get the deprecation from a struct attribute in the derive case.
pub fn extract_normalization(ast: &syn::DeriveInput) -> Result<Normalization, anyhow::Error> {
    extract_attr(&ast, "normalization")?
        .to_lowercase()
        .as_str()
        .parse()
        .map_err(|_| format_err!("{}", NORMALIZATION_ERROR))
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
}
