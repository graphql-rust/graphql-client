use graphql_parser::query;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;

#[derive(Debug)]
pub struct GqlUnion(pub Vec<String>);

impl GqlUnion {
    pub fn response_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &query::SelectionSet,
        prefix: &str,
    ) -> TokenStream {
        unimplemented!("union generation")
    }
}
