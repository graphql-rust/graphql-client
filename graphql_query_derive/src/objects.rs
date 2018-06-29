use constants::*;
use failure;
use field_type::FieldType;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use selection::*;
use shared::{field_impls_for_selection, response_fields_for_selection};
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub struct GqlObject {
    pub name: String,
    pub fields: Vec<GqlObjectField>,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct GqlObjectField {
    pub name: String,
    pub type_: FieldType,
}

impl GqlObject {
    pub fn new(name: Cow<str>) -> GqlObject {
        GqlObject {
            name: name.into_owned(),
            fields: vec![typename_field()],
        }
    }

    pub fn from_graphql_parser_object(obj: ::graphql_parser::schema::ObjectType) -> Self {
        let mut item = GqlObject::new(obj.name.into());
        item.fields
            .extend(obj.fields.iter().map(|f| GqlObjectField {
                name: f.name.clone(),
                type_: FieldType::from(f.field_type.clone()),
            }));
        item
    }

    pub fn from_introspected_schema_json(obj: &::introspection_response::FullType) -> Self {
        let mut item = GqlObject::new(obj.name.clone().expect("missing object name").into());
        let fields = obj.fields.clone().unwrap().into_iter().filter_map(|t| {
            t.map(|t| GqlObjectField {
                name: t.name.expect("field name"),
                type_: FieldType::from(t.type_.expect("field type")),
            })
        });

        item.fields.extend(fields);

        item
    }

    pub fn response_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<TokenStream, failure::Error> {
        let name = Ident::new(prefix, Span::call_site());
        let fields = self.response_fields_for_selection(query_context, selection, prefix)?;
        let field_impls = self.field_impls_for_selection(query_context, selection, &prefix)?;
        Ok(quote! {
            #(#field_impls)*

            #[derive(Debug, Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct #name {
                #(#fields,)*
            }
        })
    }

    pub fn field_impls_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<Vec<TokenStream>, failure::Error> {
        field_impls_for_selection(&self.fields, query_context, selection, prefix)
    }

    pub fn response_fields_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<Vec<TokenStream>, failure::Error> {
        response_fields_for_selection(&self.fields, query_context, selection, prefix)
    }
}
