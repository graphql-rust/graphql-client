use quote::quote;
use std::cell::Cell;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct Scalar<'schema> {
    pub name: &'schema str,
    pub description: Option<&'schema str>,
    pub is_required: Cell<bool>,
}

impl<'schema> Scalar<'schema> {
    // TODO: do something smarter here
    pub fn to_rust(&self) -> proc_macro2::TokenStream {
        use proc_macro2::{Ident, Span};
        let name = self.name;
        #[cfg(feature = "normalize_query_types")]
        let name = {
            use heck::CamelCase;

            name.to_camel_case()
        };
        let ident = Ident::new(&name, Span::call_site());
        let description = match &self.description {
            Some(d) => quote!(#[doc = #d]),
            None => quote!(),
        };
        quote!(#description type #ident = super::#ident;)
    }
}
