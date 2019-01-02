use constants::*;
use graphql_parser::query::SelectionSet;
use std::collections::BTreeMap;

/// A single object field as part of a selection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionField<'query> {
    pub alias: Option<&'query str>,
    pub name: &'query str,
    pub fields: Selection<'query>,
}

/// A spread fragment in a selection (e.g. `...MyFragment`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionFragmentSpread<'query> {
    pub fragment_name: &'query str,
}

/// An inline fragment as part of a selection (e.g. `...on MyThing { name }`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionInlineFragment<'query> {
    pub on: &'query str,
    pub fields: Selection<'query>,
}

/// An element in a query selection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectionItem<'query> {
    Field(SelectionField<'query>),
    FragmentSpread(SelectionFragmentSpread<'query>),
    InlineFragment(SelectionInlineFragment<'query>),
}

impl<'query> SelectionItem<'query> {
    pub fn as_typename(&self) -> Option<&SelectionField> {
        if let SelectionItem::Field(f) = self {
            if f.name == TYPENAME_FIELD {
                return Some(f);
            }
        }
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selection<'query>(pub Vec<SelectionItem<'query>>);

impl<'query> Selection<'query> {
    pub(crate) fn extract_typename<'s, 'context: 's>(
        &'s self,
        context: &'context crate::query::QueryContext,
    ) -> Option<&SelectionField> {
        // __typename is selected directly
        if let Some(field) = self.0.iter().filter_map(|f| f.as_typename()).next() {
            return Some(field);
        };

        // typename is selected through a fragment
        self.0
            .iter()
            .filter_map(|f| match f {
                SelectionItem::FragmentSpread(SelectionFragmentSpread { fragment_name }) => {
                    Some(fragment_name)
                }
                _ => None,
            })
            .filter_map(|fragment_name| {
                let fragment = context.fragments.get(fragment_name);

                fragment.and_then(|fragment| fragment.selection.extract_typename(context))
            })
            .next()
    }

    // Implementation helper for `selected_variants_on_union`.
    fn selected_variants_on_union_inner<'s>(
        &'s self,
        context: &'s crate::query::QueryContext,
        selected_variants: &mut BTreeMap<&'s str, Selection<'s>>,
    ) -> Result<(), failure::Error> {
        for item in self.0.iter() {
            match item {
                SelectionItem::Field(_) => (),
                SelectionItem::InlineFragment(inline_fragment) => {
                    selected_variants
                        .entry(inline_fragment.on)
                        .and_modify(|entry| entry.0.extend(inline_fragment.fields.0.clone()))
                        .or_insert_with(|| {
                            let mut items = Vec::with_capacity(inline_fragment.fields.0.len());
                            items.extend(inline_fragment.fields.0.clone());
                            Selection(items)
                        });
                }
                SelectionItem::FragmentSpread(SelectionFragmentSpread { fragment_name }) => {
                    let fragment = context
                        .fragments
                        .get(fragment_name)
                        .ok_or_else(|| format_err!("Unknown fragment: {}", &fragment_name))?;

                    fragment
                        .selection
                        .selected_variants_on_union_inner(context, selected_variants)?;
                }
            }
        }

        Ok(())
    }

    /// This method should only be invoked on selections on union fields. It returns the selected varianst and the
    ///
    /// Importantly, it will "flatten" the fragments and handle multiple selections of the same variant.
    ///
    /// The `context` argument is required so we can expand the fragments.
    pub(crate) fn selected_variants_on_union<'s>(
        &'s self,
        context: &'s crate::query::QueryContext,
    ) -> Result<BTreeMap<&'s str, Selection<'s>>, failure::Error> {
        let mut selected_variants = BTreeMap::new();

        self.selected_variants_on_union_inner(context, &mut selected_variants)?;

        Ok(selected_variants)
    }

    #[cfg(test)]
    pub(crate) fn new_empty() -> Selection<'static> {
        Selection(Vec::new())
    }
}

