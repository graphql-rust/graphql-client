//! The responsibility of this module is to resolve and validate a query against a given schema.

use crate::schema::{Schema, StoredFieldId, TypeId};

pub(crate) fn resolve(
    schema: &crate::schema::Schema,
    query: &graphql_parser::query::Document,
) -> anyhow::Result<ResolvedQuery> {
    let mut resolved_query: ResolvedQuery = Default::default();

    for definition in &query.definitions {
        match definition {
            graphql_parser::query::Definition::Fragment(fragment) => (),
            graphql_parser::query::Definition::Operation(operation) => (),
        }
    }

    todo!("resolve")
}

#[derive(Debug, Clone, Copy)]
struct ResolvedFragmentId(usize);

#[derive(Debug, Default)]
pub(crate) struct ResolvedQuery {
    operations: Vec<ResolvedOperation>,
    fragments: Vec<ResolvedFragment>,
}

#[derive(Debug)]
struct ResolvedFragment {
    name: String,
    on: crate::schema::TypeId,
    selection: IdSelection,
}

#[derive(Debug)]
struct ResolvedOperation {
    name: String,
    operation_type: crate::operations::OperationType,
    variables: Vec<ResolvedVariable>,
}

#[derive(Debug)]
struct ResolvedVariable {
    name: String,
    default: Option<graphql_parser::query::Value>,
    r#type: crate::schema::StoredInputFieldType,
}

#[derive(Debug, Clone)]
enum IdSelection {
    Field(StoredFieldId),
    FragmentSpread(ResolvedFragmentId),
    InlineFragment(TypeId, Vec<IdSelection>),
}
