//! The responsibility of this module is to resolve and validate a query
//! against a given schema.

use crate::{
    constants::TYPENAME_FIELD,
    schema::{
        resolve_field_type, EnumRef, FieldRef, InterfaceRef, ObjectId, ObjectRef, ScalarRef,
        Schema, StoredFieldId, StoredFieldType, TypeId, TypeRef, UnionRef,
    },
};
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
                    if field.name == TYPENAME_FIELD {
                        return Ok(IdSelection::Typename);
                    }

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
                object_id: on.id(),
                name: m.name.as_ref().expect("mutation without name").to_owned(),
                operation_type: crate::operations::OperationType::Mutation,
                variables: resolve_variables(
                    &m.variable_definitions,
                    schema,
                    query.operations.len(),
                )?,
                selection: resolve_object_selection(on, &m.selection_set)?,
            };

            query.operations.push(resolved_operation);
        }
        graphql_parser::query::OperationDefinition::Query(q) => {
            let on = schema.query_type();

            let resolved_operation: ResolvedOperation = ResolvedOperation {
                name: q.name.as_ref().expect("query without name").to_owned(),
                operation_type: crate::operations::OperationType::Query,
                variables: resolve_variables(
                    &q.variable_definitions,
                    schema,
                    query.operations.len(),
                )?,
                object_id: on.id(),
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
                variables: resolve_variables(
                    &s.variable_definitions,
                    schema,
                    query.operations.len(),
                )?,
                object_id: on.id(),
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
        let operation = self.get();
        operation
            .selection
            .iter()
            .map(move |id_selection| id_selection.upgrade(&self.schema, &self.query, None))
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

    pub(crate) fn has_no_variables(&self) -> bool {
        self.get().variables.is_empty()
    }

    pub(crate) fn variables<'b>(&'b self) -> impl Iterator<Item = Variable<'a>> + 'b {
        self.get()
            .variables
            .iter()
            .enumerate()
            .map(move |(idx, _)| Variable {
                variable_id: idx,
                operation: *self,
            })
    }
}

#[derive(Debug)]
pub(crate) struct ResolvedOperation {
    name: String,
    operation_type: crate::operations::OperationType,
    variables: Vec<ResolvedVariable>,
    selection: Vec<IdSelection>,
    object_id: ObjectId,
}

impl ResolvedOperation {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug)]
struct ResolvedVariable {
    operation_id: usize,
    name: String,
    default: Option<graphql_parser::query::Value>,
    r#type: StoredFieldType,
}

pub(crate) struct Variable<'a> {
    operation: Operation<'a>,
    variable_id: usize,
}

impl<'a> Variable<'a> {
    fn get(&self) -> &'a ResolvedVariable {
        self.operation
            .get()
            .variables
            .get(self.variable_id)
            .unwrap()
    }

    pub(crate) fn name(&self) -> &'a str {
        &self.get().name
    }
}

#[derive(Debug, Clone)]
enum IdSelection {
    Typename,
    Field(StoredFieldId, Vec<IdSelection>),
    FragmentSpread(String),
    InlineFragment(TypeId, Vec<IdSelection>),
}

impl IdSelection {
    fn upgrade<'a>(
        &self,
        schema: &'a Schema,
        query: &'a ResolvedQuery,
        parent: Option<FieldRef<'a>>,
    ) -> Selection<'a> {
        let selection_set = match self {
            IdSelection::Typename => SelectionSet::Typename,
            IdSelection::Field(id, selection) => {
                let field = schema.field(*id);
                SelectionSet::Field(
                    field,
                    selection
                        .iter()
                        .map(|selection| selection.upgrade(schema, query, Some(field)))
                        .collect(),
                )
            }
            IdSelection::FragmentSpread(name) => SelectionSet::FragmentSpread(Fragment {
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
            IdSelection::InlineFragment(typeid, selection) => SelectionSet::InlineFragment(
                typeid.upgrade(schema),
                selection
                    .iter()
                    .map(|sel| sel.upgrade(schema, query, parent))
                    .collect(),
            ),
        };

        Selection {
            selection_set,
            parent,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Selection<'a> {
    parent: Option<FieldRef<'a>>,
    selection_set: SelectionSet<'a>,
}

#[derive(Debug, Clone)]
enum SelectionSet<'a> {
    Typename,
    Field(FieldRef<'a>, Vec<Selection<'a>>),
    FragmentSpread(Fragment<'a>),
    InlineFragment(TypeRef<'a>, Vec<Selection<'a>>),
}

impl Selection<'_> {
    fn collect_used_types(&self, used_types: &mut UsedTypes) {
        match &self.selection_set {
            SelectionSet::Typename => (),
            SelectionSet::Field(field, selection) => {
                used_types.types.insert(field.type_id());

                selection
                    .iter()
                    .for_each(|selection| selection.collect_used_types(used_types));
            }
            SelectionSet::FragmentSpread(fragment) => {
                used_types.fragments.insert(fragment.fragment_id);
                fragment
                    .selection()
                    .for_each(|selection| selection.collect_used_types(used_types))
            }
            SelectionSet::InlineFragment(on, selection) => {
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
            .map(move |selection| selection.upgrade(&self.schema, &self.query, None))
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
    operation_id: usize,
) -> Result<Vec<ResolvedVariable>, anyhow::Error> {
    variables
        .iter()
        .map(|var| {
            Ok(ResolvedVariable {
                operation_id,
                name: var.name.clone(),
                default: var.default_value.clone(),
                r#type: resolve_field_type(schema, &var.var_type),
            })
        })
        .collect()
}
