use enums::GqlEnum;
use failure;
use field_type::FieldType;
use graphql_parser::{self, query, schema};
use inputs::GqlInput;
use interfaces::GqlInterface;
use objects::{GqlObject, GqlObjectField};
use proc_macro2::TokenStream;
use query::QueryContext;
use std::collections::{BTreeMap, BTreeSet};
use unions::GqlUnion;

#[derive(Debug)]
pub struct Schema {
    pub enums: BTreeMap<String, GqlEnum>,
    pub inputs: BTreeMap<String, GqlInput>,
    pub interfaces: BTreeMap<String, GqlInterface>,
    pub objects: BTreeMap<String, GqlObject>,
    pub scalars: BTreeSet<String>,
    pub unions: BTreeMap<String, GqlUnion>,
    pub query_type: Option<String>,
    pub mutation_type: Option<String>,
    pub subscription_type: Option<String>,
}

impl Schema {
    pub fn new() -> Schema {
        Schema {
            enums: BTreeMap::new(),
            inputs: BTreeMap::new(),
            interfaces: BTreeMap::new(),
            objects: BTreeMap::new(),
            scalars: BTreeSet::new(),
            unions: BTreeMap::new(),
            query_type: None,
            mutation_type: None,
            subscription_type: None,
        }
    }

    pub fn response_for_query(self, query: query::Document) -> Result<TokenStream, failure::Error> {
        let mut context = QueryContext::new(self);
        let mut definitions = Vec::new();

        for definition in query.definitions {
            match definition {
                query::Definition::Operation(query::OperationDefinition::Query(q)) => {
                    let definition = context
                        .schema
                        .query_type
                        .clone()
                        .and_then(|query_type| context.schema.objects.get(&query_type))
                        .expect("query type is defined");
                    let prefix = &q.name.expect("unnamed operation");
                    definitions.extend(definition.field_impls_for_selection(
                        &context,
                        &q.selection_set,
                        prefix,
                    ));
                    context.query_root = Some(definition.response_fields_for_selection(
                        &context,
                        &q.selection_set,
                        prefix,
                    ));
                }
                query::Definition::Operation(query::OperationDefinition::Mutation(q)) => {
                    let definition = context
                        .schema
                        .mutation_type
                        .clone()
                        .and_then(|mutation_type| context.schema.objects.get(&mutation_type))
                        .expect("mutation type is defined");
                    let prefix = &q.name.expect("unnamed operation");

                    definitions.extend(definition.field_impls_for_selection(
                        &context,
                        &q.selection_set,
                        prefix,
                    ));
                    context.mutation_root = Some(definition.response_fields_for_selection(
                        &context,
                        &q.selection_set,
                        prefix,
                    ));
                }
                query::Definition::Operation(query::OperationDefinition::Subscription(q)) => {
                    let definition = context
                        .schema
                        .subscription_type
                        .clone()
                        .and_then(|subscription_type| {
                            context.schema.objects.get(&subscription_type)
                        })
                        .expect("subscription type is defined");
                    let prefix = &q.name.expect("unnamed operation");
                    definitions.extend(definition.field_impls_for_selection(
                        &context,
                        &q.selection_set,
                        &prefix,
                    ));
                    context._subscription_root = Some(definition.response_fields_for_selection(
                        &context,
                        &q.selection_set,
                        &prefix,
                    ));
                }
                query::Definition::Operation(query::OperationDefinition::SelectionSet(_)) => {
                    unimplemented!()
                }
                query::Definition::Fragment(fragment) => {
                    context.fragments.insert(fragment.name, BTreeMap::new());
                }
            }
        }

        let enum_definitions = context.schema.enums.values().map(|enm| enm.to_rust());
        let variables_struct = quote!(#[derive(Serialize)]
        pub struct Variables;);
        let response_data_fields = context
            .query_root
            .or(context.mutation_root)
            .or(context._subscription_root)
            .expect("no selection defined");

        use proc_macro2::{Ident, Span};

        Ok(quote! {
            #(#enum_definitions)*

            #(#definitions)*

            #variables_struct

            #[derive(Deserialize)]
            pub struct ResponseData {
                #(#response_data_fields)*,
            }

        })
    }
}

