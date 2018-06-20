use failure;
use field_type::FieldType;
use fragments::GqlFragment;
use proc_macro2::TokenStream;
use schema::Schema;
use std::collections::BTreeMap;
use selection::Selection;

pub struct QueryContext {
    pub _subscription_root: Option<Vec<TokenStream>>,
    pub fragments: BTreeMap<String, GqlFragment>,
    pub mutation_root: Option<Vec<TokenStream>>,
    pub query_root: Option<Vec<TokenStream>>,
    pub schema: Schema,
    pub variables: BTreeMap<String, FieldType>,
}

impl QueryContext {
    pub fn new(schema: Schema) -> QueryContext {
        QueryContext {
            _subscription_root: None,
            fragments: BTreeMap::new(),
            mutation_root: None,
            query_root: None,
            schema,
            variables: BTreeMap::new(),
        }
    }

    /// For testing only. creates an empty QueryContext with an empty Schema.
    #[cfg(test)]
    pub fn new_empty() -> QueryContext {
        QueryContext {
            _subscription_root: None,
            fragments: BTreeMap::new(),
            mutation_root: None,
            query_root: None,
            schema: Schema::new(),
            variables: BTreeMap::new(),
        }
    }

    pub fn maybe_expand_field(
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
            Ok(iface.response_for_selection(self, &selection, prefix))
        } else if let Some(unn) = self.schema.unions.get(ty) {
            unn.response_for_selection(self, &selection, prefix)
        } else {
            Ok(quote!())
        }
    }
}
