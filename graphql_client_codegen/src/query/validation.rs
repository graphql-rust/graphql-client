use super::{full_path_prefix, BoundQuery, Query, QueryValidationError, Selection, SelectionId};
use crate::schema::TypeId;

pub(super) fn validate_typename_presence(
    query: &BoundQuery<'_>,
) -> Result<(), QueryValidationError> {
    for fragment in &query.query.fragments {
        let type_id @ (TypeId::Interface(_) | TypeId::Union(_)) = fragment.on else {
            continue;
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
            .filter_map(|(selection_id, selection)| {
                if let Selection::Field(field) = selection {
                    let field_type_id = query.schema.get_field(field.field_id).r#type.id;

                    if matches!(field_type_id, TypeId::Interface(_) | TypeId::Union(_)) {
                        return Some((selection_id, field_type_id, &field.selection_set));
                    }
                }
                None
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

fn selection_set_contains_type_name(
    parent_type_id: TypeId,
    selection_set: &[SelectionId],
    query: &Query,
) -> bool {
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
