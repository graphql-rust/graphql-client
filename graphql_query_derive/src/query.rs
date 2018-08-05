use failure;
use fragments::GqlFragment;
use operations::Operation;
use proc_macro2::TokenStream;
use schema::Schema;
use selection::Selection;
use std::collections::BTreeMap;

/// This holds all the information we need during the code generation phase.
pub(crate) struct QueryContext {
    pub fragments: BTreeMap<String, GqlFragment>,
    pub schema: Schema,
    pub selected_operation: Option<Operation>,
}

impl QueryContext {
    /// Create a QueryContext with the given Schema.
    pub(crate) fn new(schema: Schema) -> QueryContext {
        QueryContext {
            fragments: BTreeMap::new(),
            schema,
            selected_operation: None,
        }
    }

    /// For testing only. creates an empty QueryContext with an empty Schema.
    #[cfg(test)]
    pub(crate) fn new_empty() -> QueryContext {
        QueryContext {
            fragments: BTreeMap::new(),
            schema: Schema::new(),
            selected_operation: None,
        }
    }

    pub(crate) fn maybe_expand_field(
        &self,
        ty: &str,
        selection: &Selection,
        prefix: &str,
    ) -> Result<TokenStream, failure::Error> {
        if let Some(_enm) = self.schema.enums.get(ty) {
            Ok(quote!()) // we already expand enums separately
        } else if let Some(obj) = self.schema.objects.get(ty) {
            obj.response_for_selection(self, &selection, prefix)
        } else if let Some(iface) = self.schema.interfaces.get(ty) {
            iface.response_for_selection(self, &selection, prefix)
        } else if let Some(unn) = self.schema.unions.get(ty) {
            unn.response_for_selection(self, &selection, prefix)
        } else {
            Ok(quote!())
        }
    }
}
