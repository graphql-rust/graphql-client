mod enums;
mod inputs;
mod selection;
mod shared;

use crate::{
    query::{
        all_used_types, operation_has_no_variables, walk_operation_variables, BoundQuery,
        OperationId, ResolvedVariable, UsedTypes,
    },
    schema::{InputId, TypeId},
    type_qualifiers::GraphqlTypeQualifier,
    GraphQLClientCodegenOptions,
};
use heck::ToSnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use selection::render_response_data_fields;
use std::collections::BTreeMap;

/// The main code generation function.
pub(crate) fn response_for_query(
    operation_id: OperationId,
    options: &GraphQLClientCodegenOptions,
    query: BoundQuery<'_>,
) -> TokenStream {
    let serde = options.serde_path();

    let all_used_types = all_used_types(operation_id, &query);
    let response_derives = render_derives(options.all_response_derives());
    let variable_derives = render_derives(options.all_variable_derives());

    let scalar_definitions = generate_scalar_definitions(&all_used_types, options, query);
    let enum_definitions = enums::generate_enum_definitions(&all_used_types, options, query);
    let fragment_definitions =
        generate_fragment_definitions(&all_used_types, &response_derives, options, &query);
    let input_object_definitions = inputs::generate_input_object_definitions(
        &all_used_types,
        options,
        &variable_derives,
        &query,
    );

    let variables_struct =
        generate_variables_struct(operation_id, &variable_derives, options, &query);

    let definitions =
        render_response_data_fields(operation_id, options, &query).render(&response_derives);

    quote! {
        use #serde::{Serialize, Deserialize};
        use super::*;

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

        #(#input_object_definitions)*

        #variables_struct

        #(#fragment_definitions)*

        #definitions
    }
}

fn generate_variables_struct(
    operation_id: OperationId,
    variable_derives: &impl quote::ToTokens,
    options: &GraphQLClientCodegenOptions,
    query: &BoundQuery<'_>,
) -> TokenStream {
    let serde = options.serde_path();
    let serde_path = serde.to_token_stream().to_string();

    if operation_has_no_variables(operation_id, query.query) {
        return quote!(
            #variable_derives
            #[serde(crate = #serde_path)]
            pub struct Variables;
        );
    }

    let variable_fields = walk_operation_variables(operation_id, query.query)
        .map(|(_id, variable)| generate_variable_struct_field(variable, options, query));
    let variable_defaults =
        walk_operation_variables(operation_id, query.query).map(|(_id, variable)| {
            let method_name = format!("default_{}", variable.name);
            let method_name = Ident::new(&method_name, Span::call_site());
            let method_return_type = render_variable_field_type(variable, options, query);

            variable.default.as_ref().map(|default| {
                let value = graphql_parser_value_to_literal(
                    default,
                    variable.r#type.id,
                    variable
                        .r#type
                        .qualifiers
                        .first()
                        .map(|qual| !qual.is_required())
                        .unwrap_or(true),
                    query,
                );

                quote!(
                    pub fn #method_name() -> #method_return_type {
                        #value
                    }
                )
            })
        });

    let variables_struct = quote!(
        #variable_derives
        #[serde(crate = #serde_path)]
        pub struct Variables {
            #(#variable_fields,)*
        }

        impl Variables {
            #(#variable_defaults)*
        }
    );

    variables_struct
}

