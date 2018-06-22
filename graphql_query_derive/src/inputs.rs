use query::QueryContext;
use objects::GqlObjectField;
use proc_macro2::{Ident, Span, TokenStream};
use introspection_response;
use graphql_parser;

#[derive(Debug, PartialEq)]
pub struct GqlInput {
    pub name: String,
    pub fields: Vec<GqlObjectField>,
}

impl GqlInput {
    pub fn to_rust(&self, context: &QueryContext) -> TokenStream {
        let name = Ident::new(&self.name, Span::call_site());
        let fields = self.fields.iter().map(|field| {
            let ty = field.type_.to_rust(&context, "");
            let name = Ident::new(&field.name, Span::call_site());
            quote!(#ty: #name)
        });

        quote!{
            #[derive(Debug, Serialize)]
            #[serde(rename_all = "camelCase")]
            pub struct #name {
                #(#fields,)*
            }
        }
    }
}

impl ::std::convert::From<graphql_parser::schema::InputObjectType> for GqlInput {
    fn from(schema_input: graphql_parser::schema::InputObjectType) -> GqlInput {
        unimplemented!();
    }
}

impl ::std::convert::From<introspection_response::FullType> for GqlInput {
    fn from(schema_input: introspection_response::FullType) -> GqlInput {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use constants::*;
    use field_type::FieldType;

    #[test]
    fn gql_input_to_rust() {
        let input = GqlInput {
            name: "Cat".to_string(),
            fields: vec![
                GqlObjectField {
                    name: "pawsCount".to_string(),
                    type_: FieldType::Named(float_type())
                },
                GqlObjectField {
                    name: "offsprings".to_string(),
                    type_: FieldType::Vector(Box::new(FieldType::Named(Ident::new("Cat", Span::call_site())))),
                },
                GqlObjectField {
                    name: "requirements".to_string(),
                    type_: FieldType::Optional(Box::new(FieldType::Named(Ident::new("CatRequirements", Span::call_site())))),
                },
            ],
        };


        let expected: String = vec![
            "",
            "",
        ].into_iter().collect();

        assert_eq!(format!("{:?}", input.to_rust()), expected);
    }
}
