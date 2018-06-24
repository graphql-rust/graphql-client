use enums::GqlEnum;
use failure;
use field_type::FieldType;
use fragments::GqlFragment;
use graphql_parser::{self, query, schema};
use inputs::GqlInput;
use interfaces::GqlInterface;
use objects::{GqlObject, GqlObjectField};
use proc_macro2::TokenStream;
use query::QueryContext;
use selection::Selection;
use std::collections::{BTreeMap, BTreeSet};
use unions::GqlUnion;

pub const DEFAULT_SCALARS: &[&'static str] = &["ID", "String", "Int", "Float", "Boolean"];

#[derive(Debug, PartialEq)]
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
                    context.query_root = {
                        let definition = context
                            .schema
                            .query_type
                            .clone()
                            .and_then(|query_type| context.schema.objects.get(&query_type))
                            .expect("query type is defined");
                        let prefix = &q.name.expect("unnamed operation");
                        let prefix = format!("RUST_{}", prefix);
                        let selection = Selection::from(&q.selection_set);

                        definitions.extend(
                            definition.field_impls_for_selection(&context, &selection, &prefix)?,
                        );
                        Some(
                            definition.response_fields_for_selection(&context, &selection, &prefix),
                        )
                    };

                    context.register_variables(&q.variable_definitions);
                }
                query::Definition::Operation(query::OperationDefinition::Mutation(q)) => {
                    context.mutation_root = {
                        let definition = context
                            .schema
                            .mutation_type
                            .clone()
                            .and_then(|mutation_type| context.schema.objects.get(&mutation_type))
                            .expect("mutation type is defined");
                        let prefix = &q.name.expect("unnamed operation");
                        let prefix = format!("RUST_{}", prefix);
                        let selection = Selection::from(&q.selection_set);

                        definitions.extend(
                            definition.field_impls_for_selection(&context, &selection, &prefix)?,
                        );
                        Some(
                            definition.response_fields_for_selection(&context, &selection, &prefix),
                        )
                    };

                    context.register_variables(&q.variable_definitions);
                }
                query::Definition::Operation(query::OperationDefinition::Subscription(q)) => {
                    context._subscription_root = {
                        let definition = context
                            .schema
                            .subscription_type
                            .clone()
                            .and_then(|subscription_type| {
                                context.schema.objects.get(&subscription_type)
                            })
                            .expect("subscription type is defined");
                        let prefix = &q.name.expect("unnamed operation");
                        let prefix = format!("RUST_{}", prefix);
                        let selection = Selection::from(&q.selection_set);

                        definitions.extend(
                            definition.field_impls_for_selection(&context, &selection, &prefix)?,
                        );
                        Some(
                            definition.response_fields_for_selection(&context, &selection, &prefix),
                        )
                    };

                    context.register_variables(&q.variable_definitions);
                }
                query::Definition::Operation(query::OperationDefinition::SelectionSet(_)) => {
                    unimplemented!()
                }
                query::Definition::Fragment(fragment) => {
                    let query::TypeCondition::On(on) = fragment.type_condition;
                    context.fragments.insert(
                        fragment.name.clone(),
                        GqlFragment {
                            name: fragment.name,
                            selection: Selection::from(&fragment.selection_set),
                            on,
                        },
                    );
                }
            }
        }

        let enum_definitions = context.schema.enums.values().map(|enm| enm.to_rust());
        let fragment_definitions = context
            .fragments
            .values()
            .map(|fragment| fragment.to_rust(&context));
        let variables_struct = context.expand_variables();
        let response_data_fields = context
            .query_root
            .as_ref()
            .or(context.mutation_root.as_ref())
            .or(context._subscription_root.as_ref())
            .expect("no selection defined");

        // TODO: do something smarter here
        let scalar_definitions = context.schema.scalars.iter().map(|scalar_name| {
            use proc_macro2::{Ident, Span};
            let ident = Ident::new(scalar_name, Span::call_site());
            quote!(type #ident = String;)
        });

        let input_object_definitions: Result<Vec<TokenStream>, _> = context
            .schema
            .inputs
            .values()
            .map(|i| i.to_rust(&context))
            .collect();
        let input_object_definitions = input_object_definitions?;

        Ok(quote! {
            type Boolean = bool;
            type Float = f64;
            type Int = i64;
            type ID = String;

            #(#scalar_definitions)*

            #(#input_object_definitions)*

            #(#enum_definitions)*

            #(#fragment_definitions)*

            #(#definitions)*

            #variables_struct

            #[derive(Debug, Serialize, Deserialize)]
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

                        schema
                            .objects
                            .insert(obj.name.clone(), GqlObject::from_graphql_parser_object(obj));
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
                        let tys: BTreeSet<String> = union.types.into_iter().collect();
                        schema.unions.insert(union.name, GqlUnion(tys));
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
                        schema
                            .inputs
                            .insert(input.name.clone(), GqlInput::from(input));
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

impl ::std::convert::From<::introspection_response::IntrospectionResponse> for Schema {
    fn from(src: ::introspection_response::IntrospectionResponse) -> Self {
        use introspection_response::__TypeKind;

        let mut schema = Schema::new();
        let root = src.schema.expect("__Schema is not null");

        // Holds which objects implement which interfaces so we can populate GqlInterface#implemented_by later.
        // It maps interface names to a vec of implementation names.
        let mut interface_implementations: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for ty in root
            .types
            .expect("types in schema")
            .iter()
            .filter_map(|t| t.as_ref().map(|t| &t.full_type))
        {
            let name = ty.name.clone().expect("type definition name");

            match ty.kind {
                Some(__TypeKind::ENUM) => {
                    let variants: Vec<String> = ty
                        .enum_values
                        .clone()
                        .expect("enum variants")
                        .iter()
                        .map(|t| t.clone().map(|t| t.name.expect("enum variant name")))
                        .filter_map(|t| t)
                        .collect();
                    schema
                        .enums
                        .insert(name.clone(), GqlEnum { name, variants });
                }
                Some(__TypeKind::SCALAR) => {
                    if DEFAULT_SCALARS
                        .iter()
                        .find(|s| s == &&name.as_str())
                        .is_none()
                    {
                        schema.scalars.insert(name);
                    }
                }
                Some(__TypeKind::UNION) => {
                    let variants: BTreeSet<String> = ty
                        .possible_types
                        .clone()
                        .unwrap()
                        .into_iter()
                        .filter_map(|t| t.and_then(|t| t.type_ref.name.clone()))
                        .collect();
                    schema.unions.insert(name.clone(), GqlUnion(variants));
                }
                Some(__TypeKind::OBJECT) => {
                    for implementing in ty
                        .interfaces
                        .clone()
                        .unwrap_or_else(|| Vec::new())
                        .into_iter()
                        .filter_map(|t| t)
                        .map(|t| t.type_ref.name)
                    {
                        interface_implementations
                            .entry(implementing.expect("interface name"))
                            .and_modify(|objects| objects.push(name.clone()))
                            .or_insert_with(|| vec![name.clone()]);
                    }

                    schema
                        .objects
                        .insert(name.clone(), GqlObject::from_introspected_schema_json(ty));
                }
                Some(__TypeKind::INTERFACE) => {
                    let iface = GqlInterface {
                        name: name.clone(),
                        implemented_by: Vec::new(),
                        fields: ty
                            .fields
                            .clone()
                            .expect("interface fields")
                            .into_iter()
                            .filter_map(|f| f)
                            .map(|f| GqlObjectField {
                                name: f.name.expect("field name"),
                                type_: FieldType::from(f.type_.expect("field type")),
                            })
                            .collect(),
                    };
                    schema.interfaces.insert(name, iface);
                }
                Some(__TypeKind::INPUT_OBJECT) => {
                    schema.inputs.insert(name, GqlInput::from(ty.clone()));
                }
                _ => unimplemented!("unimplemented definition"),
            }
        }

        schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use constants::*;
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
                        name: TYPENAME_FIELD.to_string(),
                        type_: FieldType::Named(string_type()),
                    },
                    GqlObjectField {
                        name: "id".to_string(),
                        type_: FieldType::Named(Ident::new("ID", Span::call_site())),
                    },
                    GqlObjectField {
                        name: "name".to_string(),
                        type_: FieldType::Named(Ident::new("String", Span::call_site())),
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
                        type_: FieldType::Named(Ident::new("FriendsConnection", Span::call_site())),
                    },
                    GqlObjectField {
                        name: "appearsIn".to_string(),
                        type_: FieldType::Vector(Box::new(FieldType::Optional(Box::new(
                            FieldType::Named(Ident::new("Episode", Span::call_site())),
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
