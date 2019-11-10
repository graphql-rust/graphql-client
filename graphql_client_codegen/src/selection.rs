use crate::constants::*;
use failure::*;
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
    pub fn as_typename(&self) -> Option<&SelectionField<'_>> {
        if let SelectionItem::Field(f) = self {
            if f.name == TYPENAME_FIELD {
                return Some(f);
            }
        }
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selection<'query>(Vec<SelectionItem<'query>>);

impl<'query> Selection<'query> {
    pub(crate) fn extract_typename<'s, 'context: 's>(
        &'s self,
        context: &'context crate::query::QueryContext<'_, '_>,
    ) -> Option<&SelectionField<'_>> {
        // __typename is selected directly
        if let Some(field) = self.0.iter().filter_map(SelectionItem::as_typename).next() {
            return Some(field);
        };

        // typename is selected through a fragment
        (&self)
            .into_iter()
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
        context: &'s crate::query::QueryContext<'_, '_>,
        selected_variants: &mut BTreeMap<&'s str, Selection<'s>>,
        // the name of the type the selection applies to
        selection_on: &str,
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

                    // The fragment can either be on the union/interface itself, or on one of its variants (type-refining fragment).
                    if fragment.on.name() == selection_on {
                        // The fragment is on the union/interface itself.
                        fragment.selection.selected_variants_on_union_inner(
                            context,
                            selected_variants,
                            selection_on,
                        )?;
                    } else {
                        // Type-refining fragment
                        selected_variants
                            .entry(fragment.on.name())
                            .and_modify(|entry| entry.0.extend(fragment.selection.0.clone()))
                            .or_insert_with(|| {
                                let mut items = Vec::with_capacity(fragment.selection.0.len());
                                items.extend(fragment.selection.0.clone());
                                Selection(items)
                            });
                    }
                }
            }
        }

        Ok(())
    }

    /// This method should only be invoked on selections on union and interface fields. It returns a map from the name of the selected variants to the corresponding selections.
    ///
    /// Importantly, it will "flatten" the fragments and handle multiple selections of the same variant.
    ///
    /// The `context` argument is required so we can expand the fragments.
    pub(crate) fn selected_variants_on_union<'s>(
        &'s self,
        context: &'s crate::query::QueryContext<'_, '_>,
        // the name of the type the selection applies to
        selection_on: &str,
    ) -> Result<BTreeMap<&'s str, Selection<'s>>, failure::Error> {
        let mut selected_variants = BTreeMap::new();

        self.selected_variants_on_union_inner(context, &mut selected_variants, selection_on)?;

        Ok(selected_variants)
    }

    #[cfg(test)]
    pub(crate) fn new_empty() -> Selection<'static> {
        Selection(Vec::new())
    }

    #[cfg(test)]
    pub(crate) fn from_vec(vec: Vec<SelectionItem<'query>>) -> Self {
        Selection(vec)
    }

    pub(crate) fn contains_fragment(&self, fragment_name: &str) -> bool {
        (&self).into_iter().any(|item| match item {
            SelectionItem::Field(field) => field.fields.contains_fragment(fragment_name),
            SelectionItem::InlineFragment(inline_fragment) => {
                inline_fragment.fields.contains_fragment(fragment_name)
            }
            SelectionItem::FragmentSpread(fragment) => fragment.fragment_name == fragment_name,
        })
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn require_items<'s>(&self, context: &crate::query::QueryContext<'query, 's>) {
        self.0.iter().for_each(|item| {
            if let SelectionItem::FragmentSpread(SelectionFragmentSpread { fragment_name }) = item {
                context.require_fragment(fragment_name);
            }
        })
    }
}

impl<'query> std::convert::From<&'query SelectionSet> for Selection<'query> {
    fn from(selection_set: &SelectionSet) -> Selection<'_> {
        use graphql_parser::query::Selection;

        let mut items = Vec::with_capacity(selection_set.items.len());

        for item in &selection_set.items {
            let converted = match item {
                Selection::Field(f) => SelectionItem::Field(SelectionField {
                    alias: f.alias.as_ref().map(String::as_str),
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

impl<'a, 'query> std::iter::IntoIterator for &'a Selection<'query> {
    type Item = &'a SelectionItem<'query>;
    type IntoIter = std::slice::Iter<'a, SelectionItem<'query>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> std::iter::FromIterator<SelectionItem<'a>> for Selection<'a> {
    fn from_iter<T: std::iter::IntoIterator<Item = SelectionItem<'a>>>(iter: T) -> Selection<'a> {
        Selection(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_extract_typename_simple_case() {
        let selection = Selection::new_empty();
        let schema = crate::schema::Schema::new();
        let context = crate::query::QueryContext::new_empty(&schema);

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

        let schema = crate::schema::Schema::new();
        let obj = crate::objects::GqlObject::new("MyObject", None);
        let mut context = crate::query::QueryContext::new_empty(&schema);
        context.fragments.insert(
            "MyFragment",
            crate::fragments::GqlFragment {
                name: "MyFragment",
                on: crate::fragments::FragmentTarget::Object(&obj),
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

        let selection: Selection<'_> = selection_set.into();

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
