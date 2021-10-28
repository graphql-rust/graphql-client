use super::{full_path_prefix, BoundQuery, Query, QueryValidationError, Selection, SelectionId};
use crate::schema::TypeId;

pub(super) fn validate_typename_presence<'a, T>(
    query: &BoundQuery<'a, '_, '_, T>,
) -> Result<(), QueryValidationError>
where
    T: graphql_parser::query::Text<'a> + std::default::Default,
{
    for fragment in query.query.fragments.iter() {
        let type_id = match fragment.on {
            id @ TypeId::Interface(_) | id @ TypeId::Union(_) => id,
            _ => continue,
        };

        if !selection_set_contains_type_name(fragment.on, &fragment.selection_set, query.query) {
            return Err(QueryValidationError::new(format!(
                "The `{}` fragment uses `{}` but does not select `__typename` on it. graphql-client cannot generate code for it. Please add `__typename` to the selection.",
                &fragment.name,
                type_id.name(query.schema),
            )));
        }
    }

    let union_and_interface_field_selections =
        query
            .query
            .selections()
            .filter_map(|(selection_id, selection)| match selection {
                Selection::Field(field) => match query.schema.get_field(field.field_id).r#type.id {
                    id @ TypeId::Interface(_) | id @ TypeId::Union(_) => {
                        Some((selection_id, id, &field.selection_set))
                    }
                    _ => None,
                },
                _ => None,
            });

    for selection in union_and_interface_field_selections {
        if !selection_set_contains_type_name(selection.1, selection.2, query.query) {
            return Err(QueryValidationError::new(format!(
                "The query uses `{path}` at `{selected_type}` but does not select `__typename` on it. graphql-client cannot generate code for it. Please add `__typename` to the selection.",
                path = full_path_prefix(selection.0, query),
                selected_type = selection.1.name(query.schema)
            )));
        }
    }

    Ok(())
}

fn selection_set_contains_type_name<'a, T>(
    parent_type_id: TypeId,
    selection_set: &[SelectionId],
    query: &Query<'a, T>,
) -> bool
where
    T: graphql_parser::query::Text<'a> + std::default::Default,
{
    for id in selection_set {
        let selection = query.get_selection(*id);

        match selection {
            Selection::Typename => return true,
            Selection::FragmentSpread(fragment_id) => {
                let fragment = query.get_fragment(*fragment_id);
                if fragment.on == parent_type_id
                    && selection_set_contains_type_name(fragment.on, &fragment.selection_set, query)
                {
                    return true;
                }
            }
            _ => (),
        }
    }

    false
}
