use super::shared::{field_rename_annotation, keyword_replace};
use crate::{
    codegen_options::GraphQLClientCodegenOptions,
    query::{BoundQuery, UsedTypes},
    schema::{input_is_recursive_without_indirection, StoredInputType},
    type_qualifiers::GraphqlTypeQualifier,
};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

pub(super) fn generate_input_object_definitions(
    all_used_types: &UsedTypes,
    options: &GraphQLClientCodegenOptions,
    variable_derives: &impl quote::ToTokens,
    query: &BoundQuery<'_>,
) -> Vec<TokenStream> {
    let custom_variable_types = options.custom_variable_types();
    all_used_types
        .inputs(query.schema)
        .map(|(input_id, input)| {
            let custom_variable_type = query
                .query
                .variables
                .iter()
                .enumerate()
                .find(|(_, v)| {
                    v.r#type
                        .id
                        .as_input_id()
                        .map(|i| i == input_id)
                        .unwrap_or_default()
                })
                .and_then(|(index, _)| custom_variable_types.get(index));
            if let Some(custom_type) = custom_variable_type {
                generate_type_def(input, options, custom_type)
            } else if input.is_one_of {
                generate_enum(input, options, variable_derives, query)
            } else {
                generate_struct(input, options, variable_derives, query)
            }
        })
        .collect()
}

fn generate_type_def(
    input: &StoredInputType,
    options: &GraphQLClientCodegenOptions,
    custom_type: &str,
) -> TokenStream {
    let custom_type = syn::parse_str::<syn::Path>(custom_type).unwrap();
    let normalized_name = options.normalization().input_name(input.name.as_str());
    let safe_name = keyword_replace(normalized_name);
    let struct_name = Ident::new(safe_name.as_ref(), Span::call_site());
    quote!(pub type #struct_name = #custom_type;)
}

fn generate_struct(
    input: &StoredInputType,
    options: &GraphQLClientCodegenOptions,
    variable_derives: &impl quote::ToTokens,
    query: &BoundQuery<'_>,
) -> TokenStream {
    let serde = options.serde_path();
    let serde_path = serde.to_token_stream().to_string();

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
        let field_type_tokens = super::decorate_type(&type_name, &field_type.qualifiers, false);
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
        #[serde(crate = #serde_path)]
        pub struct #struct_name{
            #(#fields,)*
        }
    }
}

fn generate_enum(
    input: &StoredInputType,
    options: &GraphQLClientCodegenOptions,
    variable_derives: &impl quote::ToTokens,
    query: &BoundQuery<'_>,
) -> TokenStream {
    let normalized_name = options.normalization().input_name(input.name.as_str());
    let safe_name = keyword_replace(normalized_name);
    let enum_name = Ident::new(safe_name.as_ref(), Span::call_site());

    let variants = input.fields.iter().map(|(field_name, field_type)| {
        let variant_name = field_name.to_upper_camel_case();
        let safe_variant_name = keyword_replace(&variant_name);

        let annotation = field_rename_annotation(field_name.as_ref(), &variant_name);
        let name_ident = Ident::new(safe_variant_name.as_ref(), Span::call_site());

        let normalized_field_type_name = options
            .normalization()
            .field_type(field_type.id.name(query.schema));
        let type_name = Ident::new(normalized_field_type_name.as_ref(), Span::call_site());

        // Add the required qualifier so that the variant's field isn't wrapped in Option
        let mut qualifiers = vec![GraphqlTypeQualifier::Required];
        qualifiers.extend(field_type.qualifiers.iter().cloned());

        let field_type_tokens = super::decorate_type(&type_name, &qualifiers, false);
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
            #annotation #name_ident(#field_type)
        )
    });

    quote! {
        #variable_derives
        pub enum #enum_name{
            #(#variants,)*
        }
    }
}
