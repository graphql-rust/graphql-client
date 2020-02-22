use crate::{
    deprecation::DeprecationStrategy,
    field_type::GraphqlTypeQualifier,
    normalization::Normalization,
    resolution::*,
    schema::{FieldRef, TypeId},
    shared::{field_rename_annotation, keyword_replace},
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
    operation: WithQuery<'_, OperationId>,
    options: &GraphQLClientCodegenOptions,
) -> anyhow::Result<TokenStream> {
    let all_used_types = operation.all_used_types();
    let response_derives = render_derives(options.all_response_derives());

    let scalar_definitions = generate_scalar_definitions(operation, &all_used_types);
    let enum_definitions = generate_enum_definitions(operation, &all_used_types, options);
    let (fragment_definitions, fragment_nested_definitions) =
        generate_fragment_definitions(operation, &all_used_types, &response_derives, options);
    let input_object_definitions =
        generate_input_object_definitions(operation, &all_used_types, options);
    let variables_struct = generate_variables_struct(operation, options);

    let (definitions, response_data_fields) =
        render_response_data_fields(&operation, &response_derives, options);

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

        #(#fragment_nested_definitions)*

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
    operation: WithQuery<'_, OperationId>,
    options: &GraphQLClientCodegenOptions,
) -> TokenStream {
    let variable_derives = options.all_variable_derives();
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

fn generate_variable_struct_field(variable: WithQuery<'_, VariableId>) -> TokenStream {
    let snake_case_name = variable.name().to_snake_case();
    let ident = Ident::new(
        &crate::shared::keyword_replace(&snake_case_name),
        Span::call_site(),
    );
    let annotation = crate::shared::field_rename_annotation(variable.name(), &snake_case_name);
    let r#type = render_variable_field_type(variable);

    quote::quote!(#annotation pub #ident : #r#type)
}

fn generate_scalar_definitions<'a, 'schema: 'a>(
    operation: WithQuery<'schema, OperationId>,
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
    operation: WithQuery<'schema, OperationId>,
    all_used_types: &'a crate::resolution::UsedTypes,
    options: &'a GraphQLClientCodegenOptions,
) -> impl Iterator<Item = TokenStream> + 'a {
    let derives = render_derives(
        options
            .all_response_derives()
            .filter(|d| !&["Serialize", "Deserialize"].contains(d)),
    );
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

fn render_variable_field_type(variable: WithQuery<'_, VariableId>) -> TokenStream {
    let full_name = Ident::new(variable.type_name(), Span::call_site());

    decorate_type(&full_name, variable.type_qualifiers())
}

fn render_response_data_fields<'a>(
    operation: &OperationRef<'a>,
    response_derives: &impl quote::ToTokens,
    options: &GraphQLClientCodegenOptions,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut response_types = Vec::new();
    let mut fields = Vec::new();

    render_selection(
        operation.selection(),
        &mut fields,
        &mut response_types,
        response_derives,
        options,
    );

    (response_types, fields)
}

