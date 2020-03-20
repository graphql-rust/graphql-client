use graphql_introspection_query::introspection_response;

#[derive(Clone, Debug, PartialEq, Hash)]
pub(crate) enum GraphqlTypeQualifier {
    Required,
    List,
}

impl GraphqlTypeQualifier {
    pub(crate) fn is_required(&self) -> bool {
        *self == GraphqlTypeQualifier::Required
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
