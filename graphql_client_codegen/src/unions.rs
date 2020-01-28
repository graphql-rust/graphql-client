use crate::query::QueryContext;
use crate::selection::Selection;
use anyhow::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::cell::Cell;
use std::collections::BTreeSet;

pub(crate) fn union_type_to_rust(
    ctx: &mut QueryContext<'_>,
    union: (),
) -> Result<TokenStream, anyhow::Error> {
    todo!()
}

/// A GraphQL union (simplified schema representation).
///
/// For code generation purposes, unions will "flatten" fragment spreads, so there is only one enum for the selection. See the tests in the graphql_client crate for examples.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GqlUnion<'schema> {
    pub name: &'schema str,
    pub description: Option<&'schema str>,
    pub variants: BTreeSet<&'schema str>,
    pub is_required: Cell<bool>,
}

#[derive(Debug, Fail)]
#[fail(display = "UnionError")]
enum UnionError {
    #[fail(display = "Unknown type: {}", ty)]
    UnknownType { ty: String },
    #[fail(display = "Unknown variant on union {}: {}", ty, var)]
    UnknownVariant { var: String, ty: String },
    #[fail(display = "Missing __typename in selection for {}", union_name)]
    MissingTypename { union_name: String },
}

type UnionVariantResult<'selection> =
    Result<(Vec<TokenStream>, Vec<TokenStream>, Vec<&'selection str>), anyhow::Error>;

/// Returns a triple.
///
/// - The first element is the union variants to be inserted directly into the `enum` declaration.
/// - The second is the structs for each variant's sub-selection
/// - The last one contains which fields have been selected on the union, so we can make the enum exhaustive by complementing with those missing.
pub(crate) fn union_variants<'selection>(
    selection: &'selection Selection<'_>,
    context: &'selection QueryContext<'selection>,
    prefix: &str,
    selection_on: &str,
) -> UnionVariantResult<'selection> {
    todo!()
    // let selection = selection.selected_variants_on_union(context, selection_on)?;
    // let mut used_variants: Vec<&str> = selection.keys().cloned().collect();
    // let mut children_definitions = Vec::with_capacity(selection.len());
    // let mut variants = Vec::with_capacity(selection.len());

    // for (on, fields) in selection.iter() {
    //     let variant_name = Ident::new(&on, Span::call_site());
    //     used_variants.push(on);

    //     let new_prefix = format!("{}On{}", prefix, on);

    //     let variant_type = Ident::new(&new_prefix, Span::call_site());

    //     let field_object_type = context
    //         .schema
    //         .objects
    //         .get(on)
    //         .map(|_f| context.maybe_expand_field(&on, fields, &new_prefix));
    //     let field_interface = context
    //         .schema
    //         .interfaces
    //         .get(on)
    //         .map(|_f| context.maybe_expand_field(&on, fields, &new_prefix));
    //     let field_union_type = context
    //         .schema
    //         .unions
    //         .get(on)
    //         .map(|_f| context.maybe_expand_field(&on, fields, &new_prefix));

    //     match field_object_type.or(field_interface).or(field_union_type) {
    //         Some(Ok(Some(tokens))) => children_definitions.push(tokens),
    //         Some(Err(err)) => return Err(err),
    //         Some(Ok(None)) => (),
    //         None => {
    //             return Err(UnionError::UnknownType {
    //                 ty: (*on).to_string(),
    //             }
    //             .into())
    //         }
    //     };

    //     variants.push(quote! {
    //         #variant_name(#variant_type)
    //     })
    // }

    // Ok((variants, children_definitions, used_variants))
}

