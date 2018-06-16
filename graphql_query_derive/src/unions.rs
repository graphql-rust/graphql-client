use graphql_parser::query;
use proc_macro2::TokenStream;
use query::QueryContext;

#[derive(Debug)]
pub struct GqlUnion(pub Vec<String>);

impl GqlUnion {
    pub fn response_for_selection(
        &self,
        query_context: &QueryContext,
        _selection: &query::SelectionSet,
        _prefix: &str,
    ) -> TokenStream {
        unimplemented!("union generation")
    }
}
