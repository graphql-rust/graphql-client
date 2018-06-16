use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use graphql_parser::query::SelectionSet;

#[derive(Debug, PartialEq)]
pub struct GqlFragment {
    pub name: String,
    pub on: String,
    pub selection: SelectionSet,
}

impl GqlFragment {
    pub fn to_rust(&self, context: &QueryContext) -> TokenStream {
        let name_ident = Ident::new(&self.name, Span::call_site());

        quote!{
            #[derive(Debug, Deserialize)]
            pub struct #name_ident;
        }
    }
}
