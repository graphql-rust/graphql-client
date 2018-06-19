use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use std::collections::BTreeSet;
use selection::{Selection, SelectionItem};
use heck::SnakeCase;

#[derive(Debug, PartialEq)]
pub struct GqlUnion(pub BTreeSet<String>);

impl GqlUnion {
    pub fn response_for_selection(
        &self,
        query_context: &QueryContext,
        selection: &Selection,
        prefix: &str,
    ) -> TokenStream {
        let struct_name = Ident::new(prefix, Span::call_site());
        let mut children_definitions = Vec::new();
        let fields = selection.0.iter().map(|item| {
            match item {
                SelectionItem::Field(_) => unreachable!("field selection on union"),
                SelectionItem::FragmentSpread(_) => unreachable!("fragment spread on union"),
                SelectionItem::InlineFragment(frag) => {
                    let field_name = Ident::new(
                        &format!("on_{}", frag.on).to_snake_case(),
                        Span::call_site(),
                    );

                    let field_type = Ident::new(&format!("{}On{}", prefix, frag.on), Span::call_site());

                    let new_prefix = format!("{}On{}", prefix, frag.on);

                    let field_object_type = query_context.schema.objects.get(&frag.on)
                        .map(|f| query_context.maybe_expand_field(&frag.on, &frag.fields, &new_prefix));
                    let field_interface = query_context.schema.interfaces.get(&frag.on)
                        .map(|f| query_context.maybe_expand_field(&frag.on, &frag.fields, &new_prefix));
                    // nested unions, is that even a thing?
                    let field_union_type = query_context.schema.unions.get(&frag.on)
                        .map(|f| query_context.maybe_expand_field(&frag.on, &frag.fields, &new_prefix));

                    if let Some(tokens) = field_object_type.or(field_interface).or(field_union_type) {
                        children_definitions.push(tokens)
                    }

        // query_context.maybe_expand_field(

        // );
                    quote! {
                        #field_name: #field_type
                    }
                }
            }
        });

        quote!{
            #[derive(Deserialize)]
            pub struct #struct_name {
                #(#fields),*
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn union_response_for_selection_works() {
        // unimplemented!()
    }

}
