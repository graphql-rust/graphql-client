use graphql_parser::query;
use proc_macro2::TokenStream;
use query::QueryContext;

#[derive(Debug, PartialEq)]
pub struct GqlUnion(pub Vec<String>);

impl GqlUnion {
    pub fn response_for_selection(
        &self,
        _query_context: &QueryContext,
        _selection: &query::SelectionSet,
        _prefix: &str,
    ) -> TokenStream {
        unimplemented!("union generation")
    }
}
