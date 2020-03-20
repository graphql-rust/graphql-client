use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;

// List of keywords based on https://doc.rust-lang.org/grammar.html#keywords
const RUST_KEYWORDS: &[&str] = &[
    "abstract",
    "alignof",
    "as",
    "async",
    "await",
    "become",
    "box",
    "break",
    "const",
    "continue",
    "crate",
    "do",
    "else",
    "enum",
    "extern crate",
    "extern",
    "false",
    "final",
    "fn",
    "for",
    "for",
    "if let",
    "if",
    "if",
    "impl",
    "impl",
    "in",
    "let",
    "loop",
    "macro",
    "match",
    "mod",
    "move",
    "mut",
    "offsetof",
    "override",
    "priv",
    "proc",
    "pub",
    "pure",
    "ref",
    "return",
    "self",
    "sizeof",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "typeof",
    "unsafe",
    "unsized",
    "use",
    "use",
    "virtual",
    "where",
    "while",
    "yield",
];

pub(crate) fn keyword_replace<'a>(needle: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
    let needle = needle.into();
    match RUST_KEYWORDS.binary_search(&needle.as_ref()) {
        Ok(index) => [RUST_KEYWORDS[index], "_"].concat().into(),
        Err(_) => needle,
    }
}

/// Given the GraphQL schema name for an object/interface/input object field and
/// the equivalent rust name, produces a serde annotation to map them during
/// (de)serialization if it is necessary, otherwise an empty TokenStream.
pub(crate) fn field_rename_annotation(graphql_name: &str, rust_name: &str) -> Option<TokenStream> {
    if graphql_name != rust_name {
        Some(quote!(#[serde(rename = #graphql_name)]))
    } else {
        None
    }
}

mod tests {
    #[test]
    fn keyword_replace() {
        use super::keyword_replace;
        assert_eq!("fora", keyword_replace("fora"));
        assert_eq!("in_", keyword_replace("in"));
        assert_eq!("fn_", keyword_replace("fn"));
        assert_eq!("struct_", keyword_replace("struct"));
    }
}
