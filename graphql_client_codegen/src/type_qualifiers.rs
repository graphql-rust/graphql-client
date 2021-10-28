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

pub fn graphql_parser_depth<'a, T>(schema_type: &graphql_parser::schema::Type<'a, T>) -> usize
where
    T: graphql_parser::query::Text<'a>,
{
    match schema_type {
        graphql_parser::schema::Type::ListType(inner) => 1 + graphql_parser_depth(inner),
        graphql_parser::schema::Type::NonNullType(inner) => 1 + graphql_parser_depth(inner),
        graphql_parser::schema::Type::NamedType(_) => 0,
    }
}
