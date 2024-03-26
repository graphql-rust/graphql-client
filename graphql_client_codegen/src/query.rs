//! The responsibility of this module is to bind and validate a query
//! against a given schema.

mod fragments;
mod operations;
mod selection;
mod validation;

pub(crate) use fragments::{fragment_is_recursive, ResolvedFragment};
pub(crate) use operations::ResolvedOperation;
pub(crate) use selection::*;

use crate::{
    constants::TYPENAME_FIELD,
    normalization::Normalization,
    schema::{
        resolve_field_type, EnumId, InputId, ScalarId, Schema, StoredEnum, StoredFieldType,
        StoredInputType, StoredScalar, TypeId, UnionId,
    },
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

#[derive(Debug)]
pub(crate) struct QueryValidationError {
    message: String,
}

impl Display for QueryValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for QueryValidationError {}

impl QueryValidationError {
    pub(crate) fn new(message: String) -> Self {
        QueryValidationError { message }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SelectionId(u32);
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct OperationId(u32);

impl OperationId {
    pub(crate) fn new(idx: usize) -> Self {
        OperationId(idx as u32)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ResolvedFragmentId(u32);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct VariableId(u32);

pub(crate) fn resolve<'doc, T>(
    schema: &Schema,
    query: &graphql_parser::query::Document<'doc, T>,
) -> Result<Query, QueryValidationError>
where
    T: graphql_parser::query::Text<'doc>,
{
    let mut resolved_query: Query = Default::default();

    create_roots(&mut resolved_query, query, schema)?;

    // Then resolve the selections.
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

    // Validation: to be expanded and factored out.
    validation::validate_typename_presence(&BoundQuery {
        query: &resolved_query,
        schema,
    })?;

    for (selection_id, _) in resolved_query.selections() {
        selection::validate_type_conditions(
            selection_id,
            &BoundQuery {
                query: &resolved_query,
                schema,
            },
        )?
    }

    Ok(resolved_query)
}

fn create_roots<'doc, T>(
    resolved_query: &mut Query,
    query: &graphql_parser::query::Document<'doc, T>,
    schema: &Schema,
) -> Result<(), QueryValidationError>
where
    T: graphql_parser::query::Text<'doc>,
{
    // First, give ids to all fragments and operations.
    for definition in &query.definitions {
        match definition {
            graphql_parser::query::Definition::Fragment(fragment) => {
                let graphql_parser::query::TypeCondition::On(on) = &fragment.type_condition;
                resolved_query.fragments.push(ResolvedFragment {
                    name: fragment.name.as_ref().into(),
                    on: schema.find_type(on.as_ref()).ok_or_else(|| {
                        QueryValidationError::new(format!(
                            "Could not find type {} for fragment {} in schema.",
                            on.as_ref(),
                            fragment.name.as_ref(),
                        ))
                    })?,
                    selection_set: Vec::new(),
                });
            }
            graphql_parser::query::Definition::Operation(
                graphql_parser::query::OperationDefinition::Mutation(m),
            ) => {
                let on = schema.mutation_type().ok_or_else(|| {
                    QueryValidationError::new(
                        "Query contains a mutation operation, but the schema has no mutation type."
                            .to_owned(),
                    )
                })?;
                let resolved_operation: ResolvedOperation = ResolvedOperation {
                    object_id: on,
                    name: m
                        .name
                        .as_ref()
                        .expect("mutation without name")
                        .as_ref()
                        .into(),
                    _operation_type: operations::OperationType::Mutation,
                    selection_set: Vec::with_capacity(m.selection_set.items.len()),
                };

                resolved_query.operations.push(resolved_operation);
            }
            graphql_parser::query::Definition::Operation(
                graphql_parser::query::OperationDefinition::Query(q),
            ) => {
                let on = schema.query_type();
                let resolved_operation: ResolvedOperation = ResolvedOperation {
                    name: q.name.as_ref().expect("query without name. Instead of `query (...)`, write `query SomeName(...)` in your .graphql file").as_ref().into(),
                    _operation_type: operations::OperationType::Query,
                    object_id: on,
                    selection_set: Vec::with_capacity(q.selection_set.items.len()),
                };

                resolved_query.operations.push(resolved_operation);
            }
            graphql_parser::query::Definition::Operation(
                graphql_parser::query::OperationDefinition::Subscription(s),
            ) => {
                let on = schema.subscription_type().ok_or_else(|| {
                    QueryValidationError::new(
                        "Query contains a subscription operation, but the schema has no subscription type.".to_owned()
                    )
                })?;

                if s.selection_set.items.len() != 1 {
                    return Err(QueryValidationError::new(
                        crate::constants::MULTIPLE_SUBSCRIPTION_FIELDS_ERROR.to_owned(),
                    ));
                }

                let resolved_operation: ResolvedOperation = ResolvedOperation {
                    name: s
                        .name
                        .as_ref()
                        .expect("subscription without name")
                        .as_ref()
                        .into(),
                    _operation_type: operations::OperationType::Subscription,
                    object_id: on,
                    selection_set: Vec::with_capacity(s.selection_set.items.len()),
                };

                resolved_query.operations.push(resolved_operation);
            }
            graphql_parser::query::Definition::Operation(
                graphql_parser::query::OperationDefinition::SelectionSet(_),
            ) => {
                return Err(QueryValidationError::new(
                    crate::constants::SELECTION_SET_AT_ROOT.to_owned(),
                ))
            }
        }
    }

    Ok(())
}

fn resolve_fragment<'doc, T>(
    query: &mut Query,
    schema: &Schema,
    fragment_definition: &graphql_parser::query::FragmentDefinition<'doc, T>,
) -> Result<(), QueryValidationError>
where
    T: graphql_parser::query::Text<'doc>,
{
    let graphql_parser::query::TypeCondition::On(on) = &fragment_definition.type_condition;
    let on = schema.find_type(on.as_ref()).ok_or_else(|| {
        QueryValidationError::new(format!(
            "Could not find type `{}` referenced by fragment `{}`",
            on.as_ref(),
            fragment_definition.name.as_ref(),
        ))
    })?;

    let (id, _) = query
        .find_fragment(fragment_definition.name.as_ref())
        .ok_or_else(|| {
            QueryValidationError::new(format!(
                "Could not find fragment `{}`.",
                fragment_definition.name.as_ref()
            ))
        })?;

    resolve_selection(
        query,
        on,
        &fragment_definition.selection_set,
        SelectionParent::Fragment(id),
        schema,
    )?;

    Ok(())
}

fn resolve_union_selection<'doc, T>(
    query: &mut Query,
    _union_id: UnionId,
    selection_set: &graphql_parser::query::SelectionSet<'doc, T>,
    parent: SelectionParent,
    schema: &Schema,
) -> Result<(), QueryValidationError>
where
    T: graphql_parser::query::Text<'doc>,
{
    for item in selection_set.items.iter() {
        match item {
            graphql_parser::query::Selection::Field(field) => {
                if field.name.as_ref() == TYPENAME_FIELD {
                    let id = query.push_selection(Selection::Typename, parent);
                    parent.add_to_selection_set(query, id);
                } else {
                    return Err(QueryValidationError::new(format!(
                        "Invalid field selection on union field ({:?})",
                        parent
                    )));
                }
            }
            graphql_parser::query::Selection::InlineFragment(inline_fragment) => {
                let selection_id = resolve_inline_fragment(query, schema, inline_fragment, parent)?;
                parent.add_to_selection_set(query, selection_id);
            }
            graphql_parser::query::Selection::FragmentSpread(fragment_spread) => {
                let (fragment_id, _fragment) = query
                    .find_fragment(fragment_spread.fragment_name.as_ref())
                    .ok_or_else(|| {
                        QueryValidationError::new(format!(
                            "Could not find fragment `{}` referenced by fragment spread.",
                            fragment_spread.fragment_name.as_ref()
                        ))
                    })?;

                let id = query.push_selection(Selection::FragmentSpread(fragment_id), parent);

                parent.add_to_selection_set(query, id);
            }
        }
    }

    Ok(())
}

fn resolve_object_selection<'a, 'doc, T>(
    query: &mut Query,
    object: &dyn crate::schema::ObjectLike,
    selection_set: &graphql_parser::query::SelectionSet<'doc, T>,
    parent: SelectionParent,
    schema: &'a Schema,
) -> Result<(), QueryValidationError>
where
    T: graphql_parser::query::Text<'doc>,
{
    for item in selection_set.items.iter() {
        match item {
            graphql_parser::query::Selection::Field(field) => {
                if field.name.as_ref() == TYPENAME_FIELD {
                    let id = query.push_selection(Selection::Typename, parent);
                    parent.add_to_selection_set(query, id);
                    continue;
                }

                let (field_id, schema_field) = object
                    .get_field_by_name(field.name.as_ref(), schema)
                    .ok_or_else(|| {
                        QueryValidationError::new(format!(
                            "No field named {} on {}",
                            field.name.as_ref(),
                            object.name()
                        ))
                    })?;

                let id = query.push_selection(
                    Selection::Field(SelectedField {
                        alias: field.alias.as_ref().map(|alias| alias.as_ref().into()),
                        field_id,
                        selection_set: Vec::with_capacity(selection_set.items.len()),
                    }),
                    parent,
                );

                resolve_selection(
                    query,
                    schema_field.r#type.id,
                    &field.selection_set,
                    SelectionParent::Field(id),
                    schema,
                )?;

                parent.add_to_selection_set(query, id);
            }
            graphql_parser::query::Selection::InlineFragment(inline) => {
                let selection_id = resolve_inline_fragment(query, schema, inline, parent)?;

                parent.add_to_selection_set(query, selection_id);
            }
            graphql_parser::query::Selection::FragmentSpread(fragment_spread) => {
                let (fragment_id, _fragment) = query
                    .find_fragment(fragment_spread.fragment_name.as_ref())
                    .ok_or_else(|| {
                        QueryValidationError::new(format!(
                            "Could not find fragment `{}` referenced by fragment spread.",
                            fragment_spread.fragment_name.as_ref()
                        ))
                    })?;

                let id = query.push_selection(Selection::FragmentSpread(fragment_id), parent);

                parent.add_to_selection_set(query, id);
            }
        }
    }

    Ok(())
}

fn resolve_selection<'doc, T>(
    ctx: &mut Query,
    on: TypeId,
    selection_set: &graphql_parser::query::SelectionSet<'doc, T>,
    parent: SelectionParent,
    schema: &Schema,
) -> Result<(), QueryValidationError>
where
    T: graphql_parser::query::Text<'doc>,
{
    match on {
        TypeId::Object(oid) => {
            let object = schema.get_object(oid);
            resolve_object_selection(ctx, object, selection_set, parent, schema)?;
        }
        TypeId::Interface(interface_id) => {
            let interface = schema.get_interface(interface_id);
            resolve_object_selection(ctx, interface, selection_set, parent, schema)?;
        }
        TypeId::Union(union_id) => {
            resolve_union_selection(ctx, union_id, selection_set, parent, schema)?;
        }
        other => {
            if !selection_set.items.is_empty() {
                return Err(QueryValidationError::new(format!(
                    "Selection set on non-object, non-interface type. ({:?})",
                    other
                )));
            }
        }
    };

    Ok(())
}

