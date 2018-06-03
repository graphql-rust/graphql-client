use field_type::FieldType;
use graphql_parser::query;
use proc_macro2::TokenStream;
use schema::Schema;
use std::collections::BTreeMap;

pub struct QueryContext {
    pub _subscription_root: Option<Vec<TokenStream>>,
    pub fragments: BTreeMap<String, BTreeMap<String, FieldType>>,
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

    pub fn maybe_expand_field(&self, field: &query::Field, ty: &str, prefix: &str) -> TokenStream {
        if let Some(enm) = self.schema.enums.get(ty) {
            enm.to_rust()
        } else if let Some(obj) = self.schema.objects.get(ty) {
            obj.response_for_selection(self, &field.selection_set, prefix)
        } else if let Some(iface) = self.schema.interfaces.get(ty) {
            iface.response_for_selection(self, &field.selection_set, prefix)
        } else if let Some(unn) = self.schema.unions.get(ty) {
            unn.response_for_selection(self, &field.selection_set, prefix)
        } else {
            quote!()
        }
    }
}
