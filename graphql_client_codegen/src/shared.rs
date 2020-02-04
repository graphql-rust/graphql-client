// use crate::deprecation::{DeprecationStatus, DeprecationStrategy};
// use crate::objects::GqlObjectField;
// use crate::query::QueryContext;
// use crate::selection::*;
// use failure::*;
// use heck::{CamelCase, SnakeCase};
use proc_macro2::TokenStream;
use quote::quote;

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

pub(crate) fn keyword_replace(needle: &str) -> String {
    match RUST_KEYWORDS.binary_search(&needle) {
        Ok(index) => [RUST_KEYWORDS[index], "_"].concat(),
        Err(_) => needle.to_owned(),
    }
}

// pub(crate) fn render_object_field(
//     field_name: &str,
//     field_type: &TokenStream,
//     description: Option<&str>,
//     status: &DeprecationStatus,
//     strategy: &DeprecationStrategy,
// ) -> Option<TokenStream> {
//     #[allow(unused_assignments)]
//     let mut deprecation = quote!();
//     match (status, strategy) {
//         // If the field is deprecated and we are denying usage, don't generate the
//         // field in rust at all and short-circuit.
//         (DeprecationStatus::Deprecated(_), DeprecationStrategy::Deny) => return None,
//         // Everything is allowed so there is nothing to do.
//         (_, DeprecationStrategy::Allow) => deprecation = quote!(),
//         // Current so there is nothing to do.
//         (DeprecationStatus::Current, _) => deprecation = quote!(),
//         // A reason was provided, translate it to a note.
//         (DeprecationStatus::Deprecated(Some(reason)), DeprecationStrategy::Warn) => {
//             deprecation = quote!(#[deprecated(note = #reason)])
//         }
//         // No reason provided, just mark as deprecated.
//         (DeprecationStatus::Deprecated(None), DeprecationStrategy::Warn) => {
//             deprecation = quote!(#[deprecated])
//         }
//     };

//     let description = description.map(|s| quote!(#[doc = #s]));
//     let rust_safe_field_name = keyword_replace(&field_name.to_snake_case());
//     let name_ident = Ident::new(&rust_safe_field_name, Span::call_site());
//     let rename = crate::shared::field_rename_annotation(&field_name, &rust_safe_field_name);

//     Some(quote!(#description #deprecation #rename pub #name_ident: #field_type))
// }

// pub(crate) fn field_impls_for_selection(
//     fields: &[GqlObjectField<'_>],
//     context: &QueryContext<'_>,
//     selection: &Selection<'_>,
//     prefix: &str,
// ) -> anyhow::Result<Vec<TokenStream>> {
//     todo!("field_impls_for_selection")
//     // (&selection)
//     //     .into_iter()
//     //     .map(|selected| {
//     //         if let SelectionItem::Field(selected) = selected {
//     //             let name = &selected.name;
//     //             let alias = selected.alias.as_ref().unwrap_or(name);

//     //             let ty = fields
//     //                 .iter()
//     //                 .find(|f| &f.name == name)
//     //                 .ok_or_else(|| format_err!("could not find field `{}`", name))?
//     //                 .type_
//     //                 .inner_name_str();
//     //             let prefix = format!("{}{}", prefix.to_camel_case(), alias.to_camel_case());
//     //             context.maybe_expand_field(&ty, &selected.fields, &prefix)
//     //         } else {
//     //             Ok(None)
//     //         }
//     //     })
//     //     .filter_map(|i| i.transpose())
//     //     .collect()
// }

// pub(crate) fn response_fields_for_selection(
//     type_name: &str,
//     schema_fields: &[GqlObjectField<'_>],
//     context: &QueryContext<'_>,
//     selection: &Selection<'_>,
//     prefix: &str,
// ) -> anyhow::Result<Vec<TokenStream>> {
//     todo!("response fields for selection")
//     // (&selection)
//     //     .into_iter()
//     //     .map(|item| match item {
//     //         SelectionItem::Field(f) => {
//     //             let name = &f.name;
//     //             let alias = f.alias.as_ref().unwrap_or(name);

//     //             let schema_field = &schema_fields
//     //                 .iter()
//     //                 .find(|field| &field.name == name)
//     //                 .ok_or_else(|| {
//     //                     format_err!(
//     //                         "Could not find field `{}` on `{}`. Available fields: `{}`.",
//     //                         *name,
//     //                         type_name,
//     //                         schema_fields
//     //                             .iter()
//     //                             .map(|ref field| &field.name)
//     //                             .fold(String::new(), |mut acc, item| {
//     //                                 acc.push_str(item);
//     //                                 acc.push_str(", ");
//     //                                 acc
//     //                             })
//     //                             .trim_end_matches(", ")
//     //                     )
//     //                 })?;
//     //             let ty = schema_field.type_.to_rust(
//     //                 context,
//     //                 &format!("{}{}", prefix.to_camel_case(), alias.to_camel_case()),
//     //             );

//     //             Ok(render_object_field(
//     //                 alias,
//     //                 &ty,
//     //                 schema_field.description.as_ref().cloned(),
//     //                 &schema_field.deprecation,
//     //                 &context.deprecation_strategy,
//     //             ))
//     //         }
//     //         SelectionItem::FragmentSpread(fragment) => {
//     //             let field_name =
//     //                 Ident::new(&fragment.fragment_name.to_snake_case(), Span::call_site());
//     //             context.require_fragment(&fragment.fragment_name);
//     //             let fragment_from_context = context
//     //                 .fragments
//     //                 .get(&fragment.fragment_name)
//     //                 .ok_or_else(|| format_err!("Unknown fragment: {}", &fragment.fragment_name))?;
//     //             let type_name = Ident::new(&fragment.fragment_name, Span::call_site());
//     //             let type_name = if fragment_from_context.is_recursive() {
//     //                 quote!(Box<#type_name>)
//     //             } else {
//     //                 quote!(#type_name)
//     //             };
//     //             Ok(Some(quote! {
//     //                 #[serde(flatten)]
//     //                 pub #field_name: #type_name
//     //             }))
//     //         }
//     //         SelectionItem::InlineFragment(_) => Err(format_err!(
//     //             "unimplemented: inline fragment on object field"
//     //         )),
//     //     })
//     //     .filter_map(|x| match x {
//     //         // Remove empty fields so callers always know a field has some
//     //         // tokens.
//     //         Ok(f) => f.map(Ok),
//     //         Err(err) => Some(Err(err)),
//     //     })
//     //     .collect()
// }

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
