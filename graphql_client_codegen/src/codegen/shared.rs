use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;

// List of keywords based on https://doc.rust-lang.org/reference/keywords.html
// code snippet: `[...new Set($$("code.hljs").map(x => x.textContent).filter(x => x.match(/^[_a-z0-9]+$/i)))].sort()`
const RUST_KEYWORDS: &[&str] = &[
    "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "union", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
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

#[cfg(test)]
mod tests {
    #[test]
    fn keyword_replace_works() {
        use super::keyword_replace;
        assert_eq!("fora", keyword_replace("fora"));
        assert_eq!("in_", keyword_replace("in"));
        assert_eq!("fn_", keyword_replace("fn"));
        assert_eq!("struct_", keyword_replace("struct"));
    }
}
