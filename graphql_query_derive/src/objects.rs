use field_type::FieldType;
use graphql_parser::query;
use heck::SnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;

#[derive(Debug, PartialEq)]
pub struct GqlObject {
    pub name: String,
    pub fields: Vec<GqlObjectField>,
}

#[derive(Debug, PartialEq)]
pub struct GqlObjectField {
    pub name: String,
    pub type_: FieldType,
}

impl GqlObject {
    pub fn response_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &query::SelectionSet,
        prefix: &str,
    ) -> TokenStream {
        let name = Ident::new(&format!("{}{}", prefix, self.name), Span::call_site());
        let fields = self.response_fields_for_selection(query_context, selection, prefix);
        let prefix = format!("{}{}", prefix, self.name);
        let field_impls = self.field_impls_for_selection(query_context, selection, &prefix);
        quote! {
            #(#field_impls)*

            #[derive(Debug, Deserialize)]
            pub struct #name {
                #(#fields,)*
            }
        }
    }

    pub fn field_impls_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &query::SelectionSet,
        prefix: &str,
    ) -> Vec<TokenStream> {
        selection
            .items
            .iter()
            .map(|selected| {
                if let query::Selection::Field(selected) = selected {
                    let ty = self
                        .fields
                        .iter()
                        .find(|f| f.name == selected.name)
                        .expect("field found")
                        .type_
                        .inner_name_string();
                    query_context.maybe_expand_field(&selected, &ty, &prefix)
                } else {
                    quote!()
                }
            })
            .collect()
    }

    pub fn response_fields_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &query::SelectionSet,
        prefix: &str,
    ) -> Vec<TokenStream> {
        let mut fields = Vec::new();

        for item in selection.items.iter() {
            match item {
                query::Selection::Field(f) => {
                    let name = &f.name;
                    let ty = &self
                        .fields
                        .iter()
                        .find(|field| field.name.as_str() == name.as_str())
                        .unwrap()
                        .type_;
                    let name = Ident::new(name, Span::call_site());
                    let ty = ty.to_rust(query_context, prefix);
                    fields.push(quote!(#name: #ty));
                }
                query::Selection::FragmentSpread(fragment) => {
                    let field_name =
                        Ident::new(&fragment.fragment_name.to_snake_case(), Span::call_site());
                    let type_name = Ident::new(&fragment.fragment_name, Span::call_site());
                    fields.push(quote!{
                        #[serde(flatten)]
                        #field_name: #type_name
                    })
                }
                query::Selection::InlineFragment(_) => {
                    unreachable!("inline fragment on object field")
                }
            }
        }

        fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphql_parser::Pos;

    #[test]
    fn simple_object() {
        let simple = GqlObject {
            name: "SimpleObject".to_string(),
            fields: vec![
                GqlObjectField {
                    name: "greeting".to_string(),
                    type_: FieldType::Named(Ident::new("String", Span::call_site())),
                },
                GqlObjectField {
                    name: "name".to_string(),
                    type_: FieldType::Named(Ident::new("String", Span::call_site())),
                },
            ],
        };

        let pos = Pos { line: 0, column: 0 };

        let selection_set = query::SelectionSet {
            span: (pos.clone(), pos.clone()),
            items: vec![query::Selection::Field(query::Field {
                position: pos.clone(),
                alias: None,
                name: "name".to_string(),
                arguments: vec![],
                directives: vec![],
                selection_set: query::SelectionSet {
                    span: (pos.clone(), pos.clone()),
                    items: vec![],
                },
            })],
        };

        assert_eq!(
            simple
                .response_for_selection(&mut QueryContext::new(), &selection_set)
                .to_string(),
            "# [ derive ( Debug , Deserialize ) ] pub struct SimpleObject { name : String , }"
        )
    }
}
