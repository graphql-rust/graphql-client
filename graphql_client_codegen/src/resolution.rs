//! The responsibility of this module is to resolve and validate a query
//! against a given schema.

use crate::{
    constants::TYPENAME_FIELD,
    field_type::GraphqlTypeQualifier,
    schema::{
        resolve_field_type, EnumRef, FieldRef, InterfaceRef, ObjectId, ObjectRef, ScalarRef,
        Schema, StoredFieldId, StoredFieldType, TypeId, TypeRef, UnionRef,
    },
};
use std::collections::HashSet;

// enum QueryNode {
//     Field(StoredFieldId),
//     InlineFragment(TypeId),
//     FragmentSpread(FragmentId),
// }

// enum QueryEdge {
//     Selection,
// }

#[derive(Debug, Clone, Copy)]
enum SelectionId {
    FieldId(usize),
    InlineFragmentId(usize),
    FragmentSpread(usize),
    Typename(Option<SelectionParentId>),
}

#[derive(Debug, Clone, Copy)]
enum SelectionParentId {
    FieldId(usize),
    InlineFragmentId(usize),
}

#[derive(Debug)]
struct Field {
    parent: Option<SelectionParentId>,
    alias: Option<String>,
    field_id: StoredFieldId,
}

#[derive(Debug)]
struct FragmentSpread {
    parent: Option<SelectionParentId>,
    fragment_id: ResolvedFragmentId,
}

#[derive(Debug)]
struct InlineFragment {
    parent: Option<SelectionParentId>,
    on: TypeId,
}

pub(crate) fn resolve(
    schema: &Schema,
    query: &graphql_parser::query::Document,
) -> anyhow::Result<ResolvedQuery> {
    let mut resolved_query: ResolvedQuery = Default::default();

    // First, give ids to all fragments.
    for definition in &query.definitions {
        match definition {
            graphql_parser::query::Definition::Fragment(fragment) => {
                let graphql_parser::query::TypeCondition::On(on) = &fragment.type_condition;
                resolved_query.fragments.push(ResolvedFragment {
                    name: fragment.name.clone(),
                    on: schema.find_type(on).expect("TODO: proper error message"),
                    selection: Vec::new(),
                });
            }
            _ => (),
        }
    }

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

    Ok(resolved_query)
}

fn resolve_fragment(
    query: &mut ResolvedQuery,
    schema: &Schema,
    fragment_definition: &graphql_parser::query::FragmentDefinition,
) -> anyhow::Result<()> {
    let graphql_parser::query::TypeCondition::On(on) = &fragment_definition.type_condition;
    let on = schema.find_type(&on).unwrap();

    let mut acc =
        SelectionAccumulator::with_capacity(fragment_definition.selection_set.items.len());

    resolve_selection(
        query,
        schema,
        on,
        &fragment_definition.selection_set,
        None,
        &mut acc,
    );

    let (_, mut fragment) = query
        .find_fragment(&fragment_definition.name)
        .expect("TODO: fragment resolution");

    fragment.selection = acc.into_vec();

    Ok(())
}

fn resolve_object_selection(
    query: &mut ResolvedQuery,
    object: ObjectRef<'_>,
    selection_set: &graphql_parser::query::SelectionSet,
    parent: Option<SelectionParentId>,
    acc: &mut SelectionAccumulator,
) -> anyhow::Result<()> {
    for item in selection_set.items.iter() {
        match item {
            graphql_parser::query::Selection::Field(field) => {
                if field.name == TYPENAME_FIELD {
                    acc.push(SelectionId::Typename(parent));
                    continue;
                }

                let field_ref = object.get_field_by_name(&field.name).ok_or_else(|| {
                    anyhow::anyhow!("No field named {} on {}", &field.name, object.name())
                })?;

                let id = query.selected_fields.len();
                query.selected_fields.push(Field {
                    parent,
                    alias: field.alias.clone(),
                    field_id: field_ref.id(),
                });

                resolve_selection(
                    query,
                    object.schema(),
                    field_ref.type_id(),
                    &field.selection_set,
                    parent,
                    &mut SelectionAccumulator::noop(),
                );

                acc.push(SelectionId::FieldId(id))
            }
            graphql_parser::query::Selection::InlineFragment(inline) => {
                let selection_id = resolve_inline_fragment(query, object.schema(), inline, parent)?;
            }
            graphql_parser::query::Selection::FragmentSpread(fragment_spread) => {
                let (fragment_id, _) = query
                    .find_fragment(&fragment_spread.fragment_name)
                    .expect("TODO: fragment resolution");
                let id = query.fragment_spreads.len();
                query.fragment_spreads.push(FragmentSpread {
                    fragment_id: ResolvedFragmentId(fragment_id),
                    parent,
                });

                acc.push(SelectionId::FragmentSpread(id))
            }
        }
    }

    Ok(())
}

