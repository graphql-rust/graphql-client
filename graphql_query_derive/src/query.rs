use field_type::FieldType;
use proc_macro2::TokenStream;
use std::collections::BTreeMap;

pub struct QueryContext {
    pub _subscription_root: Option<Vec<TokenStream>>,
    pub fragments: BTreeMap<String, BTreeMap<String, FieldType>>,
    pub mutation_root: Option<Vec<TokenStream>>,
    pub query_root: Option<Vec<TokenStream>>,
    pub variables: BTreeMap<String, FieldType>,
}

impl QueryContext {
    pub fn new() -> QueryContext {
        QueryContext {
            _subscription_root: None,
            fragments: BTreeMap::new(),
            mutation_root: None,
            query_root: None,
            variables: BTreeMap::new(),
        }
    }
}