fn render_selection<'a>(
    selection: impl Iterator<Item = WithQuery<'a, SelectionId>>,
    field_buffer: &mut Vec<TokenStream>,
    response_type_buffer: &mut Vec<TokenStream>,
    response_derives: &impl quote::ToTokens,
    options: &GraphQLClientCodegenOptions,
) {
    // TODO: if the selection has one item, we can sometimes generate fewer structs (e.g. single fragment spread)

    for select in selection {
        // TODO: handle deprecation here

        // match (options.deprecation_strategy(), select.on().is_deprecated()) {
        //     DeprecationStrategy::Warn => todo!(),
        //     DeprecationStrategy::Allow => (),
        //     DeprecationStrategy::Deny => continue,
        // }

        match &select.get() {
            Selection::Field(field) => {
                let field = select.refocus(field);

                match (
                    field.schema_field().is_deprecated(),
                    options.deprecation_strategy(),
                ) {
                    (false, _) | (true, DeprecationStrategy::Allow) => (),
                    (true, DeprecationStrategy::Warn) => todo!("deprecation annotation"),
                    (true, DeprecationStrategy::Deny) => continue,
                }

                let ident = field_name(&field);
                match field.schema_field().field_type().item {
                    TypeId::Enum(enm) => {
                        let type_name =
                            Ident::new(field.with_schema(enm).name(), Span::call_site());
                        let type_name =
                            decorate_type(&type_name, field.schema_field().type_qualifiers());

                        field_buffer.push(quote!(#ident: #type_name));
                    }
                    TypeId::Scalar(scalar) => {
                        let type_name =
                            Ident::new(field.with_schema(scalar).name(), Span::call_site());
                        let type_name =
                            decorate_type(&type_name, field.schema_field().type_qualifiers());

                        field_buffer.push(quote!(#ident: #type_name));
                    }
                    TypeId::Object(_) => {
                        let struct_name = Ident::new(&select.full_path_prefix(), Span::call_site());
                        let field_type =
                            decorate_type(&struct_name, field.schema_field().type_qualifiers());

                        field_buffer.push(quote!(#ident: #field_type));

                        let mut fields = Vec::new();
                        render_selection(
                            select.subselection(),
                            &mut fields,
                            response_type_buffer,
                            response_derives,
                            options,
                        );

                        let struct_definition = quote! {
                            #response_derives
                            pub struct #struct_name {
                                #(#fields),*
                            }
                        };

                        response_type_buffer.push(struct_definition);
                    }
                    _other => {
                        Ident::new("String", Span::call_site());
                        // unimplemented!("selection on {:?}", other)
                    }
                };
            }
            Selection::Typename => {
                field_buffer.push(quote!(
                    #[serde(rename = "__typename")]
                    pub typename: String
                ));
            }
            Selection::InlineFragment(inline) => todo!("render inline fragment"),
            Selection::FragmentSpread(frag) => {
                let frag = select.refocus(*frag);
                let original_field_name = frag.name().to_snake_case();
                let final_field_name = keyword_replace(&original_field_name);
                let annotation = field_rename_annotation(&original_field_name, &final_field_name);
                let field_ident = Ident::new(&final_field_name, Span::call_site());
                let type_name = Ident::new(frag.name(), Span::call_site());
                field_buffer.push(quote! {
                    #[serde(flatten)]
                    pub #annotation #field_ident: #type_name
                });
            }
        }
    }
}

fn field_name(field: &WithQuery<'_, &SelectedField>) -> impl quote::ToTokens {
    let name = field
        .alias()
        .unwrap_or_else(|| field.name())
        .to_snake_case();
    let final_name = keyword_replace(&name);
    let rename_annotation = field_rename_annotation(&name, &final_name);

    let ident = Ident::new(&final_name, Span::call_site());

    quote!(#rename_annotation pub #ident)
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

fn generate_input_object_definitions(
    operation: WithQuery<'_, OperationId>,
    all_used_types: &UsedTypes,
    options: &GraphQLClientCodegenOptions,
) -> Vec<TokenStream> {
    all_used_types
        .inputs(operation.schema())
        .map(|input| quote!(heh))
        .collect()
}

fn generate_fragment_definitions(
    operation: OperationRef<'_>,
    all_used_types: &UsedTypes,
    response_derives: &impl quote::ToTokens,
    options: &GraphQLClientCodegenOptions,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut response_type_buffer = Vec::new();
    let mut fragment_definitions = Vec::with_capacity(all_used_types.fragments_len());

    let fragments = all_used_types
        .fragment_ids()
        .map(move |id| operation.refocus(id));

    for fragment in fragments {
        let struct_name = fragment.name();
        let struct_ident = Ident::new(struct_name, Span::call_site());
        let mut fields = Vec::with_capacity(fragment.selection_set_len());

        render_selection(
            fragment.selection(),
            &mut fields,
            &mut response_type_buffer,
            response_derives,
            options,
        );

        let fragment_definition = quote! {
            #response_derives
            pub struct #struct_ident {
                #(#fields),*
            }
        };

        fragment_definitions.push(fragment_definition.into())
    }

    (fragment_definitions, response_type_buffer)
}
