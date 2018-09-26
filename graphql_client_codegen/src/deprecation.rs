use attributes;
use failure;
use syn;

static DEPRECATION_ERROR: &'static str = "deprecated must be one of 'allow', 'deny', or 'warn'";

#[derive(Debug, PartialEq, Hash, Clone)]
pub enum DeprecationStatus {
    Current,
    Deprecated(Option<String>),
}

#[derive(Debug, PartialEq)]
pub enum DeprecationStrategy {
    Allow,
    Deny,
    Warn,
}

impl Default for DeprecationStrategy {
    fn default() -> Self { DeprecationStrategy::Warn }
}

pub fn extract_deprecation_strategy(
    ast: &syn::DeriveInput,
) -> Result<DeprecationStrategy, failure::Error> {
    match attributes::extract_attr(&ast, "deprecated")?
        .to_lowercase()
        .as_str()
    {
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
