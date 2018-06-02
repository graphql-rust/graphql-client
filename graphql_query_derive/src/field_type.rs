use graphql_parser::schema;
use proc_macro2::{Ident, Span, TokenStream};

#[derive(Debug, PartialEq)]
pub enum FieldType {
    Named(Ident),
    Optional(Box<FieldType>),
    Vector(Box<FieldType>),
}

impl FieldType {
    pub fn to_rust(&self) -> TokenStream {
        match &self {
            FieldType::Named(name) => quote!(#name),
            FieldType::Optional(inner) => {
                let inner = inner.to_rust();
                quote!( Option<#inner>)
            }
            FieldType::Vector(inner) => {
                let inner = inner.to_rust();
                quote!( Vec<#inner>)
            }
        }
    }

    pub fn inner_name_string(&self) -> String {
        match &self {
            FieldType::Named(name) => name.to_string(),
            FieldType::Optional(inner) => (*inner).inner_name_string(),
            FieldType::Vector(inner) => (*inner).inner_name_string(),
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            FieldType::Named(name) => name.to_string(),
            FieldType::Optional(inner) => format!("Option<{}>", inner.to_string()),
            FieldType::Vector(inner) => format!("Vec<{}>", inner.to_string()),
        }
    }
}

impl ::std::convert::From<schema::Type> for FieldType {
    fn from(schema_type: schema::Type) -> FieldType {
        from_schema_type_inner(schema_type, false)
    }
}

fn from_schema_type_inner(inner: schema::Type, non_null: bool) -> FieldType {
    let inner_field_type = match inner {
        schema::Type::ListType(inner) => {
            let inner = from_schema_type_inner(*inner, false);
            FieldType::Vector(Box::new(inner))
        }
        schema::Type::NamedType(name) => FieldType::Named(Ident::new(&name, Span::call_site())),
        schema::Type::NonNullType(inner) => from_schema_type_inner(*inner, true),
    };

    if non_null {
        inner_field_type
    } else {
        FieldType::Optional(Box::new(inner_field_type))
    }
}
