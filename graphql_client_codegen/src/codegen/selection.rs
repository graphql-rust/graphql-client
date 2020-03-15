//! Code generation for the selection on an operation or a fragment.

use crate::codegen::decorate_type;
use crate::resolution::FragmentRef;
use crate::resolution::ResolvedFragmentId;
use crate::resolution::SelectedField;
use crate::resolution::SelectionRef;
use crate::schema::TypeRef;
use crate::shared::field_rename_annotation;
use crate::{
    deprecation::DeprecationStrategy,
    field_type::GraphqlTypeQualifier,
    // deprecation::DeprecationStrategy,
    resolution::{InlineFragment, OperationRef, ResolvedQuery, Selection, SelectionId},
    schema::{Schema, TypeId},
    shared::keyword_replace,
    GraphQLClientCodegenOptions,
};
use heck::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::borrow::Cow;

pub(crate) fn render_response_data_fields<'a>(
    operation: &OperationRef<'a>,
    options: &'a GraphQLClientCodegenOptions,
) -> ExpandedSelection<'a> {
    let mut expanded_selection = ExpandedSelection {
        query: operation.query(),
        schema: operation.schema(),
        types: Vec::with_capacity(8),
        variants: Vec::new(),
        fields: Vec::with_capacity(operation.selection_ids().len()),
        options,
    };

    let response_data_type_id = expanded_selection.push_type(ExpandedType {
        name: Cow::Borrowed("ResponseData"),
        schema_type: operation.on_ref(),
    });

    calculate_selection(
        &mut expanded_selection,
        operation.selection_ids(),
        response_data_type_id,
        operation.on_ref(),
    );

    expanded_selection
}

pub(super) fn render_fragment<'a>(
    fragment: &FragmentRef<'a>,
    options: &'a GraphQLClientCodegenOptions,
) -> ExpandedSelection<'a> {
    let mut expanded_selection = ExpandedSelection {
        query: fragment.query(),
        schema: fragment.schema(),
        types: Vec::with_capacity(8),
        variants: Vec::new(),
        fields: Vec::with_capacity(fragment.selection_ids().len()),
        options,
    };

    let response_type_id = expanded_selection.push_type(ExpandedType {
        name: fragment.name().into(),
        schema_type: fragment.on_ref(),
    });

    calculate_selection(
        &mut expanded_selection,
        fragment.selection_ids(),
        response_type_id,
        fragment.on_ref(),
    );

    expanded_selection
}

/// A sub-selection set (spread) on one of the variants of a union or interface.
enum VariantSelection<'a> {
    InlineFragment(&'a InlineFragment),
    FragmentSpread(FragmentRef<'a>),
}

impl<'a> VariantSelection<'a> {
    /// The second argument is the parent type id, so it can be excluded.
    fn from_selection(
        selection_ref: &SelectionRef<'a>,
        type_id: TypeId,
    ) -> Option<VariantSelection<'a>> {
        match selection_ref.selection() {
            Selection::InlineFragment(inline_fragment) => {
                Some(VariantSelection::InlineFragment(inline_fragment))
            }
            Selection::FragmentSpread(fragment_id) => {
                let schema = selection_ref.schema();
                let fragment_ref = selection_ref.query().get_fragment_ref(schema, *fragment_id);

                if fragment_ref.on() == type_id {
                    // The selection is on the type itself.
                    None
                } else {
                    // The selection is on one of the variants of the type.
                    Some(VariantSelection::FragmentSpread(fragment_ref))
                }
            }
            Selection::Field(_) | Selection::Typename => None,
        }
    }

    fn variant_type_id(&self) -> TypeId {
        match self {
            VariantSelection::InlineFragment(f) => f.type_id,
            VariantSelection::FragmentSpread(f) => f.on(),
        }
    }
}

