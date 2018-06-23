use failure;
use graphql_parser;
use introspection_response;
use objects::GqlObjectField;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct GqlInput {
    pub name: String,
    pub fields: HashMap<String, GqlObjectField>,
}

impl GqlInput {
    pub fn to_rust(&self, context: &QueryContext) -> Result<TokenStream, failure::Error> {
        let name = Ident::new(&self.name, Span::call_site());
        let mut fields: Vec<&GqlObjectField> = self.fields.values().collect();
        fields.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        let fields = fields.iter().map(|field| {
            let ty = field.type_.to_rust(&context, "");
            let name = Ident::new(&field.name, Span::call_site());
            quote!(#name: #ty)
        });

        Ok(quote! {
            #[derive(Debug, Serialize)]
            #[serde(rename_all = "camelCase")]
            pub struct #name {
                #(#fields,)*
            }
        })
    }
}

impl ::std::convert::From<graphql_parser::schema::InputObjectType> for GqlInput {
    fn from(schema_input: graphql_parser::schema::InputObjectType) -> GqlInput {
        GqlInput {
            name: schema_input.name,
            fields: schema_input
                .fields
                .into_iter()
                .map(|field| {
                    let name = field.name.clone();
                    let field = GqlObjectField {
                        name: field.name,
                        type_: field.value_type.into(),
                    };
                    (name, field)
                })
                .collect(),
        }
    }
}

impl ::std::convert::From<introspection_response::FullType> for GqlInput {
    fn from(schema_input: introspection_response::FullType) -> GqlInput {
        GqlInput {
            name: schema_input.name.expect("unnamed input object"),
            fields: schema_input
                .input_fields
                .expect("fields on input object")
                .into_iter()
                .filter_map(|a| a)
                .map(|f| {
                    let name = f.input_value.name.expect("unnamed input object field");
                    let field = GqlObjectField {
                        name: name.clone(),
                        type_: f
                            .input_value
                            .type_
                            .expect("type on input object field")
                            .into(),
                    };
                    (name, field)
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use constants::*;
    use field_type::FieldType;

    #[test]
    fn gql_input_to_rust() {
        let cat = GqlInput {
            name: "Cat".to_string(),
            fields: vec![
                (
                    "pawsCount".to_string(),
                    GqlObjectField {
                        name: "pawsCount".to_string(),
                        type_: FieldType::Named(float_type()),
                    },
                ),
                (
                    "offsprings".to_string(),
                    GqlObjectField {
                        name: "offsprings".to_string(),
                        type_: FieldType::Vector(Box::new(FieldType::Named(Ident::new(
                            "Cat",
                            Span::call_site(),
                        )))),
                    },
                ),
                (
                    "requirements".to_string(),
                    GqlObjectField {
                        name: "requirements".to_string(),
                        type_: FieldType::Optional(Box::new(FieldType::Named(Ident::new(
                            "CatRequirements",
                            Span::call_site(),
                        )))),
                    },
                ),
            ].into_iter()
                .collect(),
        };

        let expected: String = vec![
            "# [ derive ( Debug , Serialize ) ] ",
            "# [ serde ( rename_all = \"camelCase\" ) ] ",
            "pub struct Cat { ",
            "offsprings : Vec < Cat > , ",
            "pawsCount : Float , ",
            "requirements : Option < CatRequirements > , ",
            "}",
        ].into_iter()
            .collect();

        let mut context = QueryContext::new_empty();
        context.schema.inputs.insert(cat.name.clone(), cat);

        assert_eq!(
            format!(
                "{}",
                context
                    .schema
                    .inputs
                    .get("Cat")
                    .unwrap()
                    .to_rust(&context)
                    .unwrap()
            ),
            expected
        );
    }
}