impl ::std::convert::From<graphql_parser::schema::Document> for Schema {
    fn from(ast: graphql_parser::schema::Document) -> Schema {
        let mut schema = Schema::new();

        // Holds which objects implement which interfaces so we can populate GqlInterface#implemented_by later.
        // It maps interface names to a vec of implementation names.
        let mut interface_implementations: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for definition in ast.definitions {
            match definition {
                schema::Definition::TypeDefinition(ty_definition) => match ty_definition {
                    schema::TypeDefinition::Object(obj) => {
                        for implementing in obj.implements_interfaces.iter() {
                            let name = &obj.name;
                            interface_implementations
                                .entry(implementing.to_string())
                                .and_modify(|objects| objects.push(name.clone()))
                                .or_insert_with(|| vec![name.clone()]);
                        }

                        schema.objects.insert(
                            obj.name.clone(),
                            GqlObject {
                                name: obj.name.clone(),
                                fields: obj
                                    .fields
                                    .iter()
                                    .map(|f| GqlObjectField {
                                        name: f.name.clone(),
                                        type_: FieldType::from(f.field_type.clone()),
                                    })
                                    .collect(),
                            },
                        );
                    }
                    schema::TypeDefinition::Enum(enm) => {
                        schema.enums.insert(
                            enm.name.clone(),
                            GqlEnum {
                                name: enm.name.clone(),
                                variants: enm.values.iter().map(|v| v.name.clone()).collect(),
                            },
                        );
                    }
                    schema::TypeDefinition::Scalar(scalar) => {
                        schema.scalars.insert(scalar.name);
                    }
                    schema::TypeDefinition::Union(union) => {
                        schema.unions.insert(union.name, GqlUnion(union.types));
                    }
                    schema::TypeDefinition::Interface(interface) => {
                        schema.interfaces.insert(
                            interface.name.clone(),
                            GqlInterface {
                                name: interface.name,
                                implemented_by: Vec::new(),
                                fields: interface
                                    .fields
                                    .iter()
                                    .map(|f| GqlObjectField {
                                        name: f.name.clone(),
                                        type_: FieldType::from(f.field_type.clone()),
                                    })
                                    .collect(),
                            },
                        );
                    }
                    schema::TypeDefinition::InputObject(input) => {
                        schema.inputs.insert(input.name, GqlInput);
                    }
                },
                schema::Definition::DirectiveDefinition(_) => (),
                schema::Definition::TypeExtension(_extension) => (),
                schema::Definition::SchemaDefinition(definition) => {
                    schema.query_type = definition.query;
                    schema.mutation_type = definition.mutation;
                    schema.subscription_type = definition.subscription;
                }
            }
        }

        schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::{Ident, Span};

    #[test]
    fn build_schema_works() {
        let gql_schema = include_str!("star_wars_schema.graphql");
        let gql_schema = graphql_parser::parse_schema(gql_schema).unwrap();
        let built = Schema::from(gql_schema);
        assert_eq!(
            built.objects.get("Droid"),
            Some(&GqlObject {
                name: "Droid".to_string(),
                fields: vec![
                    GqlObjectField {
                        name: "id".to_string(),
                        type_: FieldType::Optional(Box::new(FieldType::Named(Ident::new(
                            "ID",
                            Span::call_site(),
                        )))),
                    },
                    GqlObjectField {
                        name: "name".to_string(),
                        type_: FieldType::Optional(Box::new(FieldType::Named(Ident::new(
                            "String",
                            Span::call_site(),
                        )))),
                    },
                    GqlObjectField {
                        name: "friends".to_string(),
                        type_: FieldType::Optional(Box::new(FieldType::Vector(Box::new(
                            FieldType::Optional(Box::new(FieldType::Named(Ident::new(
                                "Character",
                                Span::call_site(),
                            )))),
                        )))),
                    },
                    GqlObjectField {
                        name: "friendsConnection".to_string(),
                        type_: FieldType::Optional(Box::new(FieldType::Named(Ident::new(
                            "FriendsConnection",
                            Span::call_site(),
                        )))),
                    },
                    GqlObjectField {
                        name: "appearsIn".to_string(),
                        type_: FieldType::Optional(Box::new(FieldType::Vector(Box::new(
                            FieldType::Optional(Box::new(FieldType::Named(Ident::new(
                                "Episode",
                                Span::call_site(),
                            )))),
                        )))),
                    },
                    GqlObjectField {
                        name: "primaryFunction".to_string(),
                        type_: FieldType::Optional(Box::new(FieldType::Named(Ident::new(
                            "String",
                            Span::call_site(),
                        )))),
                    },
                ],
            })
        )
    }
}
