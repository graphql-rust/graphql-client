use super::shared::{field_rename_annotation, keyword_replace};
use crate::{
    codegen_options::GraphQLClientCodegenOptions,
    query::{BoundQuery, UsedTypes},
    schema::input_is_recursive_without_indirection,
};
use heck::ToSnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub(super) fn generate_input_object_definitions(
    all_used_types: &UsedTypes,
    options: &GraphQLClientCodegenOptions,
    variable_derives: &impl quote::ToTokens,
    query: &BoundQuery<'_>,
) -> Vec<TokenStream> {
    all_used_types
        .inputs(query.schema)
        .map(|(_input_id, input)| {
            let normalized_name = options.normalization().input_name(input.name.as_str());
            let safe_name = keyword_replace(normalized_name);
            let struct_name = Ident::new(safe_name.as_ref(), Span::call_site());

            let fields = input.fields.iter().map(|(field_name, field_type)| {
                let safe_field_name = keyword_replace(field_name.to_snake_case());
                let annotation = field_rename_annotation(field_name, safe_field_name.as_ref());
                let name_ident = Ident::new(safe_field_name.as_ref(), Span::call_site());
                let normalized_field_type_name = options
                    .normalization()
                    .field_type(field_type.id.name(query.schema));
                let optional_skip_serializing_none =
                    if *options.skip_serializing_none() && field_type.is_optional() {
                        Some(quote!(#[serde(skip_serializing_if = "Option::is_none")]))
                    } else {
                        None
                    };
                let type_name = Ident::new(normalized_field_type_name.as_ref(), Span::call_site());
                let field_type_tokens = super::decorate_type(&type_name, &field_type.qualifiers);
                let field_type = if field_type
                    .id
                    .as_input_id()
                    .map(|input_id| input_is_recursive_without_indirection(input_id, query.schema))
                    .unwrap_or(false)
                {
                    quote!(Box<#field_type_tokens>)
                } else {
                    field_type_tokens
                };

                quote!(
                    #optional_skip_serializing_none
                    #annotation pub #name_ident: #field_type
                )
            });

            quote! {
                #variable_derives
                pub struct #struct_name{
                    #(#fields,)*
                }
            }
        })
        .collect()
}
