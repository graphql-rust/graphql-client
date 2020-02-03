// use crate::fragments::GqlFragment;
use crate::normalization::Normalization;
// use crate::operations::Operation;
// use crate::query::QueryContext;
use crate::schema;
// use crate::selection::Selection;
use crate::resolution::{Operation, ResolvedQuery};
use graphql_parser::query;
use proc_macro2::TokenStream;
use quote::*;

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
    options: &crate::GraphQLClientCodegenOptions,
) -> anyhow::Result<TokenStream> {
    let all_used_types = operation.all_used_types();
    let scalar_definitions = generate_scalar_definitions(operation, &all_used_types);
    let enum_definitions = generate_enum_definitions(operation, &all_used_types);
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

fn generate_enum_definitions<'a, 'schema: 'a>(
    operation: Operation<'schema>,
    all_used_types: &'a crate::resolution::UsedTypes,
) -> impl Iterator<Item = TokenStream> + 'a {
    all_used_types.enums(operation.schema()).map(|r#enum| {
        let ident = syn::Ident::new(r#enum.name(), proc_macro2::Span::call_site());

        todo!()
    })
}
