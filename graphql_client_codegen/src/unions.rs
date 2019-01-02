use constants::*;
use failure;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use selection::{Selection, SelectionFragmentSpread, SelectionItem};
use std::cell::Cell;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GqlUnion {
    pub description: Option<String>,
    pub variants: BTreeSet<String>,
    pub is_required: Cell<bool>,
}

#[derive(Debug, Fail)]
#[fail(display = "UnionError")]
enum UnionError {
    #[fail(display = "Unknown type: {}", ty)]
    UnknownType { ty: String },
    #[fail(display = "Missing __typename in selection for {}", union_name)]
    MissingTypename { union_name: String },
}

type UnionVariantResult<'selection> =
    Result<(Vec<TokenStream>, Vec<TokenStream>, Vec<&'selection str>), failure::Error>;

/// Returns a triple.
///
/// - The first element is the union variants to be inserted directly into the `enum` declaration.
/// - The second is the structs for each variant's sub-selection
/// - The last one contains which fields have been selected on the union, so we can make the enum exhaustive by complementing with those missing.
pub(crate) fn union_variants<'selection>(
    selection: &'selection Selection,
    context: &QueryContext,
    prefix: &str,
) -> UnionVariantResult<'selection> {
    let selection = selection.selected_variants_on_union(context)?;
    let used_variants = selection.keys().collect();
    let mut children_definitions = Vec::with_capacity(selection.size());
    let mut variants = Vec::with_capacity(selection.size());

    for (on, fields) in selection.iter() {
        let variant_name = Ident::new(&on, Span::call_site());
        used_variants.push(on.to_string());

        let new_prefix = format!("{}On{}", prefix, on);

        let variant_type = Ident::new(&new_prefix, Span::call_site());

        let field_object_type = context
            .schema
            .objects
            .get(on)
            .map(|_f| context.maybe_expand_field(&on, fields, &new_prefix));
        let field_interface = context
            .schema
            .interfaces
            .get(on)
            .map(|_f| context.maybe_expand_field(&on, fields, &new_prefix));
        let field_union_type = context
            .schema
            .unions
            .get(on)
            .map(|_f| context.maybe_expand_field(&on, fields, &new_prefix));

        match field_object_type.or(field_interface).or(field_union_type) {
            Some(tokens) => children_definitions.push(tokens?),
            None => Err(UnionError::UnknownType { ty: on.to_string() })?,
        };

        variants.push(quote! {
            #variant_name(#variant_type)
        })
    }

    Ok((variants, children_definitions, used_variants))

    // let variants: Result<Vec<TokenStream>, failure::Error> = selection
    //     .0
    //     .iter()
    //     // ignore __typename
    //     .filter(|item| {
    //         if let SelectionItem::Field(f) = item {
    //             f.name != TYPENAME_FIELD
    //         } else {
    //             true
    //         }
    //     })
    //     // .flat_map(
    //     //     |item| -> impl ::std::iter::Iterator<Item = Result<(_, _), failure::Error>> {
    //     //         match item {
    //     //             SelectionItem::Field(_) => Err(format_err!("field selection on union"))?,
    //     //             SelectionItem::FragmentSpread(SelectionFragmentSpread { fragment_name }) => {
    //     //                 let fragment = query_context
    //     //                     .fragments
    //     //                     .get(fragment_name)
    //     //                     .ok_or_else(|| format_err!("Unknown fragment: {}", &fragment_name))?;
    //     //                 // found the bug! `on` doesn't mean the same here as in the inline fragments
    //     //                 std::iter::once(Ok((&fragment.on, &fragment.selection)))
    //     //             }
    //     //             SelectionItem::InlineFragment(frag) => {
    //     //                 std::iter::once(Ok((&frag.on, &frag.fields)))
    //     //             }
    //     //         }
    //     //     },
    //     // )
    //     // // .collect::<Result<_, _>>()?
    //     .map(|result: Result<(_, _), failure::Error>| -> Result<_, _> {
    //         let Ok((on, fields)) = result?;

    // let variants = variants?;
}

