//! The responsibility of this module is to resolve and validate a query
//! against a given schema.

use crate::schema::InputRef;
use crate::{
    constants::TYPENAME_FIELD,
    field_type::GraphqlTypeQualifier,
    schema::{
        resolve_field_type, EnumRef, FieldRef, ObjectId, ScalarRef, Schema, StoredFieldId,
        StoredFieldType, TypeId, TypeRef,
    },
};
use heck::CamelCase;
use petgraph::prelude::EdgeRef;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub(crate) struct SelectionItem<'a> {
    parent_id: Option<NodeId>,
    node_id: NodeId,
    variant: SelectionVariant<'a>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum SelectionVariant<'a> {
    SelectedField {
        alias: Option<&'a str>,
        field: FieldRef<'a>,
    },
    FragmentSpread(ResolvedFragmentId),
    InlineFragment(TypeRef<'a>),
    Typename,
}

impl<'a> WithQuery<'a, SelectionItem<'a>> {
    pub(crate) fn variant(&self) -> &SelectionVariant<'a> {
        &self.item.variant
    }

    pub(crate) fn parent(&self) -> Option<WithQuery<'a, SelectionItem<'a>>> {
        self.item
            .parent_id
            .map(|parent_id| self.refocus(parent_id).upgrade())
    }

    pub(crate) fn full_path_prefix(&self, root_name: &str) -> String {
        let mut path = vec![self.to_path_segment()];

        let mut item = *self;

        while let Some(parent) = item.parent() {
            item = parent;
            path.push(parent.to_path_segment());
        }

        path.push(root_name.to_owned());
        path.reverse();
        path.join("")
    }

    fn to_path_segment(&self) -> String {
        match self.item.variant {
            SelectionVariant::SelectedField { alias, field } => alias
                .map(|alias| alias.to_camel_case())
                .unwrap_or_else(|| field.name().to_camel_case()),
            SelectionVariant::InlineFragment(type_ref) => {
                format!("On{}", type_ref.name().to_camel_case())
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn subselection<'b>(
        &'b self,
    ) -> impl Iterator<Item = WithQuery<'a, SelectionItem<'a>>> + 'b {
        let id_selection = self.refocus(self.item.node_id);

        id_selection.into_subselection().map(move |s| s.upgrade())
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WithQuery<'a, T> {
    query: &'a ResolvedQuery,
    schema: &'a Schema,
    item: T,
}

impl<'a, T> WithQuery<'a, T> {
    pub(crate) fn refocus<U>(&self, new_item: U) -> WithQuery<'a, U> {
        WithQuery {
            query: self.query,
            schema: self.schema,
            item: new_item,
        }
    }
}

type NodeId = petgraph::prelude::NodeIndex<u32>;
type SelectionGraph = petgraph::Graph<QueryNode, QueryEdge, petgraph::Directed, u32>;

impl<'a> WithQuery<'a, NodeId> {
    fn get_node(&self) -> WithQuery<'a, &'a QueryNode> {
        let item = &self.query.selection_graph[self.item];
        self.refocus(item)
    }

    pub fn into_subselection(self) -> impl Iterator<Item = WithQuery<'a, NodeId>> {
        self.query
            .selection_graph
            .edges_directed(self.item, petgraph::Direction::Outgoing)
            .filter(|edge| match edge.weight() {
                QueryEdge::Selection => true,
            })
            .map(move |edge| self.refocus(edge.target()))
    }

    pub(crate) fn subselection<'b>(&'b self) -> impl Iterator<Item = WithQuery<'a, NodeId>> + 'b {
        self.into_subselection()
    }

    pub(crate) fn collect_used_types(&self, used_types: &mut UsedTypes) {
        let node = self.get_node();
        match node.item {
            QueryNode::SelectedField(field) => {
                let field_ref = self.schema.field(field.field_id);
                used_types.types.insert(field_ref.type_id());

                for item in self.subselection() {
                    item.collect_used_types(used_types);
                }
            }
            QueryNode::InlineFragment(type_id) => {
                used_types.types.insert(*type_id);

                for item in self.subselection() {
                    item.collect_used_types(used_types);
                }
            }
            QueryNode::FragmentSpread(fragment_id) => {
                used_types.fragments.insert(*fragment_id);

                for item in self.refocus(*fragment_id).selection_ids() {
                    item.collect_used_types(used_types);
                }
            }
            QueryNode::Typename => (),
        }
    }

    fn upgrade(&self) -> WithQuery<'a, SelectionItem<'a>> {
        let node = self.get_node();

        let variant = match node.item {
            QueryNode::FragmentSpread(frag_id) => SelectionVariant::FragmentSpread(*frag_id),
            QueryNode::InlineFragment(type_id) => {
                SelectionVariant::InlineFragment(type_id.upgrade(self.schema))
            }
            QueryNode::Typename => SelectionVariant::Typename,
            QueryNode::SelectedField(f) => SelectionVariant::SelectedField {
                alias: f.alias.as_ref().map(String::as_str),
                field: self.schema.field(f.field_id),
            },
        };

        self.refocus(SelectionItem {
            node_id: self.item,
            parent_id: self.parent_id(),
            variant,
        })
    }

    fn parent_id(&self) -> Option<NodeId> {
        self.query
            .selection_graph
            .edges_directed(self.item, petgraph::Direction::Incoming)
            .filter(|edge| match edge.weight() {
                QueryEdge::Selection => true,
            })
            .map(|edge| edge.source())
            .next()
    }
}

