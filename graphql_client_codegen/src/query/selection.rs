use super::{
    BoundQuery, OperationId, Query, QueryValidationError, ResolvedFragmentId, SelectionId,
    UsedTypes,
};
use crate::schema::{Schema, StoredField, StoredFieldId, TypeId};
use heck::CamelCase;

/// This checks that the `on` clause on fragment spreads and inline fragments
/// are valid in their context.
pub(super) fn validate_type_conditions<'a, T>(
    selection_id: SelectionId,
    query: &BoundQuery<'a, '_, '_, T>,
) -> Result<(), QueryValidationError>
where
    T: graphql_parser::query::Text<'a> + std::default::Default,
{
    let selection = query.query.get_selection(selection_id);

    let selected_type = match selection {
        Selection::FragmentSpread(fragment_id) => query.query.get_fragment(*fragment_id).on,
        Selection::InlineFragment(inline_fragment) => inline_fragment.type_id,
        _ => return Ok(()),
    };

    let parent_schema_type_id = query
        .query
        .selection_parent_idx
        .get(&selection_id)
        .expect("Could not find selection parent")
        .schema_type_id(query);

    if parent_schema_type_id == selected_type {
        return Ok(());
    }

    match parent_schema_type_id {
        TypeId::Union(union_id) => {
            let union = query.schema.get_union(union_id);

            if !union
                .variants
                .iter()
                .any(|variant| *variant == selected_type)
            {
                return Err(QueryValidationError::new(format!(
                    "The spread {}... on {} is not valid.",
                    union.name,
                    selected_type.name(query.schema)
                )));
            }
        }
        TypeId::Interface(interface_id) => {
            let mut variants = query
                .schema
                .objects()
                .filter(|(_, obj)| obj.implements_interfaces.contains(&interface_id));

            if !variants.any(|(id, _)| TypeId::Object(id) == selected_type) {
                return Err(QueryValidationError::new(format!(
                    "The spread {}... on {} is not valid.",
                    parent_schema_type_id.name(query.schema),
                    selected_type.name(query.schema),
                )));
            }
        }
        _ => (),
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub(super) enum SelectionParent {
    Field(SelectionId),
    InlineFragment(SelectionId),
    Fragment(ResolvedFragmentId),
    Operation(OperationId),
}

#[allow(clippy::trivially_copy_pass_by_ref)]
impl SelectionParent {
    fn schema_type_id<'a, T>(&self, query: &BoundQuery<'a, '_, '_, T>) -> TypeId
    where
        T: graphql_parser::query::Text<'a> + std::default::Default,
    {
        match self {
            SelectionParent::Fragment(fragment_id) => query.query.get_fragment(*fragment_id).on,
            SelectionParent::Operation(operation_id) => {
                TypeId::Object(query.query.get_operation(*operation_id).object_id)
            }
            SelectionParent::Field(id) => {
                let field_id = query
                    .query
                    .get_selection(*id)
                    .as_selected_field()
                    .unwrap()
                    .field_id;
                query.schema.get_field(field_id).r#type.id
            }
            SelectionParent::InlineFragment(id) => {
                { query.query.get_selection(*id).as_inline_fragment().unwrap() }.type_id
            }
        }
    }

    pub(super) fn add_to_selection_set<'a, T>(
        &self,
        q: &mut Query<'a, T>,
        selection_id: SelectionId,
    ) where
        T: graphql_parser::query::Text<'a> + std::default::Default,
    {
        match self {
            SelectionParent::Field(parent_selection_id)
            | SelectionParent::InlineFragment(parent_selection_id) => {
                let parent_selection = q
                    .selections
                    .get_mut(parent_selection_id.0 as usize)
                    .expect("get parent selection");

                match parent_selection {
                    Selection::Field(f) => f.selection_set.push(selection_id),
                    Selection::InlineFragment(inline) => inline.selection_set.push(selection_id),
                    other => unreachable!("impossible parent selection: {:?}", other),
                }
            }
            SelectionParent::Fragment(fragment_id) => {
                let fragment = q
                    .fragments
                    .get_mut(fragment_id.0 as usize)
                    .expect("get fragment");

                fragment.selection_set.push(selection_id);
            }
            SelectionParent::Operation(operation_id) => {
                let operation = q
                    .operations
                    .get_mut(operation_id.0 as usize)
                    .expect("get operation");

                operation.selection_set.push(selection_id);
            }
        }
    }

    pub(crate) fn to_path_segment<'a, T>(self, query: &BoundQuery<'a, '_, '_, T>) -> String
    where
        T: graphql_parser::query::Text<'a> + std::default::Default,
    {
        match self {
            SelectionParent::Field(id) | SelectionParent::InlineFragment(id) => {
                query.query.get_selection(id).to_path_segment(query)
            }
            SelectionParent::Operation(id) => query.query.get_operation(id).to_path_segment(),
            SelectionParent::Fragment(id) => query.query.get_fragment(id).to_path_segment(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Selection {
    Field(SelectedField),
    InlineFragment(InlineFragment),
    FragmentSpread(ResolvedFragmentId),
    Typename,
}

impl Selection {
    pub(crate) fn as_selected_field(&self) -> Option<&SelectedField> {
        match self {
            Selection::Field(f) => Some(f),
            _ => None,
        }
    }

    pub(crate) fn as_inline_fragment(&self) -> Option<&InlineFragment> {
        match self {
            Selection::InlineFragment(f) => Some(f),
            _ => None,
        }
    }

    pub(crate) fn collect_used_types<'a, T>(
        &self,
        used_types: &mut UsedTypes,
        query: &'a BoundQuery<'a, '_, '_, T>,
    ) where
        T: graphql_parser::query::Text<'a> + std::default::Default,
    {
        match self {
            Selection::Field(field) => {
                let stored_field = query.schema.get_field(field.field_id);
                used_types.types.insert(stored_field.r#type.id);

                for selection_id in self.subselection() {
                    let selection = query.query.get_selection(*selection_id);
                    selection.collect_used_types(used_types, query);
                }
            }
            Selection::InlineFragment(inline_fragment) => {
                used_types.types.insert(inline_fragment.type_id);

                for selection_id in self.subselection() {
                    let selection = query.query.get_selection(*selection_id);
                    selection.collect_used_types(used_types, query);
                }
            }
            Selection::FragmentSpread(fragment_id) => {
                // This is necessary to avoid infinite recursion.
                if used_types.fragments.contains(fragment_id) {
                    return;
                }

                used_types.fragments.insert(*fragment_id);

                let fragment = query.query.get_fragment(*fragment_id);

                for (_id, selection) in query.query.walk_selection_set(&fragment.selection_set) {
                    selection.collect_used_types(used_types, query);
                }
            }
            Selection::Typename => (),
        }
    }

    pub(crate) fn contains_fragment<'a, T>(
        &self,
        fragment_id: ResolvedFragmentId,
        query: &Query<'a, T>,
    ) -> bool
    where
        T: graphql_parser::query::Text<'a> + std::default::Default,
    {
        match self {
            Selection::FragmentSpread(id) => *id == fragment_id,
            _ => self.subselection().iter().any(|selection_id| {
                query
                    .get_selection(*selection_id)
                    .contains_fragment(fragment_id, query)
            }),
        }
    }

    pub(crate) fn subselection(&self) -> &[SelectionId] {
        match self {
            Selection::Field(field) => field.selection_set.as_slice(),
            Selection::InlineFragment(inline_fragment) => &inline_fragment.selection_set,
            _ => &[],
        }
    }

    pub(super) fn to_path_segment<'a, T>(&self, query: &BoundQuery<'a, '_, '_, T>) -> String
    where
        T: graphql_parser::query::Text<'a> + std::default::Default,
    {
        match self {
            Selection::Field(field) => field
                .alias
                .as_ref()
                .map(|alias| alias.to_camel_case())
                .unwrap_or_else(move || {
                    query.schema.get_field(field.field_id).name.to_camel_case()
                }),
            Selection::InlineFragment(inline_fragment) => format!(
                "On{}",
                inline_fragment.type_id.name(query.schema).to_camel_case()
            ),
            other => unreachable!("{:?} in to_path_segment", other),
        }
    }
}

#[derive(Debug)]
pub(crate) struct InlineFragment {
    pub(crate) type_id: TypeId,
    pub(crate) selection_set: Vec<SelectionId>,
}

#[derive(Debug)]
pub(crate) struct SelectedField {
    // TODO make this Option<graphql_parser::query::Text>?
    pub(crate) alias: Option<String>,
    pub(crate) field_id: StoredFieldId,
    pub(crate) selection_set: Vec<SelectionId>,
}

impl SelectedField {
    pub(crate) fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    pub(crate) fn schema_field<'a>(&self, schema: &'a Schema) -> &'a StoredField {
        schema.get_field(self.field_id)
    }
}
