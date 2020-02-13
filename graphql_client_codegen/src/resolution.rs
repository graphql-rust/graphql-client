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
        selection: resolve_selection(query, schema, on, &fragment.selection_set)?,
    };

    query.fragments.push(resolved_fragment);

    Ok(())
}

fn resolve_object_selection(
    query: &mut ResolvedQuery,
    object: ObjectRef<'_>,
    selection_set: &graphql_parser::query::SelectionSet,
) -> anyhow::Result<Vec<usize>> {
    let id_selection: Vec<usize> = selection_set
        .items
        .iter()
        .map(|item| -> anyhow::Result<_> {
            match item {
                graphql_parser::query::Selection::Field(field) => {
                    if field.name == TYPENAME_FIELD {
                        return Ok(IdSelectionItemContents::Typename);
                    }

                    let field_ref = object.get_field_by_name(&field.name).ok_or_else(|| {
                        anyhow::anyhow!("No field named {} on {}", &field.name, object.name())
                    })?;
                    Ok(IdSelectionItemContents::Field {
                        field_id: field_ref.id(),
                        alias: field.alias.clone(),
                        selection: resolve_selection(
                            query,
                            object.schema(),
                            field_ref.type_id(),
                            &field.selection_set,
                        )?,
                    })
                }
                graphql_parser::query::Selection::InlineFragment(inline) => {
                    resolve_inline_fragment(query, object.schema(), inline)
                }
                graphql_parser::query::Selection::FragmentSpread(fragment_spread) => Ok(
                    IdSelectionItemContents::FragmentSpread(fragment_spread.fragment_name.clone()),
                ),
            }
        })
        .collect::<Result<_, _>>()?;

    Ok(IdSelectionSet {
        selection_set: id_selection,
        parent,
    })
}

fn resolve_selection(
    ctx: &mut ResolvedQuery,
    schema: &Schema,
    on: TypeId,
    selection_set: &graphql_parser::query::SelectionSet,
) -> anyhow::Result<usize> {
    let selection = match on {
        TypeId::Object(oid) => {
            let object = schema.object(oid);
            resolve_object_selection(ctx, object, selection_set)?
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
            IdSelection {
                on,
                selection_set: Vec::new(),
            }
        }
    };

    let selection_id = ctx.selections.len();
    ctx.selections.push(selection);

    Ok(selection_id)
}

fn resolve_inline_fragment(
    query: &mut ResolvedQuery,
    schema: &Schema,
    inline_fragment: &graphql_parser::query::InlineFragment,
) -> anyhow::Result<IdSelectionItem> {
    let graphql_parser::query::TypeCondition::On(on) = inline_fragment
        .type_condition
        .as_ref()
        .expect("missing type condition");
    let type_id = schema
        .find_type(on)
        .ok_or_else(|| anyhow::anyhow!("TODO: error message"))?;
    Ok(IdSelectionItemContents::InlineFragment(
        type_id,
        resolve_selection(&mut query, schema, type_id, &inline_fragment.selection_set)?,
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
                selection: resolve_object_selection(query, on, &m.selection_set)?,
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
                selection: resolve_object_selection(query, on, &q.selection_set)?,
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
                selection: resolve_object_selection(query, on, &s.selection_set)?,
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
    selections: Vec<IdSelectionSet>,
    selection_items: Vec<IdSelectionItem>,
}

#[derive(Debug)]
struct ResolvedFragment {
    name: String,
    on: crate::schema::TypeId,
    selection: usize,
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

    pub(crate) fn selection(&self) -> impl Iterator<Item = SelectionSetRef<'_>> {
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
    selection: usize,
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
struct IdSelectionSet {
    on: TypeId,
    /// A vec of IdSelectionItem ids.
    selection_set: Vec<usize>,
}

#[derive(Debug, Clone)]
enum IdSelectionItem {
    Typename,
    Field {
        field_id: StoredFieldId,
        alias: Option<String>,
        /// The id of a selection set.
        selection: usize,
    },
    /// The id of a fragment
    FragmentSpread(usize),
    InlineFragment(TypeId, IdSelectionSet),
}

impl IdSelectionItem {
    fn upgrade<'a>(
        &self,
        schema: &'a Schema,
        query: &'a ResolvedQuery,
        parent: Option<(SelectionSetRef<'a>, usize)>,
    ) -> SelectionSetRef<'a> {
        let selection_set = match self {
            IdSelectionItemContents::Typename => SelectionItemContents::Typename,
            IdSelectionItemContents::Field {
                field_id: id,
                alias,
                selection,
            } => {
                let field = schema.field(*id);
                SelectionItemContents::Field {
                    field: field.clone(),
                    alias: alias.to_owned(),
                    selection: selection
                        .selection_set
                        .iter()
                        .map(move |selection| {
                            selection.upgrade(
                                schema,
                                query,
                                Some(SelectionOn::Field(field.clone())),
                            )
                        })
                        .collect(),
                }
            }
            IdSelectionItemContents::FragmentSpread(name) => {
                SelectionItemContents::FragmentSpread(Fragment {
                    fragment_id: ResolvedFragmentId(
                        query
                            .fragments
                            .iter()
                            .position(|frag| frag.name.as_str() == name.as_str())
                            .expect("fragment not found"),
                    ),
                    query,
                    schema,
                })
            }
            IdSelectionItemContents::InlineFragment(typeid, selection) => {
                let on = typeid.upgrade(schema);
                SelectionItemContents::InlineFragment(
                    on,
                    selection
                        .selection_set
                        .iter()
                        .map(|sel| {
                            sel.upgrade(schema, query, Some(SelectionOn::InlineFragment(parent)))
                        })
                        .collect(),
                )
            }
        };

        SelectionSetRef {
            query,
            selection_set,
            parent,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SelectionSetRef<'a> {
    query: &'a ResolvedQuery,
    /// Selection set id and selection item id.
    parent: Option<(usize, usize)>,
    selection_set_id: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct SelectionItem<'a> {
    query: &'a ResolvedQuery,
    selection: Option<SelectionId>,
    contents: SelectionItemContents<'a>,
}

#[derive(Debug, Clone)]
pub(crate) enum SelectionItemContents<'a> {
    Typename,
    Field {
        field: FieldRef<'a>,
        selection: Vec<SelectionSetRef<'a>>,
        alias: Option<String>,
    },
    FragmentSpread(Fragment<'a>),
    InlineFragment(TypeRef<'a>, Vec<SelectionSetRef<'a>>),
}

impl SelectionSetRef<'_> {
    fn collect_used_types(&self, used_types: &mut UsedTypes) {
        for item in &self.selection_set {
            match item {
                SelectionItemContents::Typename => (),
                SelectionItemContents::Field {
                    field,
                    selection,
                    alias: _,
                } => {
                    used_types.types.insert(field.type_id());

                    selection
                        .iter()
                        .for_each(|selection| selection.collect_used_types(used_types));
                }
                SelectionItemContents::FragmentSpread(fragment) => {
                    used_types.fragments.insert(fragment.fragment_id);
                    fragment
                        .selection()
                        .for_each(|selection| selection.collect_used_types(used_types))
                }
                SelectionItemContents::InlineFragment(on, selection) => {
                    used_types.types.insert(on.type_id());

                    selection
                        .iter()
                        .for_each(|selection| selection.collect_used_types(used_types))
                }
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

    pub(crate) fn selection(&self) -> SelectionSetRef<'_> {
        let selection_id = self.get().selection;

        SelectionSetRef {
            parent: None,
            query: self.query,
            selection_set: selection_id,
        }
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