impl<'schema> GqlUnion<'schema> {
    /// Returns the code to deserialize this union in the response given the query selection.
    pub(crate) fn response_for_selection(
        &self,
        query_context: &QueryContext<'_>,
        selection: &Selection<'_>,
        prefix: &str,
    ) -> Result<TokenStream, anyhow::Error> {
        let typename_field = selection.extract_typename(query_context);

        if typename_field.is_none() {
            return Err(UnionError::MissingTypename {
                union_name: prefix.into(),
            }
            .into());
        }

        let struct_name = Ident::new(prefix, Span::call_site());
        let derives = query_context.response_derives();

        let (mut variants, children_definitions, used_variants) =
            union_variants(selection, query_context, prefix, &self.name)?;

        for used_variant in used_variants.iter() {
            if !self.variants.contains(used_variant) {
                return Err(UnionError::UnknownVariant {
                    ty: self.name.into(),
                    var: (*used_variant).to_string(),
                }
                .into());
            }
        }

        variants.extend(
            self.variants
                .iter()
                .filter(|v| used_variants.iter().find(|a| a == v).is_none())
                .map(|v| {
                    let v = Ident::new(v, Span::call_site());
                    quote!(#v)
                }),
        );

        Ok(quote! {
            #(#children_definitions)*

            #derives
            #[serde(tag = "__typename")]
            pub enum #struct_name {
                #(#variants),*
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;
    use crate::deprecation::DeprecationStatus;
    use crate::field_type::FieldType;
    use crate::objects::{GqlObject, GqlObjectField};
    use crate::selection::*;

    #[test]
    fn union_response_for_selection_complains_if_typename_is_missing() {
        let fields = vec![
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "User",
                fields: Selection::from_vec(vec![SelectionItem::Field(SelectionField {
                    alias: None,
                    name: "firstName",
                    fields: Selection::new_empty(),
                })]),
            }),
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "Organization",
                fields: Selection::from_vec(vec![SelectionItem::Field(SelectionField {
                    alias: None,
                    name: "title",
                    fields: Selection::new_empty(),
                })]),
            }),
        ];
        let selection = Selection::from_vec(fields);
        let prefix = "Meow";
        let union = GqlUnion {
            name: "MyUnion",
            description: None,
            variants: BTreeSet::new(),
            is_required: false.into(),
        };

        let mut schema = crate::schema::Schema::new();

        schema.objects.insert(
            "User",
            GqlObject {
                description: None,
                name: "User",
                fields: vec![
                    GqlObjectField {
                        description: None,
                        name: "firstName",
                        type_: FieldType::new("String").nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "lastName",
                        type_: FieldType::new("String").nonnull(),

                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "createdAt",
                        type_: FieldType::new("Date").nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                ],
                is_required: false.into(),
            },
        );

        schema.objects.insert(
            "Organization",
            GqlObject {
                description: None,
                name: "Organization",
                fields: vec![
                    GqlObjectField {
                        description: None,
                        name: "title",
                        type_: FieldType::new("String").nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "created_at",
                        type_: FieldType::new("Date").nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                ],
                is_required: false.into(),
            },
        );
        let context = QueryContext::new_empty(&schema);

        let result = union.response_for_selection(&context, &selection, &prefix);

        assert!(result.is_err());

        assert_eq!(
            format!("{}", result.unwrap_err()),
            "Missing __typename in selection for Meow"
        );
    }

    #[test]
    fn union_response_for_selection_works() {
        let fields = vec![
            SelectionItem::Field(SelectionField {
                alias: None,
                name: "__typename",
                fields: Selection::new_empty(),
            }),
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "User",
                fields: Selection::from_vec(vec![SelectionItem::Field(SelectionField {
                    alias: None,
                    name: "firstName",
                    fields: Selection::new_empty(),
                })]),
            }),
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "Organization",
                fields: Selection::from_vec(vec![SelectionItem::Field(SelectionField {
                    alias: None,
                    name: "title",
                    fields: Selection::new_empty(),
                })]),
            }),
        ];
        let schema = crate::schema::Schema::new();
        let context = QueryContext::new_empty(&schema);
        let selection: Selection<'_> = fields.into_iter().collect();
        let prefix = "Meow";
        let mut union_variants = BTreeSet::new();
        union_variants.insert("User");
        union_variants.insert("Organization");
        let union = GqlUnion {
            name: "MyUnion",
            description: None,
            variants: union_variants,
            is_required: false.into(),
        };

        let result = union.response_for_selection(&context, &selection, &prefix);

        assert!(result.is_err());

        let mut schema = crate::schema::Schema::new();
        schema.objects.insert(
            "User",
            GqlObject {
                description: None,
                name: "User",
                fields: vec![
                    GqlObjectField {
                        description: None,
                        name: "__typename",
                        type_: FieldType::new(string_type()).nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "firstName",
                        type_: FieldType::new(string_type()).nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "lastName",
                        type_: FieldType::new(string_type()).nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "createdAt",
                        type_: FieldType::new("Date").nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                ],
                is_required: false.into(),
            },
        );

        schema.objects.insert(
            "Organization",
            GqlObject {
                description: None,
                name: "Organization",
                fields: vec![
                    GqlObjectField {
                        description: None,
                        name: "__typename",
                        type_: FieldType::new(string_type()).nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "title",
                        type_: FieldType::new("String").nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "createdAt",
                        type_: FieldType::new("Date").nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                ],
                is_required: false.into(),
            },
        );

        let context = QueryContext::new_empty(&schema);

        let result = union.response_for_selection(&context, &selection, &prefix);

        println!("{:?}", result);

        assert!(result.is_ok());

        assert_eq!(
            result.unwrap().to_string(),
            vec![
                "# [ derive ( Deserialize ) ] ",
                "pub struct MeowOnOrganization { pub title : String , } ",
                "# [ derive ( Deserialize ) ] ",
                "pub struct MeowOnUser { # [ serde ( rename = \"firstName\" ) ] pub first_name : String , } ",
                "# [ derive ( Deserialize ) ] ",
                "# [ serde ( tag = \"__typename\" ) ] ",
                "pub enum Meow { Organization ( MeowOnOrganization ) , User ( MeowOnUser ) }",
            ].into_iter()
                .collect::<String>(),
        );
    }

    #[test]
    fn union_rejects_selection_on_non_member_type() {
        let fields = vec![
            SelectionItem::Field(SelectionField {
                alias: None,
                name: "__typename",
                fields: Selection::new_empty(),
            }),
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "SomeNonUnionType",
                fields: Selection::from_vec(vec![SelectionItem::Field(SelectionField {
                    alias: None,
                    name: "field",
                    fields: Selection::new_empty(),
                })]),
            }),
        ];
        let schema = crate::schema::Schema::new();
        let context = QueryContext::new_empty(&schema);
        let selection: Selection<'_> = fields.into_iter().collect();
        let prefix = "Meow";
        let mut union_variants = BTreeSet::new();
        union_variants.insert("Int");
        union_variants.insert("String");
        let union = GqlUnion {
            name: "MyUnion",
            description: None,
            variants: union_variants,
            is_required: false.into(),
        };

        let result = union.response_for_selection(&context, &selection, &prefix);

        assert!(result.is_err());

        let mut schema = crate::schema::Schema::new();
        schema.unions.insert("MyUnion", union.clone());
        schema.objects.insert(
            "SomeNonUnionType",
            GqlObject {
                description: None,
                name: "SomeNonUnionType",
                fields: vec![GqlObjectField {
                    description: None,
                    name: "field",
                    type_: FieldType::new(string_type()),
                    deprecation: DeprecationStatus::Current,
                }],
                is_required: false.into(),
            },
        );

        let context = QueryContext::new_empty(&schema);

        let result = union.response_for_selection(&context, &selection, &prefix);

        println!("{:?}", result);

        assert!(result.is_err());

        match result.unwrap_err().downcast::<UnionError>() {
            Ok(UnionError::UnknownVariant { var, ty }) => {
                assert_eq!(var, "SomeNonUnionType");
                assert_eq!(ty, "MyUnion");
            }
            err => panic!("Unexpected error type: {:?}", err),
        }
    }
}