impl GqlUnion {
    /// Returns the code to deserialize this union in the response given the query selection.
    pub(crate) fn response_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<TokenStream, failure::Error> {
        let typename_field = selection.extract_typename(query_context);

        if typename_field.is_none() {
            Err(UnionError::MissingTypename {
                union_name: prefix.into(),
            })?;
        }

        let struct_name = Ident::new(prefix, Span::call_site());
        let derives = query_context.response_derives();

        let (mut variants, children_definitions, used_variants) =
            union_variants(selection, query_context, prefix)?;

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
    use deprecation::DeprecationStatus;
    use field_type::FieldType;
    use objects::{GqlObject, GqlObjectField};
    use selection::*;

    #[test]
    fn union_response_for_selection_complains_if_typename_is_missing() {
        let fields = vec![
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "User".to_string(),
                fields: Selection(vec![SelectionItem::Field(SelectionField {
                    alias: None,
                    name: "firstName".to_string(),
                    fields: Selection(vec![]),
                })]),
            }),
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "Organization".to_string(),
                fields: Selection(vec![SelectionItem::Field(SelectionField {
                    alias: None,
                    name: "title".to_string(),
                    fields: Selection(vec![]),
                })]),
            }),
        ];
        let mut context = QueryContext::new_empty();
        let selection = Selection(fields);
        let prefix = "Meow";
        let union = GqlUnion {
            description: None,
            variants: BTreeSet::new(),
            is_required: false.into(),
        };

        context.schema.objects.insert(
            "User".to_string(),
            GqlObject {
                description: None,
                name: "User".to_string(),
                fields: vec![
                    GqlObjectField {
                        description: None,
                        name: "firstName".to_string(),
                        type_: FieldType::Named("String".to_string()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "lastName".to_string(),
                        type_: FieldType::Named("String".to_string()),

                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "createdAt".to_string(),
                        type_: FieldType::Named("Date".to_string()),
                        deprecation: DeprecationStatus::Current,
                    },
                ],
                is_required: false.into(),
            },
        );

        context.schema.objects.insert(
            "Organization".to_string(),
            GqlObject {
                description: None,
                name: "Organization".to_string(),
                fields: vec![
                    GqlObjectField {
                        description: None,
                        name: "title".to_string(),
                        type_: FieldType::Named("String".to_string()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "created_at".to_string(),
                        type_: FieldType::Named("Date".to_string()),
                        deprecation: DeprecationStatus::Current,
                    },
                ],
                is_required: false.into(),
            },
        );

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
                name: "__typename".to_string(),
                fields: Selection(vec![]),
            }),
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "User".to_string(),
                fields: Selection(vec![SelectionItem::Field(SelectionField {
                    alias: None,
                    name: "firstName".to_string(),
                    fields: Selection(vec![]),
                })]),
            }),
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "Organization".to_string(),
                fields: Selection(vec![SelectionItem::Field(SelectionField {
                    alias: None,
                    name: "title".to_string(),
                    fields: Selection(vec![]),
                })]),
            }),
        ];
        let mut context = QueryContext::new_empty();
        let selection = Selection(fields);
        let prefix = "Meow";
        let union = GqlUnion {
            description: None,
            variants: BTreeSet::new(),
            is_required: false.into(),
        };

        let result = union.response_for_selection(&context, &selection, &prefix);

        assert!(result.is_err());

        context.schema.objects.insert(
            "User".to_string(),
            GqlObject {
                description: None,
                name: "User".to_string(),
                fields: vec![
                    GqlObjectField {
                        description: None,
                        name: "__typename".to_string(),
                        type_: FieldType::Named(string_type()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "firstName".to_string(),
                        type_: FieldType::Named(string_type()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "lastName".to_string(),
                        type_: FieldType::Named(string_type()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "createdAt".to_string(),
                        type_: FieldType::Named("Date".to_string()),
                        deprecation: DeprecationStatus::Current,
                    },
                ],
                is_required: false.into(),
            },
        );

        context.schema.objects.insert(
            "Organization".to_string(),
            GqlObject {
                description: None,
                name: "Organization".to_string(),
                fields: vec![
                    GqlObjectField {
                        description: None,
                        name: "__typename".to_string(),
                        type_: FieldType::Named(string_type()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "title".to_string(),
                        type_: FieldType::Named("String".to_string()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "createdAt".to_string(),
                        type_: FieldType::Named("Date".to_string()),
                        deprecation: DeprecationStatus::Current,
                    },
                ],
                is_required: false.into(),
            },
        );

        let result = union.response_for_selection(&context, &selection, &prefix);

        println!("{:?}", result);

        assert!(result.is_ok());

        assert_eq!(
            result.unwrap().to_string(),
            vec![
                "# [ derive ( Deserialize ) ] ",
                "pub struct MeowOnUser { # [ serde ( rename = \"firstName\" ) ] pub first_name : String , } ",
                "# [ derive ( Deserialize ) ] ",
                "pub struct MeowOnOrganization { pub title : String , } ",
                "# [ derive ( Deserialize ) ] ",
                "# [ serde ( tag = \"__typename\" ) ] ",
                "pub enum Meow { User ( MeowOnUser ) , Organization ( MeowOnOrganization ) }",
            ].into_iter()
                .collect::<String>(),
        );
    }
}
