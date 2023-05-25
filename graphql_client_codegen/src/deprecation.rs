/// Whether an item is deprecated, with context.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DeprecationStatus {
    /// Not deprecated
    Current,
    /// Deprecated
    Deprecated(Option<String>),
}

/// The available deprecation strategies.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum DeprecationStrategy {
    /// Allow use of deprecated items in queries, and say nothing.
    Allow,
    /// Fail compilation if a deprecated item is used.
    Deny,
    /// Allow use of deprecated items in queries, but warn about them (default).
    #[default]
    Warn,
}

impl std::str::FromStr for DeprecationStrategy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.trim() {
            "allow" => Ok(DeprecationStrategy::Allow),
            "deny" => Ok(DeprecationStrategy::Deny),
            "warn" => Ok(DeprecationStrategy::Warn),
            _ => Err(()),
        }
    }
}
