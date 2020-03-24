use super::{Query, QueryWith, ResolvedFragmentId, SelectionId, SelectionRef};
use crate::schema::{Schema, TypeId, TypeRef};
use heck::*;

#[derive(Debug)]
pub(crate) struct ResolvedFragment {
    pub(crate) name: String,
    pub(crate) on: crate::schema::TypeId,
    pub(crate) selection: Vec<SelectionId>,
}

pub(crate) struct FragmentRef<'a>(
    pub(super) QueryWith<'a, (ResolvedFragmentId, &'a ResolvedFragment)>,
);

impl<'a> FragmentRef<'a> {
    pub(crate) fn is_recursive(&self) -> bool {
        let id = self.0.focus.0;

        self.selection_set()
            .any(|selection| selection.contains_fragment(id))
    }

    pub(crate) fn query(&self) -> &'a Query {
        self.0.query
    }

    pub(crate) fn name(&self) -> &'a str {
        &self.0.focus.1.name
    }

    pub(crate) fn on(&self) -> TypeId {
        self.0.focus.1.on
    }

    pub(crate) fn on_ref(&self) -> TypeRef<'a> {
        self.0.schema.type_ref(self.0.focus.1.on)
    }

    pub(crate) fn schema(&self) -> &'a Schema {
        self.0.schema
    }

    pub(crate) fn selection_ids(&self) -> &[SelectionId] {
        &self.0.focus.1.selection
    }

    pub(crate) fn selection_set<'b>(&'b self) -> impl Iterator<Item = SelectionRef<'a>> + 'b {
        self.selection_ids()
            .iter()
            .map(move |id| self.0.query.get_selection_ref(self.0.schema, *id))
    }

    pub(super) fn to_path_segment(&self) -> String {
        self.0.focus.1.name.to_camel_case()
    }
}
