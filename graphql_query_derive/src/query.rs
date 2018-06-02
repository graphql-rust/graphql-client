use field_type::FieldType;
use std::collections::BTreeMap;

pub struct QueryContext {
    pub fragments: BTreeMap<String, BTreeMap<String, FieldType>>,
}

impl QueryContext {
    pub fn new() -> QueryContext {
        QueryContext {
            fragments: BTreeMap::new(),
        }
    }
}
