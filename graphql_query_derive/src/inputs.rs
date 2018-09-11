use deprecation::DeprecationStatus;
use failure;
use graphql_parser;
use heck::SnakeCase;
use introspection_response;
use objects::GqlObjectField;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;
use std::collections::HashMap;

/// Represents an input object type from a GraphQL schema
#[derive(Debug, PartialEq)]
pub struct GqlInput {
    pub description: Option<String>,
    pub name: String,
    pub fields: HashMap<String, GqlObjectField>,
}

impl GqlInput {
    pub(crate) fn to_rust(&self, context: &QueryContext) -> Result<TokenStream, failure::Error> {
        let name = Ident::new(&self.name, Span::call_site());
        let mut fields: Vec<&GqlObjectField> = self.fields.values().collect();
        fields.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        let fields = fields.iter().map(|field| {
            let ty = field.type_.to_rust(&context, "");
            let original_name = &field.name;
            let snake_case_name = field.name.to_snake_case();
            let rename = ::shared::field_rename_annotation(&original_name, &snake_case_name);
            let name = Ident::new(&snake_case_name, Span::call_site());

            quote!(#rename pub #name: #ty)
        });

        Ok(quote! {
            #[derive(Debug, Serialize)]
            pub struct #name {
                #(#fields,)*
            }
        })
    }
}

impl ::std::convert::From<graphql_parser::schema::InputObjectType> for GqlInput {
    fn from(schema_input: graphql_parser::schema::InputObjectType) -> GqlInput {
        GqlInput {
            description: schema_input.description,
            name: schema_input.name,
            fields: schema_input
                .fields
                .into_iter()
                .map(|field| {
                    let name = field.name.clone();
                    let field = GqlObjectField {
                        description: None,
                        name: field.name,
                        type_: field.value_type.into(),
                        deprecation: DeprecationStatus::Current,
                    };
                    (name, field)
                }).collect(),
        }
    }
}

impl ::std::convert::From<introspection_response::FullType> for GqlInput {
    fn from(schema_input: introspection_response::FullType) -> GqlInput {
        GqlInput {
            description: schema_input.description,
            name: schema_input.name.expect("unnamed input object"),
            fields: schema_input
                .input_fields
                .expect("fields on input object")
                .into_iter()
                .filter_map(|a| a)
                .map(|f| {
                    let name = f.input_value.name.expect("unnamed input object field");
                    let field = GqlObjectField {
                        description: None,
                        name: name.clone(),
                        type_: f
                            .input_value
                            .type_
                            .expect("type on input object field")
                            .into(),
                        deprecation: DeprecationStatus::Current,
                    };
                    (name, field)
                }).collect(),
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
            description: None,
            name: "Cat".to_string(),
            fields: vec![
                (
                    "pawsCount".to_string(),
                    GqlObjectField {
                        description: None,
                        name: "pawsCount".to_string(),
                        type_: FieldType::Named(float_type()),
                        deprecation: DeprecationStatus::Current,
                    },
                ),
                (
                    "offsprings".to_string(),
                    GqlObjectField {
                        description: None,
                        name: "offsprings".to_string(),
                        type_: FieldType::Vector(Box::new(FieldType::Named("Cat".to_string()))),
                        deprecation: DeprecationStatus::Current,
                    },
                ),
                (
                    "requirements".to_string(),
                    GqlObjectField {
                        description: None,
                        name: "requirements".to_string(),
                        type_: FieldType::Optional(Box::new(FieldType::Named("CatRequirements".to_string()))),
                        deprecation: DeprecationStatus::Current,
                    },
                ),
            ].into_iter()
            .collect(),
        };

        let expected: String = vec![
            "# [ derive ( Debug , Serialize ) ] ",
            "pub struct Cat { ",
            "pub offsprings : Vec < Cat > , ",
            "# [ serde ( rename = \"pawsCount\" ) ] ",
            "pub paws_count : Float , ",
            "pub requirements : Option < CatRequirements > , ",
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
