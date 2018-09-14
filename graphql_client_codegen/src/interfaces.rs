use failure;
use objects::GqlObjectField;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use selection::{Selection, SelectionItem};
use shared::*;
use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashSet;
use unions::union_variants;

#[derive(Debug, Clone, PartialEq)]
pub struct GqlInterface {
    pub description: Option<String>,
    pub implemented_by: HashSet<String>,
    pub name: String,
    pub fields: Vec<GqlObjectField>,
    pub is_required: Cell<bool>,
}

impl GqlInterface {
    pub fn new(name: Cow<str>, description: Option<&str>) -> GqlInterface {
        GqlInterface {
            description: description.map(|d| d.to_owned()),
            name: name.into_owned(),
            implemented_by: HashSet::new(),
            fields: vec![],
            is_required: false.into(),
        }
    }

    pub(crate) fn response_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<TokenStream, failure::Error> {
        let name = Ident::new(&prefix, Span::call_site());
        let derives = query_context.response_derives();

        selection
            .extract_typename()
            .ok_or_else(|| format_err!("Missing __typename in selection for {}", prefix))?;

        let object_selection = Selection(
            selection.0.iter()
            // Only keep what we can handle
            .filter(|f| match f {
                SelectionItem::Field(f) => f.name != "__typename",
                SelectionItem::FragmentSpread(_) => true,
                SelectionItem::InlineFragment(_) => false,
            }).map(|a| (*a).clone()).collect(),
        );

        let union_selection = Selection(
            selection.0.iter()
            // Only keep what we can handle
            .filter(|f| match f {
                SelectionItem::InlineFragment(_) => true,
                SelectionItem::Field(_) | SelectionItem::FragmentSpread(_) => false,
            }).map(|a| (*a).clone()).collect(),
        );

        let object_fields =
            response_fields_for_selection(&self.fields, query_context, &object_selection, prefix)?;

        let object_children =
            field_impls_for_selection(&self.fields, query_context, &object_selection, prefix)?;
        let (mut union_variants, union_children, used_variants) =
            union_variants(&union_selection, query_context, prefix)?;

        union_variants.extend(
            self.implemented_by
                .iter()
                .filter(|obj| used_variants.iter().find(|v| v == obj).is_none())
                .map(|v| {
                    let v = Ident::new(v, Span::call_site());
                    quote!(#v)
                }),
        );

        let attached_enum_name = Ident::new(&format!("{}On", name), Span::call_site());
        let (attached_enum, last_object_field) = if !union_variants.is_empty() {
            let attached_enum = quote! {
                #derives
                #[serde(tag = "__typename")]
                pub enum #attached_enum_name {
                    #(#union_variants,)*
                }
            };
            let last_object_field = quote!(#[serde(flatten)] pub on: #attached_enum_name,);
            (attached_enum, last_object_field)
        } else {
            (quote!(), quote!())
        };

        Ok(quote! {

            #(#object_children)*

            #(#union_children)*

            #attached_enum

            #derives
            pub struct #name {
                #(#object_fields,)*
                #last_object_field
            }
        })
    }
}