fn resolve_selection(
    ctx: &mut ResolvedQuery,
    schema: &Schema,
    on: TypeId,
    selection_set: &graphql_parser::query::SelectionSet,
    parent: Option<SelectionParentId>,
    acc: &mut SelectionAccumulator,
) -> anyhow::Result<()> {
    let selection = match on {
        TypeId::Object(oid) => {
            let object = schema.object(oid);
            resolve_object_selection(ctx, object, selection_set, parent, acc)?;
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
        }
    };

    Ok(())
}

fn resolve_inline_fragment(
    query: &mut ResolvedQuery,
    schema: &Schema,
    inline_fragment: &graphql_parser::query::InlineFragment,
    parent: Option<SelectionParentId>,
) -> anyhow::Result<SelectionId> {
    let graphql_parser::query::TypeCondition::On(on) = inline_fragment
        .type_condition
        .as_ref()
        .expect("missing type condition on inline fragment");
    let type_id = schema
        .find_type(on)
        .ok_or_else(|| anyhow::anyhow!("TODO: error message"))?;

    let id = query.inline_fragments.len();
    query.inline_fragments.push(InlineFragment {
        parent,
        on: type_id,
    });

    resolve_selection(
        query,
        schema,
        type_id,
        &inline_fragment.selection_set,
        Some(SelectionParentId::InlineFragmentId(id)),
        &mut SelectionAccumulator::noop(),
    );

    Ok(SelectionId::InlineFragmentId(id))
}

fn resolve_operation(
    query: &mut ResolvedQuery,
    schema: &Schema,
    operation: &graphql_parser::query::OperationDefinition,
) -> anyhow::Result<()> {
    match operation {
        graphql_parser::query::OperationDefinition::Mutation(m) => {
            let on = schema.mutation_type();
            let mut acc = SelectionAccumulator::with_capacity(m.selection_set.items.len());
            resolve_object_selection(query, on, &m.selection_set, None, &mut acc)?;
            let resolved_operation: ResolvedOperation = ResolvedOperation {
                object_id: on.id(),
                name: m.name.as_ref().expect("mutation without name").to_owned(),
                operation_type: crate::operations::OperationType::Mutation,
                variables: resolve_variables(
                    &m.variable_definitions,
                    schema,
                    query.operations.len(),
                )?,
                selection: acc.into_vec(),
            };

            query.operations.push(resolved_operation);
        }
        graphql_parser::query::OperationDefinition::Query(q) => {
            let on = schema.query_type();
            let mut acc = SelectionAccumulator::with_capacity(q.selection_set.items.len());
            resolve_object_selection(query, on, &q.selection_set, None, &mut acc)?;

            let resolved_operation: ResolvedOperation = ResolvedOperation {
                name: q.name.as_ref().expect("query without name").to_owned(),
                operation_type: crate::operations::OperationType::Query,
                variables: resolve_variables(
                    &q.variable_definitions,
                    schema,
                    query.operations.len(),
                )?,
                object_id: on.id(),
                selection: acc.into_vec(),
            };

            query.operations.push(resolved_operation);
        }
        graphql_parser::query::OperationDefinition::Subscription(s) => {
            let on = schema.subscription_type();
            let mut acc = SelectionAccumulator::with_capacity(s.selection_set.items.len());
            resolve_object_selection(query, on, &s.selection_set, None, &mut acc)?;

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
                selection: acc.into_vec(),
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
    selected_fields: Vec<Field>,
    inline_fragments: Vec<InlineFragment>,
    fragment_spreads: Vec<FragmentSpread>,
}

impl ResolvedQuery {
    fn find_fragment(&mut self, name: &str) -> Option<(usize, &mut ResolvedFragment)> {
        self.fragments
            .iter_mut()
            .enumerate()
            .find(|(_, frag)| frag.name == name)
    }
}

#[derive(Debug)]
struct ResolvedFragment {
    name: String,
    on: crate::schema::TypeId,
    selection: Vec<SelectionId>,
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

    pub(crate) fn selection(&self) -> impl Iterator<Item = SelectionRef<'_>> {
        let operation = self.get();
        operation
            .selection
            .iter()
            .map(move |selection_id| SelectionRef {
                selection_id: *selection_id,
                query: self.query,
                schema: self.schema,
            })
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
pub(crate) struct SelectionRef<'a> {
    query: &'a ResolvedQuery,
    schema: &'a Schema,
    selection_id: SelectionId,
}

impl<'a> SelectionRef<'a> {
    fn collect_used_types(&self, used_types: &mut UsedTypes) {
        match self.refine() {
            SelectionItem::Field(selected_field_ref) => {
                used_types
                    .types
                    .insert(selected_field_ref.field().type_id());

                for item in selected_field_ref.subselection() {
                    item.collect_used_types(used_types)
                }
            }
            SelectionItem::InlineFragment(inline_fragment_ref) => {
                todo!();
            }
            SelectionItem::FragmentSpread(fragment_spread_ref) => fragment_spread_ref
                .fragment()
                .collect_used_types(used_types),
            SelectionItem::Typename => (),
        }
    }

    pub(crate) fn refine(&self) -> SelectionItem<'a> {
        match self.selection_id {
            SelectionId::FieldId(field_id) => SelectionItem::Field(SelectedFieldRef {
                query: self.query,
                schema: self.schema,
                field_id,
            }),
            SelectionId::InlineFragmentId(inline_fragment_id) => {
                SelectionItem::InlineFragment(InlineFragmentRef {
                    query: self.query,
                    schema: self.schema,
                    inline_fragment_id,
                })
            }
            SelectionId::FragmentSpread(fragment_spread_id) => {
                SelectionItem::FragmentSpread(FragmentSpreadRef {
                    query: self.query,
                    schema: self.schema,
                    fragment_spread_id,
                })
            }
            SelectionId::Typename(_) => todo!(),
        }
    }
}

pub(crate) enum SelectionItem<'a> {
    Field(SelectedFieldRef<'a>),
    InlineFragment(InlineFragmentRef<'a>),
    FragmentSpread(FragmentSpreadRef<'a>),
    Typename,
}

pub(crate) struct SelectedFieldRef<'a> {
    query: &'a ResolvedQuery,
    schema: &'a Schema,
    field_id: usize,
}

pub(crate) struct FragmentSpreadRef<'a> {
    query: &'a ResolvedQuery,
    schema: &'a Schema,
    fragment_spread_id: usize,
}

impl<'a> FragmentSpreadRef<'a> {
    fn get(&self) -> &'a FragmentSpread {
        self.query
            .fragment_spreads
            .get(self.fragment_spread_id)
            .unwrap()
    }

    fn fragment(&self) -> Fragment<'a> {
        Fragment {
            query: self.query,
            schema: self.schema,
            fragment_id: self.get().fragment_id,
        }
    }
}

