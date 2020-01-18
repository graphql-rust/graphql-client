use crate::query::QueryContext;
use crate::selection::Selection;
use proc_macro2::TokenStream;
use std::cell::Cell;

/// Represents which type a fragment is defined on. This is the type mentioned in the fragment's `on` clause.
#[derive(Debug, PartialEq)]
pub(crate) enum FragmentTarget<'context> {
    Object(&'context crate::objects::GqlObject<'context>),
    Interface(&'context crate::interfaces::GqlInterface<'context>),
    Union(&'context crate::unions::GqlUnion<'context>),
}

impl<'context> FragmentTarget<'context> {
    pub(crate) fn name(&self) -> &str {
        match self {
            FragmentTarget::Object(obj) => obj.name,
            FragmentTarget::Interface(iface) => iface.name,
            FragmentTarget::Union(unn) => unn.name,
        }
    }
}

/// Represents a fragment extracted from a query document.
#[derive(Debug, PartialEq)]
pub(crate) struct GqlFragment<'query> {
    /// The name of the fragment, matching one-to-one with the name in the GraphQL query document.
    pub name: &'query str,
    /// The `on` clause of the fragment.
    pub on: FragmentTarget<'query>,
    /// The selected fields.
    pub selection: Selection<'query>,
    /// Whether the fragment is used in the current query
    pub is_required: Cell<bool>,
}

impl<'query> GqlFragment<'query> {
    /// Generate all the Rust code required by the fragment's object selection.
    pub(crate) fn to_rust(
        &self,
        context: &QueryContext<'_>,
    ) -> Result<TokenStream, failure::Error> {
        match self.on {
            FragmentTarget::Object(obj) => {
                obj.response_for_selection(context, &self.selection, &self.name)
            }
            FragmentTarget::Interface(iface) => {
                iface.response_for_selection(context, &self.selection, &self.name)
            }
            FragmentTarget::Union(_) => {
                unreachable!("Wrong code path. Fragment on unions are treated differently.")
            }
        }
    }

    pub(crate) fn is_recursive(&self) -> bool {
        self.selection.contains_fragment(&self.name)
    }

    pub(crate) fn require<'schema>(&self, context: &QueryContext<'query>) {
        self.is_required.set(true);
        self.selection.require_items(context);
    }
}
