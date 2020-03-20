use crate::field_type::FieldType;
use crate::query::QueryContext;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Variable<'query> {
    pub name: &'query str,
    pub ty: FieldType<'query>,
    pub default: Option<&'query graphql_parser::query::Value>,
}

impl<'query> Variable<'query> {
    pub(crate) fn generate_default_value_constructor(
        &self,
        context: &QueryContext<'_>,
        schema: &crate::schema::Schema,
    ) -> Option<TokenStream> {
        todo!("generate default value constructor")
        // // TODO
        // // context.schema.require(&self.ty.inner_name_str());
        // match &self.default {
        //     Some(default) => {
        //         let fn_name = Ident::new(&format!("default_{}", self.name), Span::call_site());
        //         let ty = self.ty.to_rust(context, "");
        //         let value = graphql_parser_value_to_literal(
        //             default,
        //             context,
        //             &self.ty,
        //             self.ty.is_optional(),
        //         );
        //         Some(quote! {
        //             pub fn #fn_name() -> #ty {
        //                 #value
        //             }

        //         })
        //     }
        //     None => None,
        // }
    }
}

impl<'query> std::convert::From<&'query graphql_parser::query::VariableDefinition>
    for Variable<'query>
{
    fn from(def: &'query graphql_parser::query::VariableDefinition) -> Variable<'query> {
        Variable {
            name: &def.name,
            ty: FieldType::from(&def.var_type),
            default: def.default_value.as_ref(),
        }
    }
}