fn calculate_selection<'a>(
    context: &mut ExpandedSelection<'a>,
    selection_set: &[SelectionId],
    struct_id: ResponseTypeId,
    type_ref: TypeRef<'a>,
) {
    // If we are on a union or an interface, we need to generate an enum that matches the variants _exhaustively_.
    {
        let variants: Option<Cow<'_, [TypeId]>> = match type_ref.type_id() {
            TypeId::Interface(interface_id) => {
                let interface = context.schema().interface(interface_id);

                Some(interface.variants().collect())
            }
            TypeId::Union(union_id) => {
                let union = context.schema().union(union_id);
                Some(union.variants().into())
            }
            _ => None,
        };

        if let Some(variants) = variants {
            let variant_selections: Vec<(SelectionRef<'_>, VariantSelection<'_>)> = selection_set
                .iter()
                .map(|id| context.get_selection_ref(*id))
                .filter_map(|selection_ref| {
                    VariantSelection::from_selection(&selection_ref, type_ref.type_id())
                        .map(|variant_selection| (selection_ref, variant_selection))
                })
                .collect();

            // For each variant, get the corresponding fragment spreads and
            // inline fragments, or default to an empty variant (one with no
            // associated data).
            for variant in variants.as_ref() {
                let variant_schema_type = context.schema().type_ref(*variant);
                let variant_name_str = variant_schema_type.name();

                let mut variant_selections = variant_selections
                    .iter()
                    .filter(|(_selection_ref, variant)| {
                        variant.variant_type_id() == variant_schema_type.type_id()
                    })
                    .peekable();

                if let Some((selection_ref, _variant)) = variant_selections.peek() {
                    let mut variant_struct_name_str = selection_ref.full_path_prefix();
                    variant_struct_name_str.reserve(2 + variant_name_str.len());
                    variant_struct_name_str.push_str("On");
                    variant_struct_name_str.push_str(variant_name_str);

                    context.push_variant(ExpandedVariant {
                        name: variant_name_str.into(),
                        variant_type: Some(variant_struct_name_str.clone().into()),
                        on: struct_id,
                    });

                    let expanded_type = ExpandedType {
                        name: variant_struct_name_str.into(),
                        schema_type: variant_schema_type,
                    };

                    let struct_id = context.push_type(expanded_type);

                    for (_selection, variant_selection) in variant_selections {
                        match variant_selection {
                            VariantSelection::InlineFragment(_) => {
                                calculate_selection(
                                    context,
                                    selection_ref.subselection_ids(),
                                    struct_id,
                                    variant_schema_type,
                                );
                            }
                            VariantSelection::FragmentSpread(fragment_ref) => {
                                context.push_field(ExpandedField {
                                    field_type: fragment_ref.name().into(),
                                    field_type_qualifiers: &[GraphqlTypeQualifier::Required],
                                    flatten: true,
                                    graphql_name: None,
                                    rust_name: fragment_ref.name().to_snake_case().into(),
                                    struct_id,
                                    deprecation: None,
                                    boxed: fragment_ref.is_recursive(),
                                })
                            }
                        }
                    }
                } else {
                    context.push_variant(ExpandedVariant {
                        name: variant_name_str.into(),
                        on: struct_id,
                        variant_type: None,
                    });
                }
            }
        }
    }

    for id in selection_set {
        let selection_ref = context.get_selection_ref(*id);

        match selection_ref.selection() {
            Selection::Field(field) => {
                let (graphql_name, rust_name) = context.field_name(&field);
                let schema_field = field.schema_field(context.schema());
                let field_type = schema_field.field_type();

                match field_type.type_id() {
                    TypeId::Enum(enm) => {
                        context.push_field(ExpandedField {
                            graphql_name: Some(graphql_name),
                            rust_name,
                            struct_id,
                            field_type: context.schema().r#enum(enm).name().into(),
                            field_type_qualifiers: schema_field.type_qualifiers(),
                            flatten: false,
                            deprecation: schema_field.deprecation(),
                            boxed: false,
                        });
                    }
                    TypeId::Scalar(scalar) => {
                        context.push_field(ExpandedField {
                            field_type: context.schema().scalar(scalar).name().into(),
                            field_type_qualifiers: field
                                .schema_field(context.schema())
                                .type_qualifiers(),
                            graphql_name: Some(graphql_name),
                            struct_id,
                            rust_name,
                            flatten: false,
                            deprecation: schema_field.deprecation(),
                            boxed: false,
                        });
                    }
                    TypeId::Object(_) | TypeId::Interface(_) | TypeId::Union(_) => {
                        let struct_name_string = selection_ref.full_path_prefix();

                        context.push_field(ExpandedField {
                            struct_id,
                            graphql_name: Some(graphql_name),
                            rust_name,
                            field_type_qualifiers: schema_field.type_qualifiers(),
                            field_type: Cow::Owned(struct_name_string.clone()),
                            flatten: false,
                            boxed: false,
                            deprecation: schema_field.deprecation(),
                        });

                        let type_id = context.push_type(ExpandedType {
                            name: Cow::Owned(struct_name_string),
                            schema_type: field_type,
                        });

                        calculate_selection(
                            context,
                            selection_ref.subselection_ids(),
                            type_id,
                            field_type,
                        );
                    }
                    TypeId::Input(_) => unreachable!("field selection on input type"),
                };
            }
            Selection::Typename => (),
            Selection::InlineFragment(_inline) => (),
            Selection::FragmentSpread(fragment_id) => {
                // Here we only render fragments that are directly on the type
                // itself, and not on one of its variants.

                let fragment = context.get_fragment_ref(*fragment_id);

                // Assuming the query was validated properly, a fragment spread
                // is either on the field's type itself, or on one of the
                // variants (union or interfaces). If it's not directly a field
                // on the struct, it will be handled in the `on` variants.
                if fragment.on() != type_ref.type_id() {
                    continue;
                }

                let original_field_name = fragment.name().to_snake_case();
                let final_field_name = keyword_replace(original_field_name);

                context.push_field(ExpandedField {
                    field_type: fragment.name().into(),
                    field_type_qualifiers: &[GraphqlTypeQualifier::Required],
                    graphql_name: None,
                    rust_name: final_field_name,
                    struct_id,
                    flatten: true,
                    deprecation: None,
                    boxed: fragment.is_recursive(),
                });

                // We stop here, because the structs for the fragments are generated separately, to
                // avoid duplication.
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct ResponseTypeId(u32);

struct ExpandedField<'a> {
    graphql_name: Option<&'a str>,
    rust_name: Cow<'a, str>,
    field_type: Cow<'a, str>,
    field_type_qualifiers: &'a [GraphqlTypeQualifier],
    struct_id: ResponseTypeId,
    flatten: bool,
    deprecation: Option<Option<&'a str>>,
    boxed: bool,
}

impl<'a> ExpandedField<'a> {
    fn render(&self, options: &GraphQLClientCodegenOptions) -> Option<TokenStream> {
        let ident = Ident::new(&self.rust_name, Span::call_site());
        let qualified_type = decorate_type(
            &Ident::new(&self.field_type, Span::call_site()),
            self.field_type_qualifiers,
        );

        let qualified_type = if self.boxed {
            quote!(Box<#qualified_type>)
        } else {
            qualified_type
        };

        let optional_rename = self
            .graphql_name
            .as_ref()
            .map(|graphql_name| field_rename_annotation(graphql_name, &self.rust_name));
        let optional_flatten = if self.flatten {
            Some(quote!(#[serde(flatten)]))
        } else {
            None
        };

        let optional_deprecation_annotation =
            match (self.deprecation, options.deprecation_strategy()) {
                (None, _) | (Some(_), DeprecationStrategy::Allow) => None,
                (Some(msg), DeprecationStrategy::Warn) => {
                    let optional_msg = msg.map(|msg| quote!((note = #msg)));

                    Some(quote!(#[deprecated#optional_msg]))
                }
                (Some(_), DeprecationStrategy::Deny) => return None,
            };

        let tokens = quote! {
            #optional_flatten
            #optional_rename
            #optional_deprecation_annotation
            pub #ident: #qualified_type
        };

        Some(tokens)
    }
}

struct ExpandedVariant<'a> {
    name: Cow<'a, str>,
    variant_type: Option<Cow<'a, str>>,
    on: ResponseTypeId,
}

impl<'a> ExpandedVariant<'a> {
    fn render(&self) -> TokenStream {
        let name_ident = Ident::new(&self.name, Span::call_site());
        let optional_type_ident = self.variant_type.as_ref().map(|variant_type| {
            let ident = Ident::new(&variant_type, Span::call_site());
            quote!((#ident))
        });

        quote!(#name_ident #optional_type_ident)
    }
}

pub(crate) struct ExpandedType<'a> {
    name: Cow<'a, str>,
    schema_type: TypeRef<'a>,
}

pub(crate) struct ExpandedSelection<'a> {
    query: &'a ResolvedQuery,
    schema: &'a Schema,
    types: Vec<ExpandedType<'a>>,
    fields: Vec<ExpandedField<'a>>,
    variants: Vec<ExpandedVariant<'a>>,
    options: &'a GraphQLClientCodegenOptions,
}

impl<'a> ExpandedSelection<'a> {
    pub(crate) fn schema(&self) -> &'a Schema {
        self.schema
    }

    fn push_type(&mut self, tpe: ExpandedType<'a>) -> ResponseTypeId {
        let id = self.types.len();
        self.types.push(tpe);

        ResponseTypeId(id as u32)
    }

    fn push_field(&mut self, field: ExpandedField<'a>) {
        self.fields.push(field);
    }

    fn push_variant(&mut self, variant: ExpandedVariant<'a>) {
        self.variants.push(variant);
    }

    pub(crate) fn get_selection_ref(&self, selection_id: SelectionId) -> SelectionRef<'a> {
        self.query.get_selection_ref(self.schema, selection_id)
    }

    pub(crate) fn get_fragment_ref(&self, fragment_id: ResolvedFragmentId) -> FragmentRef<'a> {
        self.query.get_fragment_ref(self.schema, fragment_id)
    }

    /// Returns a tuple to be interpreted as (graphql_name, rust_name).
    pub(crate) fn field_name(&self, field: &'a SelectedField) -> (&'a str, Cow<'a, str>) {
        let name = field
            .alias()
            .unwrap_or_else(|| field.schema_field(self.schema).name());
        let snake_case_name = name.to_snake_case();
        let final_name = keyword_replace(snake_case_name);

        (name, final_name)
    }

    fn types(&self) -> impl Iterator<Item = (ResponseTypeId, &ExpandedType<'_>)> {
        self.types
            .iter()
            .enumerate()
            .map(|(idx, ty)| (ResponseTypeId(idx as u32), ty))
    }

    pub fn render(&self, response_derives: &impl quote::ToTokens) -> TokenStream {
        let mut items = Vec::with_capacity(self.types.len());

        for (type_id, ty) in self.types() {
            let struct_name = Ident::new(&ty.name, Span::call_site());
            let mut fields = self
                .fields
                .iter()
                .filter(|field| field.struct_id == type_id)
                .filter_map(|field| field.render(self.options))
                .peekable();

            let on_variants: Vec<TokenStream> = self
                .variants
                .iter()
                .filter(|variant| variant.on == type_id)
                .map(|variant| variant.render())
                .collect();

            // If we only have an `on` field, turn the struct into the enum
            // of the variants.
            if fields.peek().is_none() {
                let item = quote! {
                    #response_derives
                    #[serde(tag = "__typename")]
                    pub enum #struct_name {
                        #(#on_variants),*
                    }
                };
                items.push(item);
                continue;
            }

            let (on_field, on_enum) = if on_variants.len() > 0 {
                let enum_name = Ident::new(&format!("{}On", ty.name), Span::call_site());

                let on_field = quote!(#[serde(flatten)] pub on: #enum_name);

                let on_enum = quote!(
                    #response_derives
                    #[serde(tag = "__typename")]
                    pub enum #enum_name {
                        #(#on_variants,)*
                    }
                );

                (Some(on_field), Some(on_enum))
            } else {
                (None, None)
            };

            let tokens = quote! {
                #response_derives
                pub struct #struct_name {
                    #(#fields,)*
                    #on_field
                }

                #on_enum
            };

            items.push(tokens);
        }

        quote!(#(#items)*)
    }
}