fn resolve_inline_fragment<'doc, T>(
    query: &mut Query,
    schema: &Schema,
    inline_fragment: &graphql_parser::query::InlineFragment<'doc, T>,
    parent: SelectionParent,
) -> Result<SelectionId, QueryValidationError>
where
    T: graphql_parser::query::Text<'doc>,
{
    let graphql_parser::query::TypeCondition::On(on) = inline_fragment
        .type_condition
        .as_ref()
        .expect("missing type condition on inline fragment");
    let type_id = schema.find_type(on.as_ref()).ok_or_else(|| {
        QueryValidationError::new(format!(
            "Could not find type `{}` referenced by inline fragment.",
            on.as_ref()
        ))
    })?;

    let id = query.push_selection(
        Selection::InlineFragment(InlineFragment {
            type_id,
            selection_set: Vec::with_capacity(inline_fragment.selection_set.items.len()),
        }),
        parent,
    );

    resolve_selection(
        query,
        type_id,
        &inline_fragment.selection_set,
        SelectionParent::InlineFragment(id),
        schema,
    )?;

    Ok(id)
}

fn resolve_operation<'doc, T>(
    query: &mut Query,
    schema: &Schema,
    operation: &graphql_parser::query::OperationDefinition<'doc, T>,
) -> Result<(), QueryValidationError>
where
    T: graphql_parser::query::Text<'doc>,
{
    match operation {
        graphql_parser::query::OperationDefinition::Mutation(m) => {
            let on = schema.mutation_type().ok_or_else(|| {
                QueryValidationError::new(
                    "Query contains a mutation operation, but the schema has no mutation type."
                        .to_owned(),
                )
            })?;
            let on = schema.get_object(on);

            let (id, _) = query
                .find_operation(m.name.as_ref().map(|name| name.as_ref()).unwrap())
                .unwrap();

            resolve_variables(query, &m.variable_definitions, schema, id);
            resolve_object_selection(
                query,
                on,
                &m.selection_set,
                SelectionParent::Operation(id),
                schema,
            )?;
        }
        graphql_parser::query::OperationDefinition::Query(q) => {
            let on = schema.get_object(schema.query_type());
            let (id, _) = query
                .find_operation(q.name.as_ref().map(|name| name.as_ref()).unwrap())
                .unwrap();

            resolve_variables(query, &q.variable_definitions, schema, id);
            resolve_object_selection(
                query,
                on,
                &q.selection_set,
                SelectionParent::Operation(id),
                schema,
            )?;
        }
        graphql_parser::query::OperationDefinition::Subscription(s) => {
            let on = schema.subscription_type().ok_or_else(|| QueryValidationError::new("Query contains a subscription operation, but the schema has no subscription type.".into()))?;
            let on = schema.get_object(on);
            let (id, _) = query
                .find_operation(s.name.as_ref().map(|name| name.as_ref()).unwrap())
                .unwrap();

            resolve_variables(query, &s.variable_definitions, schema, id);
            resolve_object_selection(
                query,
                on,
                &s.selection_set,
                SelectionParent::Operation(id),
                schema,
            )?;
        }
        graphql_parser::query::OperationDefinition::SelectionSet(_) => {
            unreachable!("unnamed queries are not supported")
        }
    }

    Ok(())
}

