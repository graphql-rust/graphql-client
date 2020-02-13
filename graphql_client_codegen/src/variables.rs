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

fn graphql_parser_value_to_literal(
    value: &graphql_parser::query::Value,
    context: &QueryContext<'_>,
    ty: &FieldType<'_>,
    is_optional: bool,
) -> TokenStream {
    use graphql_parser::query::Value;

    let inner = match value {
        Value::Boolean(b) => {
            if *b {
                quote!(true)
            } else {
                quote!(false)
            }
        }
        Value::String(s) => quote!(#s.to_string()),
        Value::Variable(_) => panic!("variable in variable"),
        Value::Null => panic!("null as default value"),
        Value::Float(f) => quote!(#f),
        Value::Int(i) => {
            let i = i.as_i64();
            quote!(#i)
        }
        Value::Enum(en) => quote!(#en),
        Value::List(inner) => {
            let elements = inner
                .iter()
                .map(|val| graphql_parser_value_to_literal(val, context, ty, false));
            quote! {
                vec![
                    #(#elements,)*
                ]
            }
        }
        Value::Object(obj) => render_object_literal(obj, ty, context),
    };

    if is_optional {
        quote!(Some(#inner))
    } else {
        inner
    }
}

fn render_object_literal(
    object: &BTreeMap<String, graphql_parser::query::Value>,
    ty: &FieldType<'_>,
    context: &QueryContext<'_>,
) -> TokenStream {
    unimplemented!()
    // let type_name = ty.inner_name_str();
    // let constructor = Ident::new(&type_name, Span::call_site());
    // let schema_type = context
    //     .schema
    //     .inputs
    //     .get(type_name)
    //     .expect("unknown input type");
    // let fields: Vec<TokenStream> = schema_type
    //     .fields
    //     .iter()
    //     .map(|(name, field)| {
    //         let field_name = Ident::new(&name, Span::call_site());
    //         let provided_value = object.get(name.to_owned());
    //         match provided_value {
    //             Some(default_value) => {
    //                 let value = graphql_parser_value_to_literal(
    //                     default_value,
    //                     context,
    //                     &field.type_,
    //                     field.type_.is_optional(),
    //                 );
    //                 quote!(#field_name: #value)
    //             }
    //             None => quote!(#field_name: None),
    //         }
    //     })
    //     .collect();

    // quote!(#constructor {
    //     #(#fields,)*
    // })
}
