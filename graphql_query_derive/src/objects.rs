use constants::*;
use failure;
use field_type::FieldType;
use heck::{CamelCase, SnakeCase};
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use selection::*;
use shared::render_object_field;
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub struct GqlObject {
    pub name: String,
    pub fields: Vec<GqlObjectField>,
}

#[derive(Debug, PartialEq)]
pub struct GqlObjectField {
    pub name: String,
    pub type_: FieldType,
}

impl GqlObject {
    pub fn new(name: Cow<str>) -> GqlObject {
        GqlObject {
            name: name.into_owned(),
            fields: vec![GqlObjectField {
                name: TYPENAME_FIELD.to_string(),
                /// Non-nullable, see spec:
                /// https://github.com/facebook/graphql/blob/master/spec/Section%204%20--%20Introspection.md
                type_: FieldType::Named(string_type()),
            }],
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
        let fields = self.response_fields_for_selection(query_context, selection, prefix);
        let field_impls = self.field_impls_for_selection(query_context, selection, &prefix)?;
        Ok(quote! {
            #(#field_impls)*

            #[derive(Debug, Serialize, Deserialize)]
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
        selection
            .0
            .iter()
            .map(|selected| {
                if let SelectionItem::Field(selected) = selected {
                    let ty = self
                        .fields
                        .iter()
                        .find(|f| f.name == selected.name)
                        .ok_or_else(|| format_err!("could not find field `{}`", selected.name))?
                        .type_
                        .inner_name_string();
                    let prefix = format!(
                        "{}{}",
                        prefix.to_camel_case(),
                        selected.name.to_camel_case()
                    );
                    query_context.maybe_expand_field(&ty, &selected.fields, &prefix)
                } else {
                    Ok(quote!())
                }
            })
            .collect()
    }

    pub fn response_fields_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Vec<TokenStream> {
        let mut fields = Vec::new();

        for item in selection.0.iter() {
            match item {
                SelectionItem::Field(f) => {
                    let name = &f.name;

                    let ty = &self
                        .fields
                        .iter()
                        .find(|field| field.name.as_str() == name.as_str())
                        .expect("could not find field")
                        .type_;
                    let ty = ty.to_rust(
                        query_context,
                        &format!("{}{}", prefix.to_camel_case(), name.to_camel_case()),
                    );
                    fields.push(render_object_field(name, ty));
                }
                SelectionItem::FragmentSpread(fragment) => {
                    let field_name =
                        Ident::new(&fragment.fragment_name.to_snake_case(), Span::call_site());
                    let type_name = Ident::new(&fragment.fragment_name, Span::call_site());
                    fields.push(quote!{
                        #[serde(flatten)]
                        #field_name: #type_name
                    })
                }
                SelectionItem::InlineFragment(_) => unreachable!("inline fragment on object field"),
            }
        }

        fields
    }
}
