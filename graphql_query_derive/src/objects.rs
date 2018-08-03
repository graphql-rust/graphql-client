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
    pub description: Option<String>,
    pub fields: Vec<GqlObjectField>,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct GqlObjectField {
    pub description: Option<String>,
    pub name: String,
    pub type_: FieldType,
}

impl GqlObject {
    pub fn new(name: Cow<str>, description: Option<&str>) -> GqlObject {
        GqlObject {
            description: description.map(|s| s.to_owned()),
            name: name.into_owned(),
            fields: vec![typename_field()],
        }
    }

    pub fn from_graphql_parser_object(obj: ::graphql_parser::schema::ObjectType) -> Self {
        let description = obj.description.as_ref().map(|s| s.as_str());
        let mut item = GqlObject::new(obj.name.into(), description);
        item.fields
            .extend(obj.fields.iter().map(|f| GqlObjectField {
                description: f.description.clone(),
                name: f.name.clone(),
                type_: FieldType::from(f.field_type.clone()),
            }));
        item
    }

    pub fn from_introspected_schema_json(obj: &::introspection_response::FullType) -> Self {
        let description = obj.description.as_ref().map(|s| s.as_str());
        let mut item = GqlObject::new(
            obj.name.clone().expect("missing object name").into(),
            description,
        );
        let fields = obj.fields.clone().unwrap().into_iter().filter_map(|t| {
            t.map(|t| GqlObjectField {
                description: t.description.clone(),
                name: t.name.expect("field name"),
                type_: FieldType::from(t.type_.expect("field type")),
            })
        });

        item.fields.extend(fields);

        item
    }

    pub(crate) fn response_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<TokenStream, failure::Error> {
        let name = Ident::new(prefix, Span::call_site());
        let fields = self.response_fields_for_selection(query_context, selection, prefix)?;
        let field_impls = self.field_impls_for_selection(query_context, selection, &prefix)?;
        let description = self.description.as_ref().map(|desc| quote!(#[doc = #desc]));
        Ok(quote! {
            #(#field_impls)*

            #[derive(Debug, Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            #description
            pub struct #name {
                #(#fields,)*
            }
        })
    }

    pub(crate) fn field_impls_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<Vec<TokenStream>, failure::Error> {
        field_impls_for_selection(&self.fields, query_context, selection, prefix)
    }

    pub(crate) fn response_fields_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<Vec<TokenStream>, failure::Error> {
        response_fields_for_selection(&self.fields, query_context, selection, prefix)
    }
}
