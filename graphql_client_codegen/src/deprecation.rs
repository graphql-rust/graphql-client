use attributes;
use failure;
use syn;

static DEPRECATION_ERROR: &'static str = "deprecated must be one of 'allow', 'deny', or 'warn'";

/// Whether an item is deprecated, with context.
#[derive(Debug, PartialEq, Hash, Clone)]
pub enum DeprecationStatus {
    /// Not deprecated
    Current,
    /// Deprecated
    Deprecated(Option<String>),
}

/// The available deprecation startegies.
#[derive(Debug, PartialEq)]
pub enum DeprecationStrategy {
    /// Allow use of deprecated items in queries, and say nothing.
    Allow,
    /// Fail compilation if a deprecated item is used.
    Deny,
    /// Allow use of deprecated items in queries, but warn about them (default).
    Warn,
}

impl Default for DeprecationStrategy {
    fn default() -> Self {
        DeprecationStrategy::Warn
    }
}

/// Get the deprecation from a struct attribute in the derive case.
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
