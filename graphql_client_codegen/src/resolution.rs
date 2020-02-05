//! The responsibility of this module is to resolve and validate a query against a given schema.

use crate::schema::resolve_field_type;
use crate::schema::EnumRef;
use crate::schema::FieldRef;
use crate::schema::ScalarRef;
use crate::schema::StoredFieldType;
use crate::schema::TypeRef;
use crate::schema::{ObjectRef, Schema, StoredFieldId, TypeId};
use std::collections::HashSet;

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
        other => {
            anyhow::ensure!(
                selection_set.items.is_empty(),
                "Selection set on non-object, non-interface type. ({:?})",
                other
            );
            Ok(Vec::new())
        }
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
            let resolved_operation: ResolvedOperation = ResolvedOperation {
                name: m.name.as_ref().expect("mutation without name").to_owned(),
                operation_type: crate::operations::OperationType::Mutation,
                variables: resolve_variables(&m.variable_definitions, schema)?,
                selection: resolve_object_selection(on, &m.selection_set)?,
            };

            query.operations.push(resolved_operation);
        }
        graphql_parser::query::OperationDefinition::Query(q) => {
            let on = schema.query_type();

            let resolved_operation: ResolvedOperation = ResolvedOperation {
                name: q.name.as_ref().expect("query without name").to_owned(),
                operation_type: crate::operations::OperationType::Query,
                variables: resolve_variables(&q.variable_definitions, schema)?,
                selection: resolve_object_selection(on, &q.selection_set)?,
            };

            query.operations.push(resolved_operation);
        }
        graphql_parser::query::OperationDefinition::Subscription(s) => {
            let on = schema.subscription_type();
            let resolved_operation: ResolvedOperation = ResolvedOperation {
                name: s
                    .name
                    .as_ref()
                    .expect("subscription without name")
                    .to_owned(),
                operation_type: crate::operations::OperationType::Subscription,
                variables: resolve_variables(&s.variable_definitions, schema)?,
                selection: resolve_object_selection(on, &s.selection_set)?,
            };

            query.operations.push(resolved_operation);
        }
        graphql_parser::query::OperationDefinition::SelectionSet(_) => {
            unreachable!("unnamed queries are not supported")
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct Operation<'a> {
    operation_id: usize,
    schema: &'a Schema,
    query: &'a ResolvedQuery,
}

impl<'a> Operation<'a> {
    pub(crate) fn new(
        operation_id: usize,
        schema: &'a Schema,
        query: &'a ResolvedQuery,
    ) -> Operation<'a> {
        Operation {
            operation_id,
            schema,
            query,
        }
    }

    fn get(&self) -> &'a ResolvedOperation {
        self.query.operations.get(self.operation_id).unwrap()
    }

    fn name(&self) -> &'a str {
        self.get().name()
    }

    fn selection(&self) -> impl Iterator<Item = Selection<'_>> {
        self.get()
            .selection
            .iter()
            .map(move |id_selection| id_selection.upgrade(&self.schema, &self.query))
    }

    pub(crate) fn schema(&self) -> &'a Schema {
        self.schema
    }

    pub(crate) fn query(&self) -> &'a ResolvedQuery {
        self.query
    }

    pub(crate) fn all_used_types(&self) -> UsedTypes {
        let mut all_used_types = UsedTypes::default();

        for selection in self.selection() {
            selection.collect_used_types(&mut all_used_types);
        }

        all_used_types
    }
}

#[derive(Debug)]
pub(crate) struct ResolvedOperation {
    name: String,
    operation_type: crate::operations::OperationType,
    variables: Vec<ResolvedVariable>,
    selection: Vec<IdSelection>,
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
    r#type: StoredFieldType,
}

#[derive(Debug, Clone)]
enum IdSelection {
    Field(StoredFieldId, Vec<IdSelection>),
    FragmentSpread(String),
    InlineFragment(TypeId, Vec<IdSelection>),
}

impl IdSelection {
    fn upgrade<'a>(&self, schema: &'a Schema, query: &'a ResolvedQuery) -> Selection<'a> {
        match self {
            IdSelection::Field(id, selection) => Selection::Field(
                schema.field(*id),
                selection
                    .iter()
                    .map(|selection| selection.upgrade(schema, query))
                    .collect(),
            ),
            IdSelection::FragmentSpread(name) => Selection::FragmentSpread(Fragment {
                fragment_id: ResolvedFragmentId(
                    query
                        .fragments
                        .iter()
                        .position(|frag| frag.name.as_str() == name.as_str())
                        .unwrap(),
                ),
                query,
                schema,
            }),
            IdSelection::InlineFragment(typeid, selection) => Selection::InlineFragment(
                typeid.upgrade(schema),
                selection
                    .iter()
                    .map(|sel| sel.upgrade(schema, query))
                    .collect(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Selection<'a> {
    Field(FieldRef<'a>, Vec<Selection<'a>>),
    FragmentSpread(Fragment<'a>),
    InlineFragment(TypeRef<'a>, Vec<Selection<'a>>),
}

impl Selection<'_> {
    fn collect_used_types(&self, used_types: &mut UsedTypes) {
        match self {
            Selection::Field(field, selection) => {
                used_types.types.insert(field.type_id());

                selection
                    .iter()
                    .for_each(|selection| selection.collect_used_types(used_types));
            }
            Selection::FragmentSpread(fragment) => {
                used_types.fragments.insert(fragment.fragment_id);
                fragment
                    .selection()
                    .for_each(|selection| selection.collect_used_types(used_types))
            }
            Selection::InlineFragment(on, selection) => {
                used_types.types.insert(on.type_id());

                selection
                    .iter()
                    .for_each(|selection| selection.collect_used_types(used_types))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Fragment<'a> {
    fragment_id: ResolvedFragmentId,
    query: &'a ResolvedQuery,
    schema: &'a Schema,
}

impl Fragment<'_> {
    fn get(&self) -> &ResolvedFragment {
        self.query.fragments.get(self.fragment_id.0).unwrap()
    }

    pub(crate) fn selection(&self) -> impl Iterator<Item = Selection<'_>> {
        self.get()
            .selection
            .iter()
            .map(move |selection| selection.upgrade(&self.schema, &self.query))
    }
}

#[derive(Debug, Default)]
pub(crate) struct UsedTypes {
    types: HashSet<TypeId>,
    fragments: HashSet<ResolvedFragmentId>,
}

impl UsedTypes {
    pub(crate) fn scalars<'s, 'a: 's>(
        &'s self,
        schema: &'a Schema,
    ) -> impl Iterator<Item = ScalarRef<'a>> + 's {
        self.types
            .iter()
            .filter_map(TypeId::as_scalar_id)
            .map(move |scalar_id| schema.scalar(scalar_id))
            .filter(|scalar| !crate::schema::DEFAULT_SCALARS.contains(&scalar.name()))
    }

    pub(crate) fn enums<'a, 'schema: 'a>(
        &'a self,
        schema: &'schema Schema,
    ) -> impl Iterator<Item = EnumRef<'schema>> + 'a {
        self.types
            .iter()
            .filter_map(TypeId::as_enum_id)
            .map(move |enum_id| schema.r#enum(enum_id))
    }
}

fn resolve_variables(
    variables: &[graphql_parser::query::VariableDefinition],
    schema: &Schema,
) -> Result<Vec<ResolvedVariable>, anyhow::Error> {
    variables
        .iter()
        .map(|var| {
            Ok(ResolvedVariable {
                name: var.name.clone(),
                default: var.default_value.clone(),
                r#type: resolve_field_type(schema, &var.var_type),
            })
        })
        .collect()
}
