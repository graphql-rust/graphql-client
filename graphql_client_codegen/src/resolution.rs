//! The responsibility of this module is to resolve and validate a query against a given schema.

use crate::schema::{ObjectRef, Schema, StoredFieldId, TypeId};

pub(crate) fn resolve(
    schema: &Schema,
    query: &graphql_parser::query::Document,
) -> anyhow::Result<ResolvedQuery> {
    let mut resolved_query: ResolvedQuery = Default::default();

    for definition in &query.definitions {
        match definition {
            graphql_parser::query::Definition::Fragment(fragment) => {
                resolve_fragment(&mut resolved_query, schema, fragment)?
            }
            graphql_parser::query::Definition::Operation(operation) => {
                resolve_operation(&mut resolved_query, schema, operation)?
            }
        }
    }

    Ok(resolved_query)
}

fn resolve_fragment(
    query: &mut ResolvedQuery,
    schema: &Schema,
    fragment: &graphql_parser::query::FragmentDefinition,
) -> anyhow::Result<()> {
    let graphql_parser::query::TypeCondition::On(on) = &fragment.type_condition;
    let on = schema.find_type(on).expect("TODO: proper error message");
    let resolved_fragment = ResolvedFragment {
        name: fragment.name.clone(),
        on,
        selection: resolve_selection(schema, on, &fragment.selection_set)?,
    };

    query.fragments.push(resolved_fragment);

    Ok(())
}

fn resolve_object_selection(
    object: ObjectRef<'_>,
    selection_set: &graphql_parser::query::SelectionSet,
) -> anyhow::Result<Vec<IdSelection>> {
    let id_selection: Vec<IdSelection> = selection_set
        .items
        .iter()
        .map(|item| -> anyhow::Result<_> {
            match item {
                graphql_parser::query::Selection::Field(field) => {
                    let field_ref = object.get_field_by_name(&field.name).ok_or_else(|| {
                        anyhow::anyhow!("No field named {} on {}", &field.name, object.name())
                    })?;
                    Ok(IdSelection::Field(
                        field_ref.id(),
                        resolve_selection(
                            object.schema(),
                            field_ref.type_id(),
                            &field.selection_set,
                        )?,
                    ))
                }
                graphql_parser::query::Selection::InlineFragment(inline) => {
                    resolve_inline_fragment(object.schema(), inline)
                }
                graphql_parser::query::Selection::FragmentSpread(fragment_spread) => Ok(
                    IdSelection::FragmentSpread(fragment_spread.fragment_name.clone()),
                ),
            }
        })
        .collect::<Result<_, _>>()?;

    Ok(id_selection)
}

fn resolve_selection(
    schema: &Schema,
    on: TypeId,
    selection_set: &graphql_parser::query::SelectionSet,
) -> anyhow::Result<Vec<IdSelection>> {
    match on {
        TypeId::Object(oid) => {
            let object = schema.object(oid);
            resolve_object_selection(object, selection_set)
        }
        TypeId::Interface(interface_id) => {
            let interface = schema.interface(interface_id);
            todo!("interface thing")
        }
        _ => anyhow::bail!("Selection set on non-object, non-interface type."),
    }
}

fn resolve_inline_fragment(
    schema: &Schema,
    inline_fragment: &graphql_parser::query::InlineFragment,
) -> anyhow::Result<IdSelection> {
    let graphql_parser::query::TypeCondition::On(on) = inline_fragment
        .type_condition
        .as_ref()
        .expect("missing type condition");
    let type_id = schema
        .find_type(on)
        .ok_or_else(|| anyhow::anyhow!("TODO: error message"))?;
    Ok(IdSelection::InlineFragment(
        type_id,
        resolve_selection(schema, type_id, &inline_fragment.selection_set)?,
    ))
}

fn resolve_operation(
    query: &mut ResolvedQuery,
    schema: &Schema,
    operation: &graphql_parser::query::OperationDefinition,
) -> anyhow::Result<()> {
    match operation {
        graphql_parser::query::OperationDefinition::Mutation(m) => {
            let on = schema.mutation_type();
            let resolved_operation: ResolvedOperation = todo!();

            query.operations.push(resolved_operation);
        }
        graphql_parser::query::OperationDefinition::Query(_) => todo!("resolve query"),
        graphql_parser::query::OperationDefinition::Subscription(_) => {
            todo!("resolve subscription")
        }
        graphql_parser::query::OperationDefinition::SelectionSet(_) => {
            unreachable!("unnamed queries are not supported")
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ResolvedFragmentId(usize);

#[derive(Debug, Default)]
pub(crate) struct ResolvedQuery {
    pub(crate) operations: Vec<ResolvedOperation>,
    fragments: Vec<ResolvedFragment>,
}

#[derive(Debug)]
struct ResolvedFragment {
    name: String,
    on: crate::schema::TypeId,
    selection: Vec<IdSelection>,
}

#[derive(Debug)]
pub(crate) struct ResolvedOperation {
    name: String,
    operation_type: crate::operations::OperationType,
    variables: Vec<ResolvedVariable>,
}

impl ResolvedOperation {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
struct ResolvedVariable {
    name: String,
    default: Option<graphql_parser::query::Value>,
    r#type: crate::schema::StoredInputFieldType,
}

#[derive(Debug, Clone)]
enum IdSelection {
    Field(StoredFieldId, Vec<IdSelection>),
    FragmentSpread(String),
    InlineFragment(TypeId, Vec<IdSelection>),
}
