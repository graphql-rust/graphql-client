use failure;
use field_type::FieldType;
use graphql_parser::query;
use heck::{CamelCase, SnakeCase};
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use shared::render_object_field;

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
        selection: &query::SelectionSet,
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
        selection: &query::SelectionSet,
        prefix: &str,
    ) -> Result<Vec<TokenStream>, failure::Error> {
        selection
            .items
            .iter()
            .map(|selected| {
                if let query::Selection::Field(selected) = selected {
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
                    query_context.maybe_expand_field(&selected, &ty, &prefix)
                } else {
                    Ok(quote!())
                }
            })
            .collect()
    }

    pub fn response_fields_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &query::SelectionSet,
        prefix: &str,
    ) -> Vec<TokenStream> {
        let mut fields = Vec::new();

        for item in selection.items.iter() {
            match item {
                query::Selection::Field(f) => {
                    let name = &f.name;
                    let ty = &self
                        .fields
                        .iter()
                        .find(|field| field.name.as_str() == name.as_str())
                        .unwrap()
                        .type_;
                    let ty = ty.to_rust(
                        query_context,
                        &format!("{}{}", prefix.to_camel_case(), name.to_camel_case()),
                    );
                    fields.push(render_object_field(name, ty));
                }
                query::Selection::FragmentSpread(fragment) => {
                    let field_name =
                        Ident::new(&fragment.fragment_name.to_snake_case(), Span::call_site());
                    let type_name = Ident::new(&fragment.fragment_name, Span::call_site());
                    fields.push(quote!{
                        #[serde(flatten)]
                        #field_name: #type_name
                    })
                }
                query::Selection::InlineFragment(_) => {
                    unreachable!("inline fragment on object field")
                }
            }
        }

        fields
    }
}