#[derive(Default)]
pub(crate) struct Query {
    fragments: Vec<ResolvedFragment>,
    operations: Vec<ResolvedOperation>,
    selection_parent_idx: BTreeMap<SelectionId, SelectionParent>,
    selections: Vec<Selection>,
    variables: Vec<ResolvedVariable>,
}

impl Query {
    fn push_selection(&mut self, node: Selection, parent: SelectionParent) -> SelectionId {
        let id = SelectionId(self.selections.len() as u32);
        self.selections.push(node);

        self.selection_parent_idx.insert(id, parent);

        id
    }

    pub fn operations(&self) -> impl Iterator<Item = (OperationId, &ResolvedOperation)> {
        walk_operations(self)
    }

    pub(crate) fn get_selection(&self, id: SelectionId) -> &Selection {
        self.selections
            .get(id.0 as usize)
            .expect("Query.get_selection")
    }

    pub(crate) fn get_fragment(&self, id: ResolvedFragmentId) -> &ResolvedFragment {
        self.fragments
            .get(id.0 as usize)
            .expect("Query.get_fragment")
    }

    pub(crate) fn get_operation(&self, id: OperationId) -> &ResolvedOperation {
        self.operations
            .get(id.0 as usize)
            .expect("Query.get_operation")
    }

    /// Selects the first operation matching `struct_name`. Returns `None` when the query document defines no operation, or when the selected operation does not match any defined operation.
    pub(crate) fn select_operation<'a>(
        &'a self,
        name: &str,
        normalization: Normalization,
    ) -> Option<(OperationId, &'a ResolvedOperation)> {
        walk_operations(self).find(|(_id, op)| normalization.operation(&op.name) == name)
    }

    fn find_fragment(&mut self, name: &str) -> Option<(ResolvedFragmentId, &mut ResolvedFragment)> {
        self.fragments
            .iter_mut()
            .enumerate()
            .find(|(_, frag)| frag.name == name)
            .map(|(id, f)| (ResolvedFragmentId(id as u32), f))
    }

    fn find_operation(&mut self, name: &str) -> Option<(OperationId, &mut ResolvedOperation)> {
        self.operations
            .iter_mut()
            .enumerate()
            .find(|(_, op)| op.name == name)
            .map(|(id, op)| (OperationId::new(id), op))
    }

    fn selections(&self) -> impl Iterator<Item = (SelectionId, &Selection)> {
        self.selections
            .iter()
            .enumerate()
            .map(|(idx, selection)| (SelectionId(idx as u32), selection))
    }

    fn walk_selection_set<'a>(
        &'a self,
        selection_ids: &'a [SelectionId],
    ) -> impl Iterator<Item = (SelectionId, &'a Selection)> + 'a {
        selection_ids
            .iter()
            .map(move |id| (*id, self.get_selection(*id)))
    }
}

