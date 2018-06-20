use failure;
use field_type::FieldType;
use heck::{CamelCase, SnakeCase};
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use shared::render_object_field;
use selection::*;

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
                SelectionItem::InlineFragment(_) => {
                    unreachable!("inline fragment on object field")
                }
            }
        }

        fields
    }
}
