use crate::enums::ENUMS_PREFIX;
use crate::introspection_response;
use crate::query::QueryContext;
use crate::schema::DEFAULT_SCALARS;
use graphql_parser;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum FieldType<'a> {
    Named(&'a str),
    Optional(Box<FieldType<'a>>),
    Vector(Box<FieldType<'a>>),
}

impl<'a> FieldType<'a> {
    /// Takes a field type with its name
    pub(crate) fn to_rust(&self, context: &QueryContext<'_, '_>, prefix: &str) -> TokenStream {
        let prefix: &str = if prefix.is_empty() {
            self.inner_name_str()
        } else {
            prefix
        };
        match &self {
            FieldType::Named(ref name) => {
                let full_name = if context
                    .schema
                    .scalars
                    .get(name)
                    .map(|s| s.is_required.set(true))
                    .is_some()
                    || DEFAULT_SCALARS.iter().any(|elem| elem == name)
                {
                    name.to_string()
                } else if context
                    .schema
                    .enums
                    .get(name)
                    .map(|enm| enm.is_required.set(true))
                    .is_some()
                {
                    format!("{}{}", ENUMS_PREFIX, name)
                } else {
                    if prefix.is_empty() {
                        panic!("Empty prefix for {:?}", self);
                    }
                    prefix.to_string()
                };

                #[cfg(feature = "normalize_query_types")]
                let full_name = if full_name == "ID" || full_name.starts_with("__") {
                    full_name
                } else {
                    use heck::CamelCase;

                    full_name.to_camel_case()
                };

                let full_name = Ident::new(&full_name, Span::call_site());

                quote!(#full_name)
            }
            FieldType::Optional(inner) => {
                let inner = inner.to_rust(context, &prefix);
                quote!( Option<#inner>)
            }
            FieldType::Vector(inner) => {
                let inner = inner.to_rust(context, &prefix);
                quote!( Vec<#inner>)
            }
        }
    }

    /// Return the innermost name - we mostly use this for looking types up in our Schema struct.
    pub fn inner_name_str(&self) -> &str {
        match &self {
            FieldType::Named(name) => name,
            FieldType::Optional(inner) => inner.inner_name_str(),
            FieldType::Vector(inner) => inner.inner_name_str(),
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            FieldType::Optional(_) => true,
            _ => false,
        }
    }

    /// A type is indirected if it is a (flat or nested) list type, optional or not.
    ///
    /// We use this to determine whether a type needs to be boxed for recursion.
    pub fn is_indirected(&self) -> bool {
        match self {
            FieldType::Vector(_) => true,
            FieldType::Named(_) => false,
            FieldType::Optional(inner) => inner.is_indirected(),
        }
    }
}

impl<'schema> ::std::convert::From<&'schema graphql_parser::schema::Type> for FieldType<'schema> {
    fn from(schema_type: &'schema graphql_parser::schema::Type) -> FieldType<'schema> {
        from_schema_type_inner(schema_type, false)
    }
}

fn from_schema_type_inner(inner: &graphql_parser::schema::Type, non_null: bool) -> FieldType<'_> {
    match inner {
        graphql_parser::schema::Type::ListType(inner) => {
            let inner = from_schema_type_inner(&*inner, false);
            let f = FieldType::Vector(Box::new(inner));
            if non_null {
                f
            } else {
                FieldType::Optional(Box::new(f))
            }
        }
        graphql_parser::schema::Type::NamedType(name) => {
            let f = FieldType::Named(name);
            if non_null {
                f
            } else {
                FieldType::Optional(Box::new(f))
            }
        }
        graphql_parser::schema::Type::NonNullType(inner) => from_schema_type_inner(&*inner, true),
    }
}

fn from_json_type_inner(inner: &introspection_response::TypeRef, non_null: bool) -> FieldType<'_> {
    use crate::introspection_response::*;

    match inner.kind {
        Some(__TypeKind::NON_NULL) => from_json_type_inner(
            &inner.of_type.as_ref().expect("inner type is missing"),
            true,
        ),
        Some(__TypeKind::LIST) => {
            let f = FieldType::Vector(Box::new(from_json_type_inner(
                &inner.of_type.as_ref().expect("inner type is missing"),
                false,
            )));
            if non_null {
                f
            } else {
                FieldType::Optional(Box::new(f))
            }
        }
        Some(_) => {
            let f = FieldType::Named(&inner.name.as_ref().expect("type name"));
            if non_null {
                f
            } else {
                FieldType::Optional(Box::new(f))
            }
        }
        None => unreachable!("non-convertible type"),
    }
}

impl<'schema> ::std::convert::From<&'schema introspection_response::FullTypeFieldsType>
    for FieldType<'schema>
{
    fn from(
        schema_type: &'schema introspection_response::FullTypeFieldsType,
    ) -> FieldType<'schema> {
        from_json_type_inner(&schema_type.type_ref, false)
    }
}

impl<'a> ::std::convert::From<&'a introspection_response::InputValueType> for FieldType<'a> {
    fn from(schema_type: &'a introspection_response::InputValueType) -> FieldType<'a> {
        from_json_type_inner(&schema_type.type_ref, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::introspection_response::{FullTypeFieldsType, TypeRef, __TypeKind};
    use graphql_parser::schema::Type as GqlParserType;

    #[test]
    fn field_type_from_graphql_parser_schema_type_works() {
        let ty = GqlParserType::NamedType("Cat".to_owned());
        assert_eq!(
            FieldType::from(&ty),
            FieldType::Optional(Box::new(FieldType::Named("Cat")))
        );

        let ty = GqlParserType::NonNullType(Box::new(GqlParserType::NamedType("Cat".to_owned())));

        assert_eq!(FieldType::from(&ty), FieldType::Named("Cat"));
    }

    #[test]
    fn field_type_from_introspection_response_works() {
        let ty = FullTypeFieldsType {
            type_ref: TypeRef {
                kind: Some(__TypeKind::OBJECT),
                name: Some("Cat".into()),
                of_type: None,
            },
        };
        assert_eq!(
            FieldType::from(&ty),
            FieldType::Optional(Box::new(FieldType::Named("Cat")))
        );

        let ty = FullTypeFieldsType {
            type_ref: TypeRef {
                kind: Some(__TypeKind::NON_NULL),
                name: None,
                of_type: Some(Box::new(TypeRef {
                    kind: Some(__TypeKind::OBJECT),
                    name: Some("Cat".into()),
                    of_type: None,
                })),
            },
        };
        assert_eq!(FieldType::from(&ty), FieldType::Named("Cat"));
    }
}
