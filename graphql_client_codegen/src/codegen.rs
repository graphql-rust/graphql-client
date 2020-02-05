use crate::{
    field_type::GraphqlTypeQualifier, normalization::Normalization, resolution::*,
    GraphQLClientCodegenOptions,
};
use heck::SnakeCase;
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
        .position(|op| norm.operation(op.name()) == struct_name)
}

/// The main code generation function.
pub(crate) fn response_for_query(
    operation: Operation<'_>,
    options: &GraphQLClientCodegenOptions,
) -> anyhow::Result<TokenStream> {
    let all_used_types = operation.all_used_types();
    let scalar_definitions = generate_scalar_definitions(operation, &all_used_types);
    let enum_definitions = generate_enum_definitions(operation, &all_used_types, options);
    let fragment_definitions: Vec<&'static str> = Vec::new();
    let definitions: Vec<&'static str> = Vec::new();
    let input_object_definitions: Vec<&'static str> = Vec::new();
    let variable_derives = options
        .variables_derives()
        .unwrap_or("Serialize")
        .split(",");

    let variables_struct = generate_variables_struct(operation, options);

    let response_derives = render_derives(options.all_response_derives());
    let response_data_fields: Vec<&'static str> = Vec::new();

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

    let q = quote! {
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

        #(#input_object_definitions)*

        #(#definitions)*

        #response_derives
        pub struct ResponseData {
            #(#response_data_fields,)*
        }

        #variables_struct
    };

    Ok(q)
}

fn generate_variables_struct(
    operation: Operation<'_>,
    options: &GraphQLClientCodegenOptions,
) -> TokenStream {
    let variable_derives = options
        .variables_derives()
        .unwrap_or("Serialize")
        .split(",");
    let variable_derives = render_derives(variable_derives);

    if operation.has_no_variables() {
        return quote!(
            #variable_derives
            pub struct Variables;
        );
    }

    let variable_fields = operation.variables().map(generate_variable_struct_field);

    let variables_struct = quote!(
        #variable_derives
        pub struct Variables {
            #(#variable_fields,)*
        }
    );

    variables_struct.into()
}

fn generate_variable_struct_field(variable: Variable<'_>) -> TokenStream {
    let snake_case_name = variable.name().to_snake_case();
    let ident = Ident::new(
        &crate::shared::keyword_replace(&snake_case_name),
        Span::call_site(),
    );
    let annotation = crate::shared::field_rename_annotation(variable.name(), &snake_case_name);
    let r#type = render_variable_field_type(variable);

    quote::quote!(#annotation #ident : #r#type)
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
    options: &'a GraphQLClientCodegenOptions,
) -> impl Iterator<Item = TokenStream> + 'a {
    let derives = render_derives(options.additional_response_derives());
    let normalization = options.normalization();

    all_used_types.enums(operation.schema()).map(move |r#enum| {
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
        let name_ident = normalization.enum_name(r#enum.name());
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

fn render_derives<'a>(derives: impl Iterator<Item = &'a str>) -> impl quote::ToTokens {
    let idents = derives.map(|s| Ident::new(s, Span::call_site()));

    quote!(#[derive(#(#idents),*)])
}

fn render_variable_field_type(variable: Variable<'_>) -> TokenStream {
    let full_name = Ident::new(variable.type_name(), Span::call_site());

    let mut qualified = quote!(#full_name);

    let mut non_null = false;

    // Note: we iterate over qualifiers in reverse because it is more intuitive. This
    // means we start from the _inner_ type and make our way to the outside.
    for qualifier in variable.type_qualifiers().iter().rev() {
        match (non_null, qualifier) {
            // We are in non-null context, and we wrap the non-null type into a list.
            // We switch back to null context.
            (true, GraphqlTypeQualifier::List) => {
                qualified = quote!(Vec<#qualified>);
                non_null = false;
            }
            // We are in nullable context, and we wrap the nullable type into a list.
            (false, GraphqlTypeQualifier::List) => {
                qualified = quote!(Vec<Option<#qualified>>);
            }
            // We are in non-nullable context, but we can't double require a type
            // (!!).
            (true, GraphqlTypeQualifier::Required) => panic!("double required annotation"),
            // We are in nullable context, and we switch to non-nullable context.
            (false, GraphqlTypeQualifier::Required) => {
                non_null = true;
            }
        }
    }

    // If we are in nullable context at the end of the iteration, we wrap the whole
    // type with an Option.
    if !non_null {
        qualified = quote!(Option<#qualified>);
    }

    qualified
}
