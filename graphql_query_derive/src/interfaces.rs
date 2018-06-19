use selection::Selection;
use objects::GqlObjectField;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;

#[derive(Debug, PartialEq)]
pub struct GqlInterface {
    pub implemented_by: Vec<String>,
    pub name: String,
    pub fields: Vec<GqlObjectField>,
}

impl GqlInterface {
    pub fn response_for_selection(
        &self,
        _query_context: &QueryContext,
        _selection: &Selection,
        prefix: &str,
    ) -> TokenStream {
        let name = Ident::new(&prefix, Span::call_site());
        quote! {
            #[derive(Debug, Deserialize)]
            pub struct #name;
        }
    }
}
