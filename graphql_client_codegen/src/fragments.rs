use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use selection::Selection;
use std::cell::Cell;

#[derive(Debug, PartialEq)]
pub struct GqlFragment {
    pub name: String,
    pub on: String,
    pub selection: Selection,
    pub is_required: Cell<bool>,
}

impl GqlFragment {
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
