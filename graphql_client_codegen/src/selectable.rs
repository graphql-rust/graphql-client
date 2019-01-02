/// This trait is meant to be implemented by all schema elements to which a SelectionSet can be applied: objects, interfaces and unions.
pub(crate) trait Selectable {
    /// The code for the selectable's corresponding struct's fields.
    fn response_fields_for_selection(
        &self,
        context: &crate::query::QueryContext,
        selection: &crate::selection::Selection,
        prefix: &str,
    ) -> Result<Vec<proc_macro2::TokenStream>, failure::Error>;
}

pub(crate) enum AnySelectable {
    Object(crate::objects::GqlObject),
    Union(crate::unions::GqlUnion),
    Interface(crate::interfaces::GqlInterface),
}

impl Selectable for AnySelectable {
    fn response_fields_for_selection(
        &self,
        context: &crate::query::QueryContext,
        selection: &crate::selection::Selection,
        prefix: &str,
    ) -> Result<Vec<proc_macro2::TokenStream>, failure::Error> {
        use self::AnySelectable::*;

        match self {
            Object(obj) => obj.response_fields_for_selection(context, selection, prefix),
            Union(un) => un.response_fields_for_selection(context, selection, prefix),
            Interface(iface) => iface.response_fields_for_selection(context, selection, prefix),
        }
    }
}
