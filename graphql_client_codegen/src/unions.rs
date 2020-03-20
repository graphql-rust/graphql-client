use crate::query::QueryContext;
use crate::selection::Selection;
use anyhow::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::cell::Cell;
use std::collections::BTreeSet;

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
