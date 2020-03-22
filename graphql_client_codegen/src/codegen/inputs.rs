use super::shared::keyword_replace;
use crate::{
    codegen_options::GraphQLClientCodegenOptions,
    query::{OperationRef, UsedTypes},
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub(super) fn generate_input_object_definitions(
    operation: &OperationRef<'_>,
    all_used_types: &UsedTypes,
    options: &GraphQLClientCodegenOptions,
    variable_derives: &impl quote::ToTokens,
) -> Vec<TokenStream> {
    all_used_types
        .inputs(operation.schema())
        .map(|input| {
            let normalized_name = options.normalization().input_name(input.name());
            let safe_name = keyword_replace(normalized_name);
            let struct_name = Ident::new(safe_name.as_ref(), Span::call_site());

            let fields = input.fields().map(|field| {
                let safe_field_name = keyword_replace(field.name());
                let name_ident = Ident::new(safe_field_name.as_ref(), Span::call_site());
                let normalized_field_type_name =
                    options.normalization().field_type(field.field_type_name());
                let type_name = Ident::new(normalized_field_type_name.as_ref(), Span::call_site());
                let field_type = super::decorate_type(&type_name, field.field_type_qualifiers());
                let field_type = if field
                    .field_type_as_input()
                    .map(|input| input.is_recursive_without_indirection())
                    .unwrap_or(false)
                {
                    quote!(Box<#field_type>)
                } else {
                    field_type
                };
                quote!(pub #name_ident: #field_type)
            });

            quote! {
                #variable_derives
                pub struct #struct_name {
                    #(#fields,)*
                }
            }
        })
        .collect()
}
