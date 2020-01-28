// use crate::enums::ENUMS_PREFIX;
// use crate::query::QueryContext;
use crate::schema::DEFAULT_SCALARS;
use graphql_introspection_query::introspection_response;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub(crate) fn field_type_to_rust() -> TokenStream {
    todo!()
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub(crate) enum GraphqlTypeQualifier {
    Required,
    List,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct FieldType<'a> {
    /// The type name of the field.
    ///
    /// e.g. for `[Int]!`, this would return `Int`.
    name: &'a str,
    /// An ordered list of qualifiers, from outer to inner.
    ///
    /// e.g. `[Int]!` would have `vec![List, Optional]`, but `[Int!]` would have `vec![Optional,
    /// List]`.
    qualifiers: Vec<GraphqlTypeQualifier>,
}

impl<'a> FieldType<'a> {
    pub(crate) fn new(name: &'a str) -> Self {
        FieldType {
            name,
            qualifiers: Vec::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn list(mut self) -> Self {
        self.qualifiers.insert(0, GraphqlTypeQualifier::List);
        self
    }

    #[cfg(test)]
    pub(crate) fn nonnull(mut self) -> Self {
        self.qualifiers.insert(0, GraphqlTypeQualifier::Required);
        self
    }

    /// Takes a field type with its name.
    // pub(crate) fn to_rust(&self, context: &QueryContext<'_>, prefix: &str) -> TokenStream {
    //     todo!()
    //     // let prefix: &str = if prefix.is_empty() {
    //     //     self.inner_name_str()
    //     // } else {
    //     //     prefix
    //     // };

    //     // let full_name = {
    //     //     if context
    //     //         .schema
    //     //         .scalars
    //     //         .get(&self.name)
    //     //         .map(|s| s.is_required.set(true))
    //     //         .is_some()
    //     //         || DEFAULT_SCALARS.iter().any(|elem| elem == &self.name)
    //     //     {
    //     //         self.name.to_string()
    //     //     } else if context
    //     //         .schema
    //     //         .enums
    //     //         .get(&self.name)
    //     //         .map(|enm| enm.is_required.set(true))
    //     //         .is_some()
    //     //     {
    //     //         format!("{}{}", ENUMS_PREFIX, self.name)
    //     //     } else {
    //     //         if prefix.is_empty() {
    //     //             panic!("Empty prefix for {:?}", self);
    //     //         }
    //     //         prefix.to_string()
    //     //     }
    //     // };

    //     // let norm = context.normalization;
    //     // let full_name = norm.field_type(crate::shared::keyword_replace(&full_name));

    //     // let full_name = Ident::new(&full_name, Span::call_site());
    //     // let mut qualified = quote!(#full_name);

    //     // let mut non_null = false;

    //     // // Note: we iterate over qualifiers in reverse because it is more intuitive. This
    //     // // means we start from the _inner_ type and make our way to the outside.
    //     // for qualifier in self.qualifiers.iter().rev() {
    //     //     match (non_null, qualifier) {
    //     //         // We are in non-null context, and we wrap the non-null type into a list.
    //     //         // We switch back to null context.
    //     //         (true, GraphqlTypeQualifier::List) => {
    //     //             qualified = quote!(Vec<#qualified>);
    //     //             non_null = false;
    //     //         }
    //     //         // We are in nullable context, and we wrap the nullable type into a list.
    //     //         (false, GraphqlTypeQualifier::List) => {
    //     //             qualified = quote!(Vec<Option<#qualified>>);
    //     //         }
    //     //         // We are in non-nullable context, but we can't double require a type
    //     //         // (!!).
    //     //         (true, GraphqlTypeQualifier::Required) => panic!("double required annotation"),
    //     //         // We are in nullable context, and we switch to non-nullable context.
    //     //         (false, GraphqlTypeQualifier::Required) => {
    //     //             non_null = true;
    //     //         }
    //     //     }
    //     // }

    //     // // If we are in nullable context at the end of the iteration, we wrap the whole
    //     // // type with an Option.
    //     // if !non_null {
    //     //     qualified = quote!(Option<#qualified>);
    //     // }

    //     // qualified
    // }

    /// Return the innermost name - we mostly use this for looking types up in our Schema struct.
    pub fn inner_name_str(&self) -> &str {
        self.name
    }

    /// Is the type nullable?
    ///
    /// Note: a list of nullable values is considered nullable only if the list itself is nullable.
    pub fn is_optional(&self) -> bool {
        if let Some(qualifier) = self.qualifiers.get(0) {
            qualifier != &GraphqlTypeQualifier::Required
        } else {
            true
        }
    }

    /// A type is indirected if it is a (flat or nested) list type, optional or not.
    ///
    /// We use this to determine whether a type needs to be boxed for recursion.
    pub fn is_indirected(&self) -> bool {
        self.qualifiers
            .iter()
            .any(|qualifier| qualifier == &GraphqlTypeQualifier::List)
    }
}

impl<'schema> std::convert::From<&'schema graphql_parser::schema::Type> for FieldType<'schema> {
    fn from(schema_type: &'schema graphql_parser::schema::Type) -> FieldType<'schema> {
        todo!()
        // from_schema_type_inner(schema_type)
    }
}

pub(crate) fn graphql_parser_depth(schema_type: &graphql_parser::schema::Type) -> usize {
    match schema_type {
        graphql_parser::schema::Type::ListType(inner) => 1 + graphql_parser_depth(inner),
        graphql_parser::schema::Type::NonNullType(inner) => 1 + graphql_parser_depth(inner),
        graphql_parser::schema::Type::NamedType(_) => 0,
    }
}

fn json_type_qualifiers_depth(typeref: &introspection_response::TypeRef) -> usize {
    use graphql_introspection_query::introspection_response::*;

    match (typeref.kind.as_ref(), typeref.of_type.as_ref()) {
        (Some(__TypeKind::NON_NULL), Some(inner)) => 1 + json_type_qualifiers_depth(inner),
        (Some(__TypeKind::LIST), Some(inner)) => 1 + json_type_qualifiers_depth(inner),
        (Some(_), None) => 0,
        _ => panic!("Non-convertible type in JSON schema: {:?}", typeref),
    }
}

fn from_json_type_inner(inner: &introspection_response::TypeRef) -> FieldType<'_> {
    use graphql_introspection_query::introspection_response::*;

    let qualifiers_depth = json_type_qualifiers_depth(inner);
    let mut qualifiers = Vec::with_capacity(qualifiers_depth);

    let mut inner = inner;

    loop {
        match (
            inner.kind.as_ref(),
            inner.of_type.as_ref(),
            inner.name.as_ref(),
        ) {
            (Some(__TypeKind::NON_NULL), Some(new_inner), _) => {
                qualifiers.push(GraphqlTypeQualifier::Required);
                inner = &new_inner;
            }
            (Some(__TypeKind::LIST), Some(new_inner), _) => {
                qualifiers.push(GraphqlTypeQualifier::List);
                inner = &new_inner;
            }
            (Some(_), None, Some(name)) => return FieldType { name, qualifiers },
            _ => panic!("Non-convertible type in JSON schema: {:?}", inner),
        }
    }
}

impl<'schema> std::convert::From<&'schema introspection_response::FullTypeFieldsType>
    for FieldType<'schema>
{
    fn from(
        schema_type: &'schema introspection_response::FullTypeFieldsType,
    ) -> FieldType<'schema> {
        from_json_type_inner(&schema_type.type_ref)
    }
}

impl<'a> std::convert::From<&'a introspection_response::InputValueType> for FieldType<'a> {
    fn from(schema_type: &'a introspection_response::InputValueType) -> FieldType<'a> {
        from_json_type_inner(&schema_type.type_ref)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use graphql_introspection_query::introspection_response::{
//         FullTypeFieldsType, TypeRef, __TypeKind,
//     };
//     use graphql_parser::schema::Type as GqlParserType;

//     #[test]
//     fn field_type_from_graphql_parser_schema_type_works() {
//         let ty = GqlParserType::NamedType("Cat".to_owned());
//         assert_eq!(FieldType::from(&ty), FieldType::new("Cat"));

//         let ty = GqlParserType::NonNullType(Box::new(GqlParserType::NamedType("Cat".to_owned())));

//         assert_eq!(FieldType::from(&ty), FieldType::new("Cat").nonnull());
//     }

//     #[test]
//     fn field_type_from_introspection_response_works() {
//         let ty = FullTypeFieldsType {
//             type_ref: TypeRef {
//                 kind: Some(__TypeKind::OBJECT),
//                 name: Some("Cat".into()),
//                 of_type: None,
//             },
//         };
//         assert_eq!(FieldType::from(&ty), FieldType::new("Cat"));

//         let ty = FullTypeFieldsType {
//             type_ref: TypeRef {
//                 kind: Some(__TypeKind::NON_NULL),
//                 name: None,
//                 of_type: Some(Box::new(TypeRef {
//                     kind: Some(__TypeKind::OBJECT),
//                     name: Some("Cat".into()),
//                     of_type: None,
//                 })),
//             },
//         };
//         assert_eq!(FieldType::from(&ty), FieldType::new("Cat").nonnull());
//     }
// }
