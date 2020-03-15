use crate::codegen_options::GraphQLClientCodegenOptions;
use crate::resolution::{OperationRef, UsedTypes};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub(super) fn generate_input_object_definitions(
    operation: &OperationRef<'_>,
    all_used_types: &UsedTypes,
    _options: &GraphQLClientCodegenOptions,
) -> Vec<TokenStream> {
    all_used_types
        .inputs(operation.schema())
        .map(|input| {
            let struct_name = Ident::new(input.name(), Span::call_site());

            let fields = input.fields().map(|field| {
                let name_ident = Ident::new(field.name(), Span::call_site());
                let type_name = Ident::new(field.field_type_name(), Span::call_site());
                let field_type = super::decorate_type(&type_name, field.field_type_qualifiers());
                let field_type = if input.is_recursive_without_indirection() {
                    quote!(Box<#field_type>)
                } else {
                    field_type
                };
                quote!(pub #name_ident: #field_type)
            });

            quote! {
                #[derive(Serialize)]
                pub struct #struct_name {
                    #(#fields,)*
                }
            }
        })
        .collect()
}
