//! Code generation for the selection on an operation or a fragment.

use crate::{
    codegen::{
        decorate_type,
        shared::{field_rename_annotation, keyword_replace},
    },
    deprecation::DeprecationStrategy,
    query::{
        fragment_is_recursive, full_path_prefix, BoundQuery, InlineFragment, OperationId,
        ResolvedFragment, ResolvedFragmentId, SelectedField, Selection, SelectionId,
    },
    schema::{Schema, TypeId},
    type_qualifiers::GraphqlTypeQualifier,
    GraphQLClientCodegenOptions,
};
use heck::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use std::borrow::Cow;
use syn::Path;

pub(crate) fn render_response_data_fields<'a>(
    operation_id: OperationId,
    options: &'a GraphQLClientCodegenOptions,
    query: &'a BoundQuery<'a>,
) -> ExpandedSelection<'a> {
    let operation = query.query.get_operation(operation_id);
    let mut expanded_selection = ExpandedSelection {
        query,
        types: Vec::with_capacity(8),
        aliases: Vec::new(),
        variants: Vec::new(),
        fields: Vec::with_capacity(operation.selection_set.len()),
        options,
    };

    let response_data_type_id = expanded_selection.push_type(ExpandedType {
        name: Cow::Borrowed("ResponseData"),
    });

    calculate_selection(
        &mut expanded_selection,
        &operation.selection_set,
        response_data_type_id,
        TypeId::Object(operation.object_id),
        options,
    );

    expanded_selection
}

pub(super) fn render_fragment<'a>(
    fragment_id: ResolvedFragmentId,
    options: &'a GraphQLClientCodegenOptions,
    query: &'a BoundQuery<'a>,
) -> ExpandedSelection<'a> {
    let fragment = query.query.get_fragment(fragment_id);
    let mut expanded_selection = ExpandedSelection {
        query,
        aliases: Vec::new(),
        types: Vec::with_capacity(8),
        variants: Vec::new(),
        fields: Vec::with_capacity(fragment.selection_set.len()),
        options,
    };

    let response_type_id = expanded_selection.push_type(ExpandedType {
        name: fragment.name.as_str().into(),
    });

    calculate_selection(
        &mut expanded_selection,
        &fragment.selection_set,
        response_type_id,
        fragment.on,
        options,
    );

    expanded_selection
}

