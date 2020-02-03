use crate::{normalization::Normalization, resolution::*, GraphQLClientCodegenOptions};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// Selects the first operation matching `struct_name`. Returns `None` when the query document defines no operation, or when the selected operation does not match any defined operation.
pub(crate) fn select_operation<'a>(
    query: &'a ResolvedQuery,
    struct_name: &str,
    norm: Normalization,
) -> Option<usize> {
    query
        .operations
        .iter()
        .position(|op| normalization.operation(op.name()) == struct_name)
}

/// The main code generation function.
pub(crate) fn response_for_query(
    operation: Operation<'_>,
    options: &GraphQLClientCodegenOptions,
) -> anyhow::Result<TokenStream> {
    let all_used_types = operation.all_used_types();
    let scalar_definitions = generate_scalar_definitions(operation, &all_used_types);
    let enum_definitions = generate_enum_definitions(operation, &all_used_types, options);
    // let mut context = QueryContext::new(
    //     schema,
    //     options.deprecation_strategy(),
    //     options.normalization(),
    // );

    // if let Some(derives) = options.variables_derives() {
    //     context.ingest_variables_derives(&derives)?;
    // }

    // if let Some(derives) = options.response_derives() {
    //     context.ingest_response_derives(&derives)?;
    // }

    // let resolved_query = crate::resolution::resolve(schema, query)?;
    // crate::rendering::render(schema, &resolved_query)

    // context.resolve_fragments(&query.definitions);

    // let module = context.types_for_operation(operation);

    // let response_data_fields = {
    //     let root_name = operation.root_name(&schema);
    //     let opt_definition = schema.get_object_by_name(&root_name);
    //     let definition = if let Some(definition) = opt_definition {
    //         definition
    //     } else {
    //         panic!(
    //             "operation type '{:?}' not in schema",
    //             operation.operation_type
    //         );
    //     };
    //     let prefix = &operation.name;
    //     let selection = &operation.selection;

    //     if operation.is_subscription() && selection.len() > 1 {
    //         return Err(format_err!(
    //             "{}",
    //             crate::constants::MULTIPLE_SUBSCRIPTION_FIELDS_ERROR
    //         ));
    //     }

    //     definitions.extend(definition.field_impls_for_selection(&context, &selection, &prefix)?);
    //     definition.response_fields_for_selection(&context, &selection, &prefix)?
    // };

    // let enum_definitions = schema.enums.values().filter_map(|enm| {
    //     if enm.is_required.get() {
    //         Some(enm.to_rust(&context))
    //     } else {
    //         None
    //     }
    // });
    // let fragment_definitions: Result<Vec<TokenStream>, _> = context
    //     .fragments
    //     .values()
    //     .filter_map(|fragment| {
    //         if fragment.is_required.get() {
    //             Some(fragment.to_rust(&context))
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();
    // let fragment_definitions = fragment_definitions?;
    // let variables_struct = operation.expand_variables(&context);

    // let input_object_definitions: Result<Vec<TokenStream>, _> = schema
    //     .inputs
    //     .values()
    //     .filter_map(|i| {
    //         if i.is_required.get() {
    //             Some(i.to_rust(&context))
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();
    // let input_object_definitions = input_object_definitions?;

    // let scalar_definitions: Vec<TokenStream> = schema
    //     .scalars
    //     .values()
    //     .filter_map(|s| {
    //         if s.is_required.get() {
    //             Some(s.to_rust(context.normalization))
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();

    // let response_derives = context.response_derives();

    Ok(quote! {
        use serde::{Serialize, Deserialize};

        #[allow(dead_code)]
        type Boolean = bool;
        #[allow(dead_code)]
        type Float = f64;
        #[allow(dead_code)]
        type Int = i64;
        #[allow(dead_code)]
        type ID = String;

        #(#scalar_definitions)*

        #(#enum_definitions)*

        #(#fragment_definitions)*

        #(#definitions)*

        #(#input_object_definitions)*

        #variables_struct

        #response_derives

        pub struct ResponseData {
            #(#response_data_fields,)*
        }

    })
}

fn generate_scalar_definitions<'a, 'schema: 'a>(
    operation: Operation<'schema>,
    all_used_types: &'a crate::resolution::UsedTypes,
) -> impl Iterator<Item = TokenStream> + 'a {
    all_used_types.scalars(operation.schema()).map(|scalar| {
        let ident = syn::Ident::new(scalar.name(), proc_macro2::Span::call_site());
        quote!(type #ident = super::#ident;)
    })
}

/**
 * About rust keyword escaping: variant_names and constructors must be escaped,
 * variant_str not.
 * Example schema:                  enum AnEnum { where \n self }
 * Generated "variant_names" enum:  pub enum AnEnum { where_, self_, Other(String), }
 * Generated serialize line: "AnEnum::where_ => "where","
 */
fn generate_enum_definitions<'a, 'schema: 'a>(
    operation: Operation<'schema>,
    all_used_types: &'a crate::resolution::UsedTypes,
    options: &GraphQLClientCodegenOptions,
) -> impl Iterator<Item = TokenStream> + 'a {
    let derives = options.response_derives();
    let normalization = options.normalization();

    all_used_types.enums(operation.schema()).map(|r#enum| {
        let ident = syn::Ident::new(r#enum.name(), proc_macro2::Span::call_site());

        let variant_names: Vec<TokenStream> = r#enum
            .variants()
            .iter()
            .map(|v| {
                let name = normalization.enum_variant(crate::shared::keyword_replace(&v));
                let name = Ident::new(&name, Span::call_site());

                // let description = &v.description;
                // let description = description.as_ref().map(|d| quote!(#[doc = #d]));

                // quote!(#description #name)
                quote!(#name)
            })
            .collect();
        let variant_names = &variant_names;
        let name_ident = normalization.enum_name(format!("{}{}", ENUMS_PREFIX, r#enum.name()));
        let name_ident = Ident::new(&name_ident, Span::call_site());
        let constructors: Vec<_> = r#enum
            .variants()
            .iter()
            .map(|v| {
                let name = normalization.enum_variant(crate::shared::keyword_replace(v));
                let v = Ident::new(&name, Span::call_site());

                quote!(#name_ident::#v)
            })
            .collect();
        let constructors = &constructors;
        let variant_str: Vec<&str> = r#enum.variants().iter().map(|s| s.as_str()).collect();
        let variant_str = &variant_str;

        let name = name_ident;

        quote! {
            #derives
            pub enum #name {
                #(#variant_names,)*
                Other(String),
            }

            impl ::serde::Serialize for #name {
                fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                    ser.serialize_str(match *self {
                        #(#constructors => #variant_str,)*
                        #name::Other(ref s) => &s,
                    })
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name {
                fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    let s = <String>::deserialize(deserializer)?;

                    match s.as_str() {
                        #(#variant_str => Ok(#constructors),)*
                        _ => Ok(#name::Other(s)),
                    }
                }
            }
        }})
}
