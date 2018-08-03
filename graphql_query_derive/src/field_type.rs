use enums::ENUMS_PREFIX;
use graphql_parser;
use introspection_response;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use schema::DEFAULT_SCALARS;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum FieldType {
    Named(Ident),
    Optional(Box<FieldType>),
    Vector(Box<FieldType>),
}

impl FieldType {
    /// Takes a field type with its name
    pub(crate) fn to_rust(&self, context: &QueryContext, prefix: &str) -> TokenStream {
        let prefix: String = if prefix.is_empty() {
            self.inner_name_string()
        } else {
            prefix.to_string()
        };
        match &self {
            FieldType::Named(name) => {
                let name_string = name.to_string();

                let name = if context.schema.scalars.contains_key(&name_string) || DEFAULT_SCALARS
                    .iter()
                    .any(|elem| elem == &name_string.as_str())
                {
                    name.clone()
                } else if context.schema.enums.contains_key(&name_string) {
                    Ident::new(
                        &format!("{}{}", ENUMS_PREFIX, &name_string),
                        Span::call_site(),
                    )
                } else {
                    if prefix.is_empty() {
                        panic!("Empty prefix for {:?}", self);
                    }
                    Ident::new(&prefix, Span::call_site())
                };

                quote!(#name)
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
    pub fn inner_name_string(&self) -> String {
        match &self {
            FieldType::Named(name) => name.to_string(),
            FieldType::Optional(inner) => inner.inner_name_string(),
            FieldType::Vector(inner) => inner.inner_name_string(),
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            FieldType::Optional(_) => true,
            _ => false,
        }
    }
}

impl ::std::convert::From<graphql_parser::schema::Type> for FieldType {
    fn from(schema_type: graphql_parser::schema::Type) -> FieldType {
        from_schema_type_inner(schema_type, false)
    }
}

fn from_schema_type_inner(inner: graphql_parser::schema::Type, non_null: bool) -> FieldType {
    match inner {
        graphql_parser::schema::Type::ListType(inner) => {
            let inner = from_schema_type_inner(*inner, false);
            let f = FieldType::Vector(Box::new(inner));
            if non_null {
                f
            } else {
                FieldType::Optional(Box::new(f))
            }
        }
        graphql_parser::schema::Type::NamedType(name) => {
            let f = FieldType::Named(Ident::new(&name, Span::call_site()));
            if non_null {
                f
            } else {
                FieldType::Optional(Box::new(f))
            }
        }
        graphql_parser::schema::Type::NonNullType(inner) => from_schema_type_inner(*inner, true),
    }
}

fn from_json_type_inner(inner: &introspection_response::TypeRef, non_null: bool) -> FieldType {
    use introspection_response::*;
    let inner = inner.clone();

    match inner.kind {
        Some(__TypeKind::NON_NULL) => {
            from_json_type_inner(&inner.of_type.expect("inner type is missing"), true)
        }
        Some(__TypeKind::LIST) => {
            let f = FieldType::Vector(Box::new(from_json_type_inner(
                &inner.of_type.expect("inner type is missing"),
                false,
            )));
            if non_null {
                f
            } else {
                FieldType::Optional(Box::new(f))
            }
        }
        Some(_) => {
            let f = FieldType::Named(Ident::new(
                &inner.name.expect("type name"),
                Span::call_site(),
            ));
            if non_null {
                f
            } else {
                FieldType::Optional(Box::new(f))
            }
        }
        None => unreachable!("non-convertible type"),
    }
}

impl ::std::convert::From<introspection_response::FullTypeFieldsType> for FieldType {
    fn from(schema_type: introspection_response::FullTypeFieldsType) -> FieldType {
        from_json_type_inner(&schema_type.type_ref, false)
    }
}

impl ::std::convert::From<introspection_response::InputValueType> for FieldType {
    fn from(schema_type: introspection_response::InputValueType) -> FieldType {
        from_json_type_inner(&schema_type.type_ref, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphql_parser::schema::Type as GqlParserType;
    use introspection_response::{FullTypeFieldsType, TypeRef, __TypeKind};

    #[test]
    fn field_type_from_graphql_parser_schema_type_works() {
        let ty = GqlParserType::NamedType("Cat".to_string());
        assert_eq!(
            FieldType::from(ty),
            FieldType::Optional(Box::new(FieldType::Named(Ident::new(
                "Cat",
                Span::call_site()
            ))))
        );

        let ty = GqlParserType::NonNullType(Box::new(GqlParserType::NamedType("Cat".to_string())));

        assert_eq!(
            FieldType::from(ty),
            FieldType::Named(Ident::new("Cat", Span::call_site()))
        );
    }

    #[test]
    fn field_type_from_introspection_response_works() {
        let ty = FullTypeFieldsType {
            type_ref: TypeRef {
                kind: Some(__TypeKind::OBJECT),
                name: Some("Cat".to_string()),
                of_type: None,
            },
        };
        assert_eq!(
            FieldType::from(ty),
            FieldType::Optional(Box::new(FieldType::Named(Ident::new(
                "Cat",
                Span::call_site()
            ))))
        );

        let ty = FullTypeFieldsType {
            type_ref: TypeRef {
                kind: Some(__TypeKind::NON_NULL),
                name: None,
                of_type: Some(Box::new(TypeRef {
                    kind: Some(__TypeKind::OBJECT),
                    name: Some("Cat".to_string()),
                    of_type: None,
                })),
            },
        };
        assert_eq!(
            FieldType::from(ty),
            FieldType::Named(Ident::new("Cat", Span::call_site()))
        );
    }
}
