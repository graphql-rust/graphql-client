use proc_macro2;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Scalar {
    pub name: String,
    pub description: Option<String>,
}

impl Scalar {
    // TODO: do something smarter here
    pub fn to_rust(&self) -> proc_macro2::TokenStream {
        use proc_macro2::{Ident, Span};
        let ident = Ident::new(&self.name, Span::call_site());
        let description = match &self.description {
            Some(d) => quote!(#[doc = #d]),
            None => quote!(),
        };
        quote!(#description type #ident = super::#ident;)
    }
}