fn generate_variable_struct_field(
    variable: &ResolvedVariable,
    options: &GraphQLClientCodegenOptions,
    query: &BoundQuery<'_>,
) -> TokenStream {
    let snake_case_name = variable.name.to_snake_case();
    let safe_name = shared::keyword_replace(&snake_case_name);
    let ident = Ident::new(&safe_name, Span::call_site());
    let rename_annotation = shared::field_rename_annotation(&variable.name, &safe_name);
    let skip_serializing_annotation = if *options.skip_serializing_none() {
        if variable.r#type.qualifiers.first() == Some(&GraphqlTypeQualifier::Required) {
            None
        } else {
            Some(quote!(#[serde(skip_serializing_if = "Option::is_none")]))
        }
    } else {
        None
    };
    let r#type = render_variable_field_type(variable, options, query);

    quote::quote!(#skip_serializing_annotation #rename_annotation pub #ident : #r#type)
}

fn generate_scalar_definitions<'a, 'schema: 'a>(
    all_used_types: &'a crate::query::UsedTypes,
    options: &'a GraphQLClientCodegenOptions,
    query: BoundQuery<'schema>,
) -> impl Iterator<Item = TokenStream> + 'a {
    all_used_types
        .scalars(query.schema)
        .map(move |(_id, scalar)| {
            let ident = syn::Ident::new(
                options.normalization().scalar_name(&scalar.name).as_ref(),
                proc_macro2::Span::call_site(),
            );

            if let Some(custom_scalars_module) = options.custom_scalars_module() {
                quote!(type #ident = #custom_scalars_module::#ident;)
            } else {
                quote!(type #ident = super::#ident;)
            }
        })
}

fn render_derives<'a>(derives: impl Iterator<Item = &'a str>) -> impl quote::ToTokens {
    let idents = derives.map(|s| {
        syn::parse_str::<syn::Path>(s)
            .map_err(|e| format!("couldn't parse {s} as a derive Path: {e}"))
            .unwrap()
    });

    quote!(#[derive(#(#idents),*)])
}

fn render_variable_field_type(
    variable: &ResolvedVariable,
    options: &GraphQLClientCodegenOptions,
    query: &BoundQuery<'_>,
) -> TokenStream {
    let normalized_name = options
        .normalization()
        .input_name(variable.type_name(query.schema));
    let safe_name = shared::keyword_replace(normalized_name.clone());
    let full_name = Ident::new(safe_name.as_ref(), Span::call_site());

    decorate_type(&full_name, &variable.r#type.qualifiers)
}

fn decorate_type(ident: &Ident, qualifiers: &[GraphqlTypeQualifier]) -> TokenStream {
    let mut qualified = quote!(#ident);

    let mut non_null = false;

    // Note: we iterate over qualifiers in reverse because it is more intuitive. This
    // means we start from the _inner_ type and make our way to the outside.
    for qualifier in qualifiers.iter().rev() {
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

fn generate_fragment_definitions<'a>(
    all_used_types: &'a UsedTypes,
    response_derives: &'a impl quote::ToTokens,
    options: &'a GraphQLClientCodegenOptions,
    query: &'a BoundQuery<'a>,
) -> impl Iterator<Item = TokenStream> + 'a {
    all_used_types.fragment_ids().map(move |fragment_id| {
        selection::render_fragment(fragment_id, options, query).render(&response_derives)
    })
}

/// For default value constructors.
fn graphql_parser_value_to_literal<'doc, T>(
    value: &graphql_parser::query::Value<'doc, T>,
    ty: TypeId,
    is_optional: bool,
    query: &BoundQuery<'_>,
) -> TokenStream
where
    T: graphql_parser::query::Text<'doc>,
    T::Value: quote::ToTokens,
{
    use graphql_parser::query::Value;

    let inner = match value {
        Value::Boolean(b) => {
            if *b {
                quote!(true)
            } else {
                quote!(false)
            }
        }
        Value::String(s) => quote!(#s.to_string()),
        Value::Variable(_) => panic!("variable in variable"),
        Value::Null => panic!("null as default value"),
        Value::Float(f) => quote!(#f),
        Value::Int(i) => {
            let i = i.as_i64();
            quote!(#i)
        }
        Value::Enum(en) => quote!(#en),
        Value::List(inner) => {
            let elements = inner
                .iter()
                .map(|val| graphql_parser_value_to_literal(val, ty, false, query));
            quote! {
                vec![
                    #(#elements,)*
                ]
            }
        }
        Value::Object(obj) => ty
            .as_input_id()
            .map(|input_id| render_object_literal(obj, input_id, query))
            .unwrap_or_else(|| {
                quote!(compile_error!(
                    "Object literal on a non-input-object field."
                ))
            }),
    };

    if is_optional {
        quote!(Some(#inner))
    } else {
        inner
    }
}

/// For default value constructors.
fn render_object_literal<'doc, T>(
    object_map: &BTreeMap<T::Value, graphql_parser::query::Value<'doc, T>>,
    input_id: InputId,
    query: &BoundQuery<'_>,
) -> TokenStream
where
    T: graphql_parser::query::Text<'doc>,
    T::Value: quote::ToTokens,
{
    let input = query.schema.get_input(input_id);
    let constructor = Ident::new(&input.name, Span::call_site());
    let fields: Vec<TokenStream> = input
        .fields
        .iter()
        .map(|(name, r#type)| {
            let field_name = Ident::new(name, Span::call_site());
            let provided_value = object_map.get(name);
            if let Some(default_value) = provided_value {
                let value = graphql_parser_value_to_literal(
                    default_value,
                    r#type.id,
                    r#type.is_optional(),
                    query,
                );
                quote!(#field_name: #value)
            } else {
                quote!(#field_name: None)
            }
        })
        .collect();

    quote!(#constructor {
        #(#fields,)*
    })
}
