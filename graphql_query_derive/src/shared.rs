use failure;
use heck::{CamelCase, SnakeCase};
use objects::GqlObjectField;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use selection::*;

pub(crate) fn render_object_field(field_name: &str, field_type: TokenStream) -> TokenStream {
    if field_name == "type" {
        let name_ident = Ident::new(&format!("{}_", field_name), Span::call_site());
        return quote! {
            #[serde(rename = #field_name)]
            pub #name_ident: #field_type
        };
    }

    let name_ident = Ident::new(field_name, Span::call_site());

    quote!(pub #name_ident: #field_type)
}

pub(crate) fn field_impls_for_selection(
    fields: &[GqlObjectField],
    context: &QueryContext,
    selection: &Selection,
    prefix: &str,
) -> Result<Vec<TokenStream>, failure::Error> {
    selection
        .0
        .iter()
        .map(|selected| {
            if let SelectionItem::Field(selected) = selected {
                let ty = fields
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
                context.maybe_expand_field(&ty, &selected.fields, &prefix)
            } else {
                Ok(quote!())
            }
        })
        .collect()
}

pub fn response_fields_for_selection(
    schema_fields: &[GqlObjectField],
    context: &QueryContext,
    selection: &Selection,
    prefix: &str,
) -> Result<Vec<TokenStream>, failure::Error> {
    selection
        .0
        .iter()
        .map(|item| match item {
            SelectionItem::Field(f) => {
                let name = &f.name;

                let ty = &schema_fields
                    .iter()
                    .find(|field| field.name.as_str() == name.as_str())
                    .ok_or(format_err!("Could not find field: {}", name.as_str()))?
                    .type_;
                let ty = ty.to_rust(
                    context,
                    &format!("{}{}", prefix.to_camel_case(), name.to_camel_case()),
                );
                Ok(render_object_field(name, ty))
            }
            SelectionItem::FragmentSpread(fragment) => {
                let field_name =
                    Ident::new(&fragment.fragment_name.to_snake_case(), Span::call_site());
                let type_name = Ident::new(&fragment.fragment_name, Span::call_site());
                Ok(quote!{
                    #[serde(flatten)]
                    #field_name: #type_name
                })
            }
            SelectionItem::InlineFragment(_) => {
                Err(format_err!("inline fragment on object field"))?
            }
        })
        .collect()
}
