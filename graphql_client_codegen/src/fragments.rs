use proc_macro2::TokenStream;
use query::QueryContext;
use selection::Selection;
use std::cell::Cell;

/// Represents a fragment extracted from a query document.
#[derive(Debug, PartialEq)]
pub(crate) struct GqlFragment<'query> {
    /// The name of the fragment, matching one-to-one with the name in the GraphQL query document.
    pub name: &'query str,
    /// The `on` clause of the fragment.
    pub on: &'query str,
    /// The selected fields.
    pub selection: Selection<'query>,
    /// Whether the fragment is used in the current query
    pub is_required: Cell<bool>,
}

impl<'query> GqlFragment<'query> {
    /// Generate all the Rust code required by the fragment's object selection.
    pub(crate) fn to_rust(&self, context: &QueryContext) -> Result<TokenStream, ::failure::Error> {
        if let Some(obj) = context.schema.objects.get(&self.on) {
            obj.response_for_selection(context, &self.selection, &self.name)
        } else if let Some(iface) = context.schema.interfaces.get(&self.on) {
            iface.response_for_selection(context, &self.selection, &self.name)
        } else {
            Err(format_err!(
                "Fragment {} is defined on unknown type: {}",
                self.name,
                self.on
            ))?
        }
    }
}