#[derive(Debug)]
pub(crate) struct ResolvedVariable {
    pub(crate) operation_id: OperationId,
    pub(crate) name: String,
    pub(crate) default: Option<graphql_parser::query::Value<'static, String>>,
    pub(crate) r#type: StoredFieldType,
}

impl ResolvedVariable {
    pub(crate) fn type_name<'schema>(&self, schema: &'schema Schema) -> &'schema str {
        self.r#type.id.name(schema)
    }

    fn collect_used_types(&self, used_types: &mut UsedTypes, schema: &Schema) {
        match self.r#type.id {
            TypeId::Input(input_id) => {
                used_types.types.insert(TypeId::Input(input_id));

                let input = schema.get_input(input_id);

                input.used_input_ids_recursive(used_types, schema)
            }
            type_id @ TypeId::Scalar(_) | type_id @ TypeId::Enum(_) => {
                used_types.types.insert(type_id);
            }
            _ => (),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct UsedTypes {
    pub(crate) types: BTreeSet<TypeId>,
    fragments: BTreeSet<ResolvedFragmentId>,
}

impl UsedTypes {
    pub(crate) fn inputs<'s, 'a: 's>(
        &'s self,
        schema: &'a Schema,
    ) -> impl Iterator<Item = (InputId, &'a StoredInputType)> + 's {
        schema
            .inputs()
            .filter(move |(id, _input)| self.types.contains(&TypeId::Input(*id)))
    }

    pub(crate) fn scalars<'s, 'a: 's>(
        &'s self,
        schema: &'a Schema,
    ) -> impl Iterator<Item = (ScalarId, &'a StoredScalar)> + 's {
        self.types
            .iter()
            .filter_map(TypeId::as_scalar_id)
            .map(move |scalar_id| (scalar_id, schema.get_scalar(scalar_id)))
            .filter(|(_id, scalar)| !crate::schema::DEFAULT_SCALARS.contains(&scalar.name.as_str()))
    }

    pub(crate) fn enums<'a, 'schema: 'a>(
        &'a self,
        schema: &'schema Schema,
    ) -> impl Iterator<Item = (EnumId, &'schema StoredEnum)> + 'a {
        self.types
            .iter()
            .filter_map(TypeId::as_enum_id)
            .map(move |enum_id| (enum_id, schema.get_enum(enum_id)))
    }

    pub(crate) fn fragment_ids(&self) -> impl Iterator<Item = ResolvedFragmentId> + '_ {
        self.fragments.iter().copied()
    }
}

