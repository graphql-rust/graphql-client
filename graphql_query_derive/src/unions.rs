use constants::*;
use failure;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use selection::{Selection, SelectionItem};
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub struct GqlUnion(pub BTreeSet<String>);

#[derive(Debug, Fail)]
#[fail(display = "UnionError")]
enum UnionError {
    #[fail(display = "Unknown type: {}", ty)]
    UnknownType { ty: String },
    #[fail(display = "Missing __typename in selection for {}", union_name)]
    MissingTypename { union_name: String },
}

pub fn union_variants(
    selection: &Selection,
    query_context: &QueryContext,
    prefix: &str,
) -> Result<(Vec<TokenStream>, Vec<TokenStream>, Vec<String>), failure::Error> {
    let mut children_definitions = Vec::new();
    let mut used_variants = Vec::with_capacity(selection.0.len());

    let variants: Result<Vec<TokenStream>, failure::Error> = selection
            .0
            .iter()
            // ignore __typename
            .filter(|item| {
                if let SelectionItem::Field(f) = item {
                    f.name != TYPENAME_FIELD
                } else {
                    true
                }
            })
            .map(|item| {
                match item {
                    SelectionItem::Field(_) => Err(format_err!("field selection on union"))?,
                    SelectionItem::FragmentSpread(_) => Err(format_err!("fragment spread on union"))?,
                    SelectionItem::InlineFragment(frag) => {
                        let variant_name = Ident::new(&frag.on, Span::call_site());
                        used_variants.push(frag.on.to_string());

                        let new_prefix = format!("{}On{}", prefix, frag.on);

                        let variant_type = Ident::new(&new_prefix, Span::call_site());

                        let field_object_type =
                            query_context.schema.objects.get(&frag.on).map(|_f| {
                                query_context.maybe_expand_field(
                                    &frag.on,
                                    &frag.fields,
                                    &new_prefix,
                                )
                            });
                        let field_interface = query_context.schema.interfaces.get(&frag.on).map(|_f| {
                            query_context.maybe_expand_field(
                                &frag.on,
                                &frag.fields,
                                &new_prefix
                            )
                        });
                        // nested unions, is that even a thing?
                        let field_union_type = query_context.schema.unions.get(&frag.on).map(|_f| {
                            query_context.maybe_expand_field(&frag.on, &frag.fields, &new_prefix)
                        });

                        match field_object_type.or(field_interface).or(field_union_type) {
                            Some(tokens) => children_definitions.push(tokens?),
                            None => Err(UnionError::UnknownType {
                                ty: frag.on.to_string(),
                            })?,
                        };

                        Ok(quote! {
                            #variant_name(#variant_type)
                        })
                    }
                }
            })
            .collect();

    let variants = variants?;

    Ok((variants, children_definitions, used_variants))
}

impl GqlUnion {
    pub fn response_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<TokenStream, failure::Error> {
        let struct_name = Ident::new(prefix, Span::call_site());

        let typename_field = selection.extract_typename();

        if typename_field.is_none() {
            Err(UnionError::MissingTypename {
                union_name: prefix.into(),
            })?;
        }

        let (mut variants, children_definitions, used_variants) =
            union_variants(selection, query_context, prefix)?;

        variants.extend(
            self.0
                .iter()
                .filter(|v| used_variants.iter().find(|a| a == v).is_none())
                .map(|v| {
                    let v = Ident::new(v, Span::call_site());
                    quote!(#v)
                }),
        );

        Ok(quote!{
            #(#children_definitions)*

            #[derive(Debug, Serialize, Deserialize)]
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
    use field_type::FieldType;
    use objects::{GqlObject, GqlObjectField};
    use selection::*;

    #[test]
    fn union_response_for_selection_complains_if_typename_is_missing() {
        let fields = vec![
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "User".to_string(),
                fields: Selection(vec![SelectionItem::Field(SelectionField {
                    name: "first_name".to_string(),
                    fields: Selection(vec![]),
                })]),
            }),
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "Organization".to_string(),
                fields: Selection(vec![SelectionItem::Field(SelectionField {
                    name: "title".to_string(),
                    fields: Selection(vec![]),
                })]),
            }),
        ];
        let mut context = QueryContext::new_empty();
        let selection = Selection(fields);
        let prefix = "Meow";
        let union = GqlUnion(BTreeSet::new());

        context.schema.objects.insert(
            "User".to_string(),
            GqlObject {
                description: None,
                name: "User".to_string(),
                fields: vec![
                    GqlObjectField {
                        description: None,
                        name: "first_name".to_string(),
                        type_: FieldType::Named(Ident::new("String", Span::call_site())),
                    },
                    GqlObjectField {
                        description: None,
                        name: "last_name".to_string(),
                        type_: FieldType::Named(Ident::new("String", Span::call_site())),
                    },
                    GqlObjectField {
                        description: None,
                        name: "created_at".to_string(),
                        type_: FieldType::Named(Ident::new("Date", Span::call_site())),
                    },
                ],
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
                        type_: FieldType::Named(Ident::new("String", Span::call_site())),
                    },
                    GqlObjectField {
                        description: None,
                        name: "created_at".to_string(),
                        type_: FieldType::Named(Ident::new("Date", Span::call_site())),
                    },
                ],
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
                name: "__typename".to_string(),
                fields: Selection(vec![]),
            }),
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "User".to_string(),
                fields: Selection(vec![SelectionItem::Field(SelectionField {
                    name: "first_name".to_string(),
                    fields: Selection(vec![]),
                })]),
            }),
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "Organization".to_string(),
                fields: Selection(vec![SelectionItem::Field(SelectionField {
                    name: "title".to_string(),
                    fields: Selection(vec![]),
                })]),
            }),
        ];
        let mut context = QueryContext::new_empty();
        let selection = Selection(fields);
        let prefix = "Meow";
        let union = GqlUnion(BTreeSet::new());

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
                    },
                    GqlObjectField {
                        description: None,
                        name: "first_name".to_string(),
                        type_: FieldType::Named(string_type()),
                    },
                    GqlObjectField {
                        description: None,
                        name: "last_name".to_string(),
                        type_: FieldType::Named(string_type()),
                    },
                    GqlObjectField {
                        description: None,
                        name: "created_at".to_string(),
                        type_: FieldType::Named(Ident::new("Date", Span::call_site())),
                    },
                ],
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
                    },
                    GqlObjectField {
                        description: None,
                        name: "title".to_string(),
                        type_: FieldType::Named(Ident::new("String", Span::call_site())),
                    },
                    GqlObjectField {
                        description: None,
                        name: "created_at".to_string(),
                        type_: FieldType::Named(Ident::new("Date", Span::call_site())),
                    },
                ],
            },
        );

        let result = union.response_for_selection(&context, &selection, &prefix);

        println!("{:?}", result);

        assert!(result.is_ok());

        assert_eq!(
            result.unwrap().to_string(),
            vec![
                "# [ derive ( Debug , Serialize , Deserialize ) ] ",
                "# [ serde ( rename_all = \"camelCase\" ) ] ",
                "pub struct MeowOnUser { pub first_name : String , } ",
                "# [ derive ( Debug , Serialize , Deserialize ) ] ",
                "# [ serde ( rename_all = \"camelCase\" ) ] ",
                "pub struct MeowOnOrganization { pub title : String , } ",
                "# [ derive ( Debug , Serialize , Deserialize ) ] ",
                "# [ serde ( tag = \"__typename\" ) ] ",
                "pub enum Meow { User ( MeowOnUser ) , Organization ( MeowOnOrganization ) }",
            ].into_iter()
                .collect::<String>(),
        );
    }
}