impl<'query> ::std::convert::From<&'query SelectionSet> for Selection<'query> {
    fn from(selection_set: &SelectionSet) -> Selection {
        use graphql_parser::query::Selection;

        let mut items = Vec::with_capacity(selection_set.items.len());

        for item in &selection_set.items {
            let converted = match item {
                Selection::Field(f) => SelectionItem::Field(SelectionField {
                    alias: f.alias.as_ref().map(|s| s.as_str()),
                    name: &f.name,
                    fields: (&f.selection_set).into(),
                }),
                Selection::FragmentSpread(spread) => {
                    SelectionItem::FragmentSpread(SelectionFragmentSpread {
                        fragment_name: &spread.fragment_name,
                    })
                }
                Selection::InlineFragment(inline) => {
                    let graphql_parser::query::TypeCondition::On(ref name) = inline
                        .type_condition
                        .as_ref()
                        .expect("Missing `on` clause.");
                    SelectionItem::InlineFragment(SelectionInlineFragment {
                        on: &name,
                        fields: (&inline.selection_set).into(),
                    })
                }
            };
            items.push(converted);
        }

        Selection(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphql_parser;

    #[test]
    fn selection_extract_typename_simple_case() {
        let selection = Selection::new_empty();
        let schema = ::schema::Schema::new();
        let context = ::query::QueryContext::new_empty(&schema);

        assert!(selection.extract_typename(&context).is_none());
    }

    #[test]
    fn selection_extract_typename_in_fragment() {
        let mut selection = Selection::new_empty();
        selection
            .0
            .push(SelectionItem::FragmentSpread(SelectionFragmentSpread {
                fragment_name: "MyFragment",
            }));

        let mut fragment_selection = Selection::new_empty();
        fragment_selection
            .0
            .push(SelectionItem::Field(SelectionField {
                alias: None,
                name: "__typename",
                fields: Selection::new_empty(),
            }));

        let schema = ::schema::Schema::new();
        let mut context = ::query::QueryContext::new_empty(&schema);
        context.fragments.insert(
            "MyFragment",
            crate::fragments::GqlFragment {
                name: "MyFragment",
                on: "something".into(),
                selection: fragment_selection,
                is_required: std::cell::Cell::new(false),
            },
        );

        assert!(selection.extract_typename(&context).is_some());
    }

    #[test]
    fn selection_from_graphql_parser_selection_set() {
        let query = r##"
        query {
          animal {
            isCat
            isHorse
            ...Timestamps
            barks
            ...on Dog {
                rating
            }
            pawsCount
            aliased: sillyName
          }
        }
        "##;
        let parsed = graphql_parser::parse_query(query).unwrap();
        let selection_set: &graphql_parser::query::SelectionSet = parsed
            .definitions
            .iter()
            .filter_map(|def| {
                if let graphql_parser::query::Definition::Operation(
                    graphql_parser::query::OperationDefinition::Query(q),
                ) = def
                {
                    Some(&q.selection_set)
                } else {
                    None
                }
            })
            .next()
            .unwrap();

        let selection: Selection = selection_set.into();

        assert_eq!(
            selection,
            Selection(vec![SelectionItem::Field(SelectionField {
                alias: None,
                name: "animal",
                fields: Selection(vec![
                    SelectionItem::Field(SelectionField {
                        alias: None,
                        name: "isCat",
                        fields: Selection(Vec::new()),
                    }),
                    SelectionItem::Field(SelectionField {
                        alias: None,
                        name: "isHorse",
                        fields: Selection(Vec::new()),
                    }),
                    SelectionItem::FragmentSpread(SelectionFragmentSpread {
                        fragment_name: "Timestamps",
                    }),
                    SelectionItem::Field(SelectionField {
                        alias: None,
                        name: "barks",
                        fields: Selection(Vec::new()),
                    }),
                    SelectionItem::InlineFragment(SelectionInlineFragment {
                        on: "Dog",
                        fields: Selection(vec![SelectionItem::Field(SelectionField {
                            alias: None,
                            name: "rating",
                            fields: Selection(Vec::new()),
                        })]),
                    }),
                    SelectionItem::Field(SelectionField {
                        alias: None,
                        name: "pawsCount",
                        fields: Selection(Vec::new()),
                    }),
                    SelectionItem::Field(SelectionField {
                        alias: Some("aliased"),
                        name: "sillyName",
                        fields: Selection(Vec::new()),
                    }),
                ]),
            })])
        );
    }
}