/// A sub-selection set (spread) on one of the variants of a union or interface.
enum VariantSelection<'a> {
    InlineFragment(&'a InlineFragment),
    FragmentSpread((ResolvedFragmentId, &'a ResolvedFragment)),
}

impl<'a> VariantSelection<'a> {
    /// The second argument is the parent type id, so it can be excluded.
    fn from_selection(
        selection: &'a Selection,
        type_id: TypeId,
        query: &BoundQuery<'a>,
    ) -> Option<VariantSelection<'a>> {
        match selection {
            Selection::InlineFragment(inline_fragment) => {
                Some(VariantSelection::InlineFragment(inline_fragment))
            }
            Selection::FragmentSpread(fragment_id) => {
                let fragment = query.query.get_fragment(*fragment_id);

                if fragment.on == type_id {
                    // The selection is on the type itself.
                    None
                } else {
                    // The selection is on one of the variants of the type.
                    Some(VariantSelection::FragmentSpread((*fragment_id, fragment)))
                }
            }
            Selection::Field(_) | Selection::Typename => None,
        }
    }

    fn variant_type_id(&self) -> TypeId {
        match self {
            VariantSelection::InlineFragment(f) => f.type_id,
            VariantSelection::FragmentSpread((_id, f)) => f.on,
        }
    }
}

fn calculate_selection<'a>(
    context: &mut ExpandedSelection<'a>,
    selection_set: &[SelectionId],
    struct_id: ResponseTypeId,
    type_id: TypeId,
    options: &'a GraphQLClientCodegenOptions,
) {
    // If the selection only contains a fragment, replace the selection with
    // that fragment.
    if selection_set.len() == 1 {
        if let Selection::FragmentSpread(fragment_id) =
            context.query.query.get_selection(selection_set[0])
        {
            let fragment = context.query.query.get_fragment(*fragment_id);
            context.push_type_alias(TypeAlias {
                name: &fragment.name,
                struct_id,
                boxed: fragment_is_recursive(*fragment_id, context.query.query),
            });
            return;
        }

        if let Some(custom_response_type) = options.custom_response_type() {
            context.push_type_alias(TypeAlias {
                name: custom_response_type.as_str(),
                struct_id,
                boxed: false,
            });
            return;
        }
    }

    // If we are on a union or an interface, we need to generate an enum that matches the variants _exhaustively_.
    {
        let variants: Option<Cow<'_, [TypeId]>> = match type_id {
            TypeId::Interface(interface_id) => {
                let variants = context
                    .query
                    .schema
                    .objects()
                    .filter(|(_, obj)| obj.implements_interfaces.contains(&interface_id))
                    .map(|(id, _)| TypeId::Object(id));

                Some(variants.collect::<Vec<TypeId>>().into())
            }
            TypeId::Union(union_id) => {
                let union = context.schema().get_union(union_id);
                Some(union.variants.as_slice().into())
            }
            _ => None,
        };

        if let Some(variants) = variants {
            let variant_selections: Vec<(SelectionId, &Selection, VariantSelection<'_>)> =
                selection_set
                    .iter()
                    .map(|id| (id, context.query.query.get_selection(*id)))
                    .filter_map(|(id, selection)| {
                        VariantSelection::from_selection(selection, type_id, context.query)
                            .map(|variant_selection| (*id, selection, variant_selection))
                    })
                    .collect();

            // For each variant, get the corresponding fragment spreads and
            // inline fragments, or default to an empty variant (one with no
            // associated data).
            for variant_type_id in variants.as_ref() {
                let variant_name_str = variant_type_id.name(context.schema());

                let variant_selections: Vec<_> = variant_selections
                    .iter()
                    .filter(|(_id, _selection_ref, variant)| {
                        variant.variant_type_id() == *variant_type_id
                    })
                    .collect();

                if let Some((selection_id, selection, _variant)) = variant_selections.first() {
                    let mut variant_struct_name_str =
                        full_path_prefix(*selection_id, context.query);
                    variant_struct_name_str.reserve(2 + variant_name_str.len());
                    variant_struct_name_str.push_str("On");
                    variant_struct_name_str.push_str(variant_name_str);

                    context.push_variant(ExpandedVariant {
                        name: variant_name_str.into(),
                        variant_type: Some(variant_struct_name_str.clone().into()),
                        on: struct_id,
                        is_default_variant: false,
                    });

                    let expanded_type = ExpandedType {
                        name: variant_struct_name_str.into(),
                    };

                    let struct_id = context.push_type(expanded_type);

                    if variant_selections.len() == 1 {
                        if let VariantSelection::FragmentSpread((fragment_id, fragment)) =
                            variant_selections[0].2
                        {
                            context.push_type_alias(TypeAlias {
                                boxed: fragment_is_recursive(fragment_id, context.query.query),
                                name: &fragment.name,
                                struct_id,
                            });
                            continue;
                        }
                    }

                    for (_selection_id, _selection, variant_selection) in variant_selections {
                        match variant_selection {
                            VariantSelection::InlineFragment(_) => {
                                calculate_selection(
                                    context,
                                    selection.subselection(),
                                    struct_id,
                                    *variant_type_id,
                                    options,
                                );
                            }
                            VariantSelection::FragmentSpread((fragment_id, fragment)) => context
                                .push_field(ExpandedField {
                                    field_type: fragment.name.as_str().into(),
                                    field_type_qualifiers: &[GraphqlTypeQualifier::Required],
                                    flatten: true,
                                    graphql_name: None,
                                    rust_name: fragment.name.to_snake_case().into(),
                                    struct_id,
                                    deprecation: None,
                                    boxed: fragment_is_recursive(*fragment_id, context.query.query),
                                }),
                        }
                    }
                } else {
                    context.push_variant(ExpandedVariant {
                        name: variant_name_str.into(),
                        on: struct_id,
                        variant_type: None,
                        is_default_variant: false,
                    });
                }
            }

            if *options.fragments_other_variant() {
                context.push_variant(ExpandedVariant {
                    name: "Unknown".into(),
                    on: struct_id,
                    variant_type: None,
                    is_default_variant: true,
                });
            }
        }
    }

    for id in selection_set {
        let selection = context.query.query.get_selection(*id);

        match selection {
            Selection::Field(field) => {
                let (graphql_name, rust_name) = context.field_name(field);
                let schema_field = field.schema_field(context.schema());
                let field_type_id = schema_field.r#type.id;

                match field_type_id {
                    TypeId::Enum(enm) => {
                        context.push_field(ExpandedField {
                            graphql_name: Some(graphql_name),
                            rust_name,
                            struct_id,
                            field_type: options
                                .normalization()
                                .field_type(&context.schema().get_enum(enm).name),
                            field_type_qualifiers: &schema_field.r#type.qualifiers,
                            flatten: false,
                            deprecation: schema_field.deprecation(),
                            boxed: false,
                        });
                    }
                    TypeId::Scalar(scalar) => {
                        context.push_field(ExpandedField {
                            field_type: options
                                .normalization()
                                .field_type(context.schema().get_scalar(scalar).name.as_str()),
                            field_type_qualifiers: &field
                                .schema_field(context.schema())
                                .r#type
                                .qualifiers,
                            graphql_name: Some(graphql_name),
                            struct_id,
                            rust_name,
                            flatten: false,
                            deprecation: schema_field.deprecation(),
                            boxed: false,
                        });
                    }
                    TypeId::Object(_) | TypeId::Interface(_) | TypeId::Union(_) => {
                        let struct_name_string = full_path_prefix(*id, context.query);

                        context.push_field(ExpandedField {
                            struct_id,
                            graphql_name: Some(graphql_name),
                            rust_name,
                            field_type_qualifiers: &schema_field.r#type.qualifiers,
                            field_type: Cow::Owned(struct_name_string.clone()),
                            flatten: false,
                            boxed: false,
                            deprecation: schema_field.deprecation(),
                        });

                        let type_id = context.push_type(ExpandedType {
                            name: Cow::Owned(struct_name_string),
                        });

                        calculate_selection(
                            context,
                            selection.subselection(),
                            type_id,
                            field_type_id,
                            options,
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

                let fragment = context.query.query.get_fragment(*fragment_id);

                // Assuming the query was validated properly, a fragment spread
                // is either on the field's type itself, or on one of the
                // variants (union or interfaces). If it's not directly a field
                // on the struct, it will be handled in the `on` variants.
                if fragment.on != type_id {
                    continue;
                }

                let original_field_name = fragment.name.to_snake_case();
                let final_field_name = keyword_replace(original_field_name);

                context.push_field(ExpandedField {
                    field_type: fragment.name.as_str().into(),
                    field_type_qualifiers: &[GraphqlTypeQualifier::Required],
                    graphql_name: None,
                    rust_name: final_field_name,
                    struct_id,
                    flatten: true,
                    deprecation: None,
                    boxed: fragment_is_recursive(*fragment_id, context.query.query),
                });

                // We stop here, because the structs for the fragments are generated separately, to
                // avoid duplication.
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct ResponseTypeId(u32);

struct TypeAlias<'a> {
    name: &'a str,
    struct_id: ResponseTypeId,
    boxed: bool,
}

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

impl ExpandedField<'_> {
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

        let is_id = self.field_type == "ID";
        let is_required = self
            .field_type_qualifiers
            .contains(&GraphqlTypeQualifier::Required);
        let id_deserialize_with = if is_id && is_required {
            Some(quote!(#[serde(deserialize_with = "graphql_client::serde_with::deserialize_id")]))
        } else if is_id {
            Some(
                quote!(#[serde(deserialize_with = "graphql_client::serde_with::deserialize_option_id")]),
            )
        } else {
            None
        };

        let optional_skip_serializing_none = if *options.skip_serializing_none()
            && self
                .field_type_qualifiers
                .first()
                .map(|qualifier| !qualifier.is_required())
                .unwrap_or(false)
        {
            Some(quote!(#[serde(skip_serializing_if = "Option::is_none")]))
        } else {
            None
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

                    Some(quote!(#[deprecated #optional_msg]))
                }
                (Some(_), DeprecationStrategy::Deny) => return None,
            };

        let tokens = quote! {
            #optional_skip_serializing_none
            #optional_flatten
            #optional_rename
            #optional_deprecation_annotation
            #id_deserialize_with
            pub #ident: #qualified_type
        };

        Some(tokens)
    }
}

struct ExpandedVariant<'a> {
    name: Cow<'a, str>,
    variant_type: Option<Cow<'a, str>>,
    on: ResponseTypeId,
    is_default_variant: bool,
}

impl ExpandedVariant<'_> {
    fn render(&self) -> TokenStream {
        let name_ident = Ident::new(&self.name, Span::call_site());
        let optional_type_ident = self.variant_type.as_ref().map(|variant_type| {
            let ident = Ident::new(variant_type, Span::call_site());
            quote!((#ident))
        });

        if self.is_default_variant {
            quote! {
                    #[serde(other)]
            #name_ident #optional_type_ident
                }
        } else {
            quote!(#name_ident #optional_type_ident)
        }
    }
}

pub(crate) struct ExpandedType<'a> {
    name: Cow<'a, str>,
}

pub(crate) struct ExpandedSelection<'a> {
    query: &'a BoundQuery<'a>,
    types: Vec<ExpandedType<'a>>,
    fields: Vec<ExpandedField<'a>>,
    variants: Vec<ExpandedVariant<'a>>,
    aliases: Vec<TypeAlias<'a>>,
    options: &'a GraphQLClientCodegenOptions,
}

impl<'a> ExpandedSelection<'a> {
    pub(crate) fn schema(&self) -> &'a Schema {
        self.query.schema
    }

    fn push_type(&mut self, tpe: ExpandedType<'a>) -> ResponseTypeId {
        let id = self.types.len();
        self.types.push(tpe);

        ResponseTypeId(id as u32)
    }

    fn push_field(&mut self, field: ExpandedField<'a>) {
        self.fields.push(field);
    }

    fn push_type_alias(&mut self, alias: TypeAlias<'a>) {
        self.aliases.push(alias)
    }

    fn push_variant(&mut self, variant: ExpandedVariant<'a>) {
        self.variants.push(variant);
    }

    /// Returns a tuple to be interpreted as (graphql_name, rust_name).
    pub(crate) fn field_name(&self, field: &'a SelectedField) -> (&'a str, Cow<'a, str>) {
        let name = field
            .alias()
            .unwrap_or_else(|| &field.schema_field(self.query.schema).name);
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
        let serde = self.options.serde_path();
        let serde_path = serde.to_token_stream().to_string();

        let mut items = Vec::with_capacity(self.types.len());

        for (type_id, ty) in self.types() {
            let struct_name = Ident::new(&ty.name, Span::call_site());

            // If the type is aliased, stop here.
            if let Some(alias) = self.aliases.iter().find(|alias| alias.struct_id == type_id) {
                let type_name = syn::parse_str::<Path>(alias.name).unwrap();
                let type_name = if alias.boxed {
                    quote!(Box<#type_name>)
                } else {
                    quote!(#type_name)
                };
                let item = quote! {
                    pub type #struct_name = #type_name;
                };
                items.push(item);
                continue;
            }

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

            let (on_field, on_enum) = if !on_variants.is_empty() {
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
                #[serde(crate = #serde_path)]
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
