use failure;
use graphql_client_codegen::deprecation::DeprecationStrategy;
use syn;

const DEPRECATION_ERROR: &str = "deprecated must be one of 'allow', 'deny', or 'warn'";

/// The `graphql` attribute as a `syn::Path`.
fn path_to_match() -> syn::Path {
    syn::parse_str("graphql").expect("`graphql` is a valid path")
}

/// Extract an configuration parameter specified in the `graphql` attribute.
pub fn extract_attr(ast: &syn::DeriveInput, attr: &str) -> Result<String, failure::Error> {
    let attributes = &ast.attrs;
    let graphql_path = path_to_match();
    let attribute = attributes
        .iter()
        .find(|attr| attr.path == graphql_path)
        .ok_or_else(|| format_err!("The graphql attribute is missing"))?;
    if let syn::Meta::List(items) = &attribute
        .interpret_meta()
        .expect("Attribute is well formatted")
    {
        for item in items.nested.iter() {
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = item {
                let syn::MetaNameValue { ident, lit, .. } = name_value;
                if ident == attr {
                    if let syn::Lit::Str(lit) = lit {
                        return Ok(lit.value());
                    }
                }
            }
        }
    }

    Err(format_err!("attribute not found"))?
}

/// Get the deprecation from a struct attribute in the derive case.
pub fn extract_deprecation_strategy(
    ast: &syn::DeriveInput,
) -> Result<DeprecationStrategy, failure::Error> {
    match extract_attr(&ast, "deprecated")?.to_lowercase().as_str() {
        "allow" => Ok(DeprecationStrategy::Allow),
        "deny" => Ok(DeprecationStrategy::Deny),
        "warn" => Ok(DeprecationStrategy::Warn),
        _ => Err(format_err!("{}", DEPRECATION_ERROR))?,
    }
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
