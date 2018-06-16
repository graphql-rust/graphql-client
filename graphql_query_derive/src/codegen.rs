use field_type::FieldType;
use quote::ToTokens;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct StructFieldDescriptor {
    pub attributes: TokenStream,
    pub name: String,
    pub ty: FieldType,
}

#[derive(Debug)]
pub struct StructDescriptor {
    pub attributes: TokenStream,
    pub fields: Vec<StructFieldDescriptor>,
    pub name: String,
}

impl StructDescriptor {
    pub fn field_by_name(&self, name: &str) -> Option<StructFieldDescriptor> {
        unimplemented!()
    }
}

pub struct EnumVariantDescriptor {
    pub attributes: TokenStream,
    pub name: String,
}

#[derive(Debug)]
pub struct EnumDescriptor {
    pub name: String,
    pub variants: Vec<String>,
}