fn resolve_variables<'doc, T>(
    query: &mut Query,
    variables: &[graphql_parser::query::VariableDefinition<'doc, T>],
    schema: &Schema,
    operation_id: OperationId,
) where
    T: graphql_parser::query::Text<'doc>,
{
    for var in variables {
        query.variables.push(ResolvedVariable {
            operation_id,
            name: var.name.as_ref().into(),
            default: var.default_value.as_ref().map(|dflt| dflt.into_static()),
            r#type: resolve_field_type(schema, &var.var_type),
        });
    }
}

pub(crate) fn walk_operations(
    query: &Query,
) -> impl Iterator<Item = (OperationId, &ResolvedOperation)> {
    query
        .operations
        .iter()
        .enumerate()
        .map(|(id, op)| (OperationId(id as u32), op))
}

pub(crate) fn operation_has_no_variables(operation_id: OperationId, query: &Query) -> bool {
    walk_operation_variables(operation_id, query)
        .next()
        .is_none()
}

pub(crate) fn walk_operation_variables(
    operation_id: OperationId,
    query: &Query,
) -> impl Iterator<Item = (VariableId, &ResolvedVariable)> {
    query
        .variables
        .iter()
        .enumerate()
        .map(|(idx, var)| (VariableId(idx as u32), var))
        .filter(move |(_id, var)| var.operation_id == operation_id)
}

pub(crate) fn all_used_types(operation_id: OperationId, query: &BoundQuery<'_>) -> UsedTypes {
    let mut used_types = UsedTypes::default();

    let operation = query.query.get_operation(operation_id);

    for (_id, selection) in query.query.walk_selection_set(&operation.selection_set) {
        selection.collect_used_types(&mut used_types, query);
    }

    for (_id, variable) in walk_operation_variables(operation_id, query.query) {
        variable.collect_used_types(&mut used_types, query.schema);
    }

    used_types
}

pub(crate) fn full_path_prefix(selection_id: SelectionId, query: &BoundQuery<'_>) -> String {
    let mut path = match query.query.get_selection(selection_id) {
        Selection::FragmentSpread(_) | Selection::InlineFragment(_) => Vec::new(),
        selection => vec![selection.to_path_segment(query)],
    };

    let mut item = selection_id;

    while let Some(parent) = query.query.selection_parent_idx.get(&item) {
        path.push(parent.to_path_segment(query));

        match parent {
            SelectionParent::Field(id) | SelectionParent::InlineFragment(id) => {
                item = *id;
            }
            _ => break,
        }
    }

    path.reverse();
    path.join("")
}

#[derive(Clone, Copy)]
pub(crate) struct BoundQuery<'a> {
    pub(crate) query: &'a Query,
    pub(crate) schema: &'a Schema,
}
