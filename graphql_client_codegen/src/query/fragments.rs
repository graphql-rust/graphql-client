use super::{Query, ResolvedFragmentId, SelectionId};
use crate::schema::TypeId;
use heck::*;

#[derive(Debug)]
pub(crate) struct ResolvedFragment {
    pub(crate) name: String,
    pub(crate) on: TypeId,
    pub(crate) selection_set: Vec<SelectionId>,
}

impl ResolvedFragment {
    pub(super) fn to_path_segment(&self) -> String {
        self.name.to_camel_case()
    }
}

pub(crate) fn fragment_is_recursive<'a, T>(
    fragment_id: ResolvedFragmentId,
    query: &'a Query<'a, T>,
) -> bool
where
    T: graphql_parser::query::Text<'a> + std::default::Default,
{
    let fragment = query.get_fragment(fragment_id);

    query
        .walk_selection_set(&fragment.selection_set)
        .any(|(_id, selection)| selection.contains_fragment(fragment_id, query))
}
