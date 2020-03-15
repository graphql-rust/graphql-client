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
                quote!(pub #name_ident: String)
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
