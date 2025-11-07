use super::{Query, ResolvedFragmentId, SelectionId};
use crate::schema::TypeId;
use heck::ToUpperCamelCase;

#[derive(Debug)]
pub(crate) struct ResolvedFragment {
    pub(crate) name: String,
    pub(crate) on: TypeId,
    pub(crate) selection_set: Vec<SelectionId>,
}

impl ResolvedFragment {
    pub(super) fn to_path_segment(&self) -> String {
        self.name.to_upper_camel_case()
    }
}

pub(crate) fn fragment_is_recursive(fragment_id: ResolvedFragmentId, query: &Query) -> bool {
    let fragment = query.get_fragment(fragment_id);

    query
        .walk_selection_set(&fragment.selection_set)
        .any(|(_id, selection)| selection.contains_fragment(fragment_id, query))
}
