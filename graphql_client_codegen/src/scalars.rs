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
        let ident = Ident::new(&self.name, Span::call_site());
        let description = self.description.map(|d| quote!(#[doc = #d]));
        quote!(#description type #ident = super::#ident;)
    }
}