impl<'a> SelectedFieldRef<'a> {
    fn get(&self) -> &'a Field {
        self.query.selected_fields.get(self.field_id).unwrap()
    }

    pub(crate) fn field(&self) -> crate::schema::FieldRef<'_> {
        self.schema.field(self.get().field_id)
    }

    pub(crate) fn alias(&self) -> Option<&'a str> {
        self.get().alias.as_ref().map(String::as_str)
    }

    pub(crate) fn subselection(&self) -> impl Iterator<Item = SelectionRef<'a>> {
        std::iter::empty()
    }
}

pub(crate) struct InlineFragmentRef<'a> {
    query: &'a ResolvedQuery,
    schema: &'a Schema,
    inline_fragment_id: usize,
}

impl<'a> InlineFragmentRef<'a> {
    fn get(&self) -> &'a InlineFragment {
        self.query
            .inline_fragments
            .get(self.inline_fragment_id)
            .unwrap()
    }
}

#[derive(Debug)]
pub(crate) struct ResolvedOperation {
    name: String,
    operation_type: crate::operations::OperationType,
    variables: Vec<ResolvedVariable>,
    selection: Vec<SelectionId>,
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

    pub(crate) fn type_name(&self) -> &'a str {
        self.get().r#type.id.upgrade(self.operation.schema()).name()
    }

    pub(crate) fn type_qualifiers(&self) -> &[GraphqlTypeQualifier] {
        &self.get().r#type.qualifiers
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Fragment<'a> {
    query: &'a ResolvedQuery,
    schema: &'a Schema,
    fragment_id: ResolvedFragmentId,
}

impl Fragment<'_> {
    fn get(&self) -> &ResolvedFragment {
        self.query.fragments.get(self.fragment_id.0).unwrap()
    }

    fn collect_used_types(&self, used_types: &mut UsedTypes) {
        todo!()
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

struct SelectionAccumulator(Option<Vec<SelectionId>>);

impl SelectionAccumulator {
    fn with_capacity(cap: usize) -> Self {
        SelectionAccumulator(Some(Vec::with_capacity(cap)))
    }

    fn noop() -> Self {
        SelectionAccumulator(None)
    }

    fn push(&mut self, item: SelectionId) {
        if let Some(v) = &mut self.0 {
            v.push(item);
        }
    }

    fn into_vec(self) -> Vec<SelectionId> {
        self.0.unwrap_or_else(Vec::new)
    }
}
