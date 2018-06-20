use failure;
use heck::SnakeCase;
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
}

impl GqlUnion {
    pub fn response_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> Result<TokenStream, failure::Error> {
        let struct_name = Ident::new(prefix, Span::call_site());
        let mut children_definitions: Vec<TokenStream> = Vec::new();
        let variants: Result<Vec<TokenStream>, failure::Error> = selection
            .0
            .iter()
            .map(|item| {
                match item {
                    SelectionItem::Field(_) => unreachable!("field selection on union"),
                    SelectionItem::FragmentSpread(_) => unreachable!("fragment spread on union"),
                    SelectionItem::InlineFragment(frag) => {
                        let variant_name = Ident::new(&frag.on, Span::call_site());

                        let new_prefix = format!("{}On{}", prefix, frag.on);

                        let variant_type = Ident::new(&new_prefix, Span::call_site());

                        let field_object_type =
                            query_context.schema.objects.get(&frag.on).map(|f| {
                                query_context.maybe_expand_field(
                                    &frag.on,
                                    &frag.fields,
                                    &new_prefix,
                                )
                            });
                        let field_interface =
                            query_context.schema.interfaces.get(&frag.on).map(|f| {
                                query_context.maybe_expand_field(
                                    &frag.on,
                                    &frag.fields,
                                    &new_prefix,
                                )
                            });
                        // nested unions, is that even a thing?
                        let field_union_type = query_context.schema.unions.get(&frag.on).map(|f| {
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

        Ok(quote!{
            #(#children_definitions)*

            #[derive(Deserialize)]
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
                name: "User".to_string(),
                fields: vec![
                    GqlObjectField {
                        name: "first_name".to_string(),
                        type_: FieldType::Named(Ident::new("String", Span::call_site())),
                    },
                    GqlObjectField {
                        name: "last_name".to_string(),
                        type_: FieldType::Named(Ident::new("String", Span::call_site())),
                    },
                    GqlObjectField {
                        name: "created_at".to_string(),
                        type_: FieldType::Named(Ident::new("Date", Span::call_site())),
                    },
                ],
            },
        );

        context.schema.objects.insert(
            "Organization".to_string(),
            GqlObject {
                name: "Organization".to_string(),
                fields: vec![
                    GqlObjectField {
                        name: "title".to_string(),
                        type_: FieldType::Named(Ident::new("String", Span::call_site())),
                    },
                    GqlObjectField {
                        name: "created_at".to_string(),
                        type_: FieldType::Named(Ident::new("Date", Span::call_site())),
                    },
                ],
            },
        );

        let result = union.response_for_selection(&context, &selection, &prefix);

        assert!(result.is_err());

        assert_eq!(format!("{}", result.unwrap_err()), "");
    }

    #[test]
    fn union_response_for_selection_works() {
        let fields = vec![
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "User".to_string(),
                fields: Selection(vec![
                    SelectionItem::Field(SelectionField {
                        name: "first_name".to_string(),
                        fields: Selection(vec![]),
                    }),
                    SelectionItem::Field(SelectionField {
                        name: "__typename".to_string(),
                        fields: Selection(vec![]),
                    }),
                ]),
            }),
            SelectionItem::InlineFragment(SelectionInlineFragment {
                on: "Organization".to_string(),
                fields: Selection(vec![
                    SelectionItem::Field(SelectionField {
                        name: "title".to_string(),
                        fields: Selection(vec![]),
                    }),
                    SelectionItem::Field(SelectionField {
                        name: "__typename".to_string(),
                        fields: Selection(vec![]),
                    }),
                ]),
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
                name: "User".to_string(),
                fields: vec![
                    GqlObjectField {
                        name: "first_name".to_string(),
                        type_: FieldType::Named(Ident::new("String", Span::call_site())),
                    },
                    GqlObjectField {
                        name: "last_name".to_string(),
                        type_: FieldType::Named(Ident::new("String", Span::call_site())),
                    },
                    GqlObjectField {
                        name: "created_at".to_string(),
                        type_: FieldType::Named(Ident::new("Date", Span::call_site())),
                    },
                ],
            },
        );

        context.schema.objects.insert(
            "Organization".to_string(),
            GqlObject {
                name: "Organization".to_string(),
                fields: vec![
                    GqlObjectField {
                        name: "title".to_string(),
                        type_: FieldType::Named(Ident::new("String", Span::call_site())),
                    },
                    GqlObjectField {
                        name: "created_at".to_string(),
                        type_: FieldType::Named(Ident::new("Date", Span::call_site())),
                    },
                ],
            },
        );

        let result = union.response_for_selection(&context, &selection, &prefix);

        assert!(result.is_ok());

        assert_eq!(
            result.unwrap().to_string(),
            vec![
                "# [ derive ( Debug , Serialize , Deserialize ) ] ",
                "pub struct MeowOnUser { first_name : String , } ",
                "# [ derive ( Debug , Serialize , Deserialize ) ] ",
                "pub struct MeowOnOrganization { title : String , } ",
                "# [ derive ( Deserialize ) ] ",
                "# [ serde ( tag = \"__typename\" ) ] ",
                "pub enum Meow { User ( MeowOnUser ) , Organization ( MeowOnOrganization ) }",
            ].into_iter()
                .collect::<String>(),
        );
    }
}