#[derive(Debug)]
enum QueryNode {
    SelectedField(SelectedField),
    InlineFragment(TypeId),
    FragmentSpread(ResolvedFragmentId),
    Typename,
}

#[derive(Debug)]
enum QueryEdge {
    Selection,
}

#[derive(Debug)]
struct SelectedField {
    alias: Option<String>,
    field_id: StoredFieldId,
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
    )?;

    let (_, mut fragment) = query
        .find_fragment(&fragment_definition.name)
        .expect("TODO: fragment resolution");

    fragment.selection = acc.into_vec();

    Ok(())
}

fn resolve_object_selection<'a>(
    query: &mut ResolvedQuery,
    object: impl crate::schema::ObjectRefLike<'a>,
    selection_set: &graphql_parser::query::SelectionSet,
    parent: Option<NodeId>,
    acc: &mut SelectionAccumulator,
) -> anyhow::Result<()> {
    for item in selection_set.items.iter() {
        match item {
            graphql_parser::query::Selection::Field(field) => {
                if field.name == TYPENAME_FIELD {
                    let id = query.push_node(QueryNode::Typename, parent);
                    acc.push(id);
                    continue;
                }

                let field_ref = object.get_field_by_name(&field.name).ok_or_else(|| {
                    anyhow::anyhow!("No field named {} on {}", &field.name, object.name())
                })?;

                let id = query.push_node(
                    QueryNode::SelectedField(SelectedField {
                        alias: field.alias.clone(),
                        field_id: field_ref.id(),
                    }),
                    parent,
                );

                resolve_selection(
                    query,
                    object.schema(),
                    field_ref.type_id(),
                    &field.selection_set,
                    Some(id),
                    &mut SelectionAccumulator::noop(),
                )?;

                acc.push(id)
            }
            graphql_parser::query::Selection::InlineFragment(inline) => {
                let selection_id = resolve_inline_fragment(query, object.schema(), inline, parent)?;

                acc.push(selection_id);
            }
            graphql_parser::query::Selection::FragmentSpread(fragment_spread) => {
                let (fragment_id, _) = query
                    .find_fragment(&fragment_spread.fragment_name)
                    .expect("TODO: fragment resolution");

                acc.push(query.push_node(
                    QueryNode::FragmentSpread(ResolvedFragmentId(fragment_id)),
                    parent,
                ));
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
    parent: Option<NodeId>,
    acc: &mut SelectionAccumulator,
) -> anyhow::Result<()> {
    let selection = match on {
        TypeId::Object(oid) => {
            let object = schema.object(oid);
            resolve_object_selection(ctx, object, selection_set, parent, acc)?;
        }
        TypeId::Interface(interface_id) => {
            let interface = schema.interface(interface_id);
            resolve_object_selection(ctx, interface, selection_set, parent, acc)?;
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
    parent: Option<NodeId>,
) -> anyhow::Result<NodeId> {
    let graphql_parser::query::TypeCondition::On(on) = inline_fragment
        .type_condition
        .as_ref()
        .expect("missing type condition on inline fragment");
    let type_id = schema
        .find_type(on)
        .ok_or_else(|| anyhow::anyhow!("TODO: error message"))?;

    let id = query.push_node(QueryNode::InlineFragment(type_id), parent);

    resolve_selection(
        query,
        schema,
        type_id,
        &inline_fragment.selection_set,
        Some(id),
        &mut SelectionAccumulator::noop(),
    )?;

    Ok(id)
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
pub(crate) struct ResolvedFragmentId(usize);

#[derive(Debug, Default)]
pub(crate) struct ResolvedQuery {
    pub(crate) operations: Vec<ResolvedOperation>,
    fragments: Vec<ResolvedFragment>,
    selection_graph: SelectionGraph,
}

impl ResolvedQuery {
    fn push_node(&mut self, node: QueryNode, parent: Option<NodeId>) -> NodeId {
        let id = self.selection_graph.add_node(node);

        if let Some(parent) = parent {
            self.selection_graph
                .add_edge(parent, id, QueryEdge::Selection);
        }

        id
    }

    fn find_fragment(&mut self, name: &str) -> Option<(usize, &mut ResolvedFragment)> {
        self.fragments
            .iter_mut()
            .enumerate()
            .find(|(_, frag)| frag.name == name)
    }
}

#[derive(Debug)]
pub(crate) struct ResolvedFragment {
    name: String,
    on: crate::schema::TypeId,
    selection: Vec<NodeId>,
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

    pub(crate) fn name(&self) -> &'a str {
        self.get().name()
    }

    pub(crate) fn rich_selection<'b>(
        &'b self,
    ) -> impl Iterator<Item = WithQuery<'a, SelectionItem<'a>>> + 'b {
        self.selection().map(|s| s.upgrade())
    }

    pub(crate) fn selection<'b>(&'b self) -> impl Iterator<Item = WithQuery<'a, NodeId>> + 'b {
        let operation = self.get();
        operation
            .selection
            .iter()
            .map(move |selection_id| WithQuery {
                item: *selection_id,
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

        for variable in self.variables() {
            variable.collect_used_types(&mut all_used_types);
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
    selection: Vec<NodeId>,
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

    fn collect_used_types(&self, used_types: &mut UsedTypes) {
        match self.get().r#type.id {
            type_id @ TypeId::Input(_)
            | type_id @ TypeId::Scalar(_)
            | type_id @ TypeId::Enum(_) => {
                used_types.types.insert(type_id);
            }
            _ => (),
        }
    }
}

impl<'a> WithQuery<'a, ResolvedFragmentId> {
    fn get(&self) -> &'a ResolvedFragment {
        self.query.fragments.get(self.item.0).unwrap()
    }

    fn selection_ids<'b>(&'b self) -> impl Iterator<Item = WithQuery<'a, NodeId>> + 'b {
        let fragment = self.get();
        fragment
            .selection
            .iter()
            .map(move |item| self.refocus(*item))
    }

    pub(crate) fn selection<'b>(
        &'b self,
    ) -> impl Iterator<Item = WithQuery<'a, SelectionItem<'a>>> + 'b {
        self.selection_ids().map(|sel| sel.upgrade())
    }

    pub(crate) fn name(&self) -> &'a str {
        &self.get().name
    }

    pub(crate) fn selection_len(&self) -> usize {
        self.get().selection.len()
    }
}

#[derive(Debug, Default)]
pub(crate) struct UsedTypes {
    types: HashSet<TypeId>,
    fragments: HashSet<ResolvedFragmentId>,
}

impl UsedTypes {
    pub(crate) fn inputs<'s, 'a: 's>(
        &'s self,
        schema: &'a Schema,
    ) -> impl Iterator<Item = InputRef<'a>> + 's {
        schema
            .inputs()
            .filter(move |input_ref| self.types.contains(&input_ref.type_id()))
    }

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

struct SelectionAccumulator(Option<Vec<NodeId>>);

impl SelectionAccumulator {
    fn with_capacity(cap: usize) -> Self {
        SelectionAccumulator(Some(Vec::with_capacity(cap)))
    }

    fn noop() -> Self {
        SelectionAccumulator(None)
    }

    fn push(&mut self, item: NodeId) {
        if let Some(v) = &mut self.0 {
            v.push(item);
        }
    }

    fn into_vec(self) -> Vec<NodeId> {
        self.0.unwrap_or_else(Vec::new)
    }
}
