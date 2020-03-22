use super::{
    OperationId, QueryWith, ResolvedQuery, SelectionId, SelectionRef, UsedTypes, VariableId,
    VariableRef,
};
use crate::schema::{ObjectId, Schema, TypeId, TypeRef};
use heck::*;

pub(crate) struct OperationRef<'a>(pub(super) QueryWith<'a, OperationId>);

impl<'a> OperationRef<'a> {
    pub(crate) fn query(&self) -> &'a ResolvedQuery {
        self.0.query
    }

    pub(crate) fn schema(&self) -> &'a Schema {
        self.0.schema
    }

    fn get(&self) -> &'a ResolvedOperation {
        self.0
            .query
            .operations
            .get(self.0.focus.0 as usize)
            .expect("get operation")
    }

    pub(crate) fn to_path_segment(&self) -> String {
        self.get().name.to_camel_case()
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

    pub(crate) fn selection<'b>(&'b self) -> impl Iterator<Item = SelectionRef<'a>> + 'b {
        let operation = self.get();
        operation.selection.iter().map(move |selection_id| {
            SelectionRef(
                self.0
                    .refocus((*selection_id, self.0.query.get_selection(*selection_id))),
            )
        })
    }

    pub(crate) fn selection_ids(&self) -> &[SelectionId] {
        &self.get().selection
    }

    pub(crate) fn variables<'b>(&'b self) -> impl Iterator<Item = VariableRef<'a>> + 'b {
        self.0
            .query
            .variables
            .iter()
            .enumerate()
            .filter(move |(_, variable)| variable.operation_id == self.0.focus)
            .map(move |(id, _)| {
                self.0
                    .query
                    .get_variable_ref(self.0.schema, VariableId::new(id))
            })
    }

    pub(crate) fn name(&self) -> &'a str {
        self.get().name()
    }

    pub(crate) fn has_no_variables(&self) -> bool {
        self.variables().next().is_none()
    }

    pub(crate) fn on_ref(&self) -> TypeRef<'a> {
        self.0.schema.type_ref(TypeId::Object(self.get().object_id))
    }
}

pub(crate) struct ResolvedOperation {
    pub(crate) name: String,
    pub(crate) _operation_type: crate::operations::OperationType,
    pub(crate) selection: Vec<SelectionId>,
    pub(crate) object_id: ObjectId,
}

impl ResolvedOperation {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}
