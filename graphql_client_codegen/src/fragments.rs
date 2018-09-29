use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use selection::Selection;
use std::cell::Cell;

/// Represents a fragment extracted from a query document.
#[derive(Debug, PartialEq)]
pub struct GqlFragment {
    /// The name of the fragment, matching one-to-one with the name in the GraphQL query document.
    pub name: String,
    /// The `on` clause of the fragment.
    pub on: String,
    /// The selected fields.
    pub selection: Selection,
    /// Whether the fragment
    pub is_required: Cell<bool>,
}

impl GqlFragment {
    /// Generate all the Rust code required by the fragment's selection.
    pub(crate) fn to_rust(&self, context: &QueryContext) -> Result<TokenStream, ::failure::Error> {
        let derives = context.response_derives();
        let name_ident = Ident::new(&self.name, Span::call_site());
        let opt_object = context.schema.objects.get(&self.on);
        let (field_impls, fields) = if let Some(object) = opt_object {
            let field_impls =
                object.field_impls_for_selection(context, &self.selection, &self.name)?;
            let fields =
                object.response_fields_for_selection(context, &self.selection, &self.name)?;
            (field_impls, fields)
        } else if let Some(iface) = context.schema.interfaces.get(&self.on) {
            let field_impls =
                iface.field_impls_for_selection(context, &self.selection, &self.name)?;
            let fields =
                iface.response_fields_for_selection(context, &self.selection, &self.name)?;
            (field_impls, fields)
        } else {
            panic!(
                "fragment '{}' cannot operate on unknown type '{}'",
                self.name, self.on
            );
        };

        Ok(quote!{
            #derives
            pub struct #name_ident {
                #(#fields,)*
            }

            #(#field_impls)*
        })
    }
}
