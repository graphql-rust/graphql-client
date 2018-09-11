use deprecation::DeprecationStatus;
use enums::{EnumVariant, GqlEnum};
use failure;
use field_type::FieldType;
use graphql_parser::{self, schema};
use inputs::GqlInput;
use interfaces::GqlInterface;
use objects::{GqlObject, GqlObjectField};
use scalars::Scalar;
use std::collections::{BTreeMap, BTreeSet};
use unions::GqlUnion;

pub const DEFAULT_SCALARS: &[&str] = &["ID", "String", "Int", "Float", "Boolean"];

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub enums: BTreeMap<String, GqlEnum>,
    pub inputs: BTreeMap<String, GqlInput>,
    pub interfaces: BTreeMap<String, GqlInterface>,
    pub objects: BTreeMap<String, GqlObject>,
    pub scalars: BTreeMap<String, Scalar>,
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
            scalars: BTreeMap::new(),
            unions: BTreeMap::new(),
            query_type: None,
            mutation_type: None,
            subscription_type: None,
        }
    }

    pub fn ingest_interface_implementations(
        &mut self,
        impls: BTreeMap<String, Vec<String>>,
    ) -> Result<(), failure::Error> {
        impls
            .into_iter()
            .map(|(iface_name, implementors)| {
                let iface = self
                    .interfaces
                    .get_mut(&iface_name)
                    .ok_or_else(|| format_err!("interface not found: {}", iface_name))?;
                iface.implemented_by = implementors.into_iter().collect();
                Ok(())
            }).collect()
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
                        for implementing in &obj.implements_interfaces {
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
                                description: enm.description,
                                variants: enm
                                    .values
                                    .iter()
                                    .map(|v| EnumVariant {
                                        description: v.description.clone(),
                                        name: v.name.clone(),
                                    }).collect(),
                            },
                        );
                    }
                    schema::TypeDefinition::Scalar(scalar) => {
                        schema.scalars.insert(
                            scalar.name.clone(),
                            Scalar {
                                name: scalar.name,
                                description: scalar.description,
                            },
                        );
                    }
                    schema::TypeDefinition::Union(union) => {
                        let variants: BTreeSet<String> = union.types.into_iter().collect();
                        schema.unions.insert(
                            union.name,
                            GqlUnion {
                                variants,
                                description: union.description,
                            },
                        );
                    }
                    schema::TypeDefinition::Interface(interface) => {
                        let mut iface = GqlInterface::new(
                            interface.name.clone().into(),
                            interface.description.as_ref().map(|d| d.as_str()),
                        );
                        iface
                            .fields
                            .extend(interface.fields.iter().map(|f| GqlObjectField {
                                description: f.description.as_ref().map(|s| s.to_owned()),
                                name: f.name.clone(),
                                type_: FieldType::from(f.field_type.clone()),
                                deprecation: DeprecationStatus::Current,
                            }));
                        schema.interfaces.insert(interface.name, iface);
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
            .ingest_interface_implementations(interface_implementations)
            .expect("schema ingestion");

        schema
    }
}

impl ::std::convert::From<::introspection_response::IntrospectionResponse> for Schema {
    fn from(src: ::introspection_response::IntrospectionResponse) -> Self {
        use introspection_response::__TypeKind;

        let mut schema = Schema::new();
        let root = src.schema.expect("__schema is not null");

        schema.query_type = root.query_type.and_then(|ty| ty.name);
        schema.mutation_type = root.mutation_type.and_then(|ty| ty.name);
        schema.subscription_type = root.subscription_type.and_then(|ty| ty.name);

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
                    let variants: Vec<EnumVariant> = ty
                        .enum_values
                        .clone()
                        .expect("enum variants")
                        .iter()
                        .map(|t| {
                            t.clone().map(|t| EnumVariant {
                                description: t.description,
                                name: t.name.expect("enum variant name"),
                            })
                        }).filter_map(|t| t)
                        .collect();
                    let mut enm = GqlEnum {
                        name: name.clone(),
                        description: ty.description.clone(),
                        variants,
                    };
                    schema.enums.insert(name, enm);
                }
                Some(__TypeKind::SCALAR) => {
                    if DEFAULT_SCALARS
                        .iter()
                        .find(|s| s == &&name.as_str())
                        .is_none()
                    {
                        schema.scalars.insert(
                            name.clone(),
                            Scalar {
                                name,
                                description: ty.description.as_ref().map(|d| d.clone()),
                            },
                        );
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
                    schema.unions.insert(
                        name.clone(),
                        GqlUnion {
                            description: ty.description.as_ref().map(|d| d.to_owned()),
                            variants,
                        },
                    );
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
                    let mut iface = GqlInterface::new(
                        name.clone().into(),
                        ty.description.as_ref().map(|t| t.as_str()),
                    );
                    iface.fields.extend(
                        ty.fields
                            .clone()
                            .expect("interface fields")
                            .into_iter()
                            .filter_map(|f| f)
                            .map(|f| GqlObjectField {
                                description: f.description,
                                name: f.name.expect("field name"),
                                type_: FieldType::from(f.type_.expect("field type")),
                                deprecation: DeprecationStatus::Current,
                            }),
                    );
                    schema.interfaces.insert(name, iface);
                }
                Some(__TypeKind::INPUT_OBJECT) => {
                    schema.inputs.insert(name, GqlInput::from(ty.clone()));
                }
                _ => unimplemented!("unimplemented definition"),
            }
        }

        schema
            .ingest_interface_implementations(interface_implementations)
            .expect("schema ingestion");

        schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use constants::*;

    #[test]
    fn build_schema_works() {
        let gql_schema = include_str!("tests/star_wars_schema.graphql");
        let gql_schema = graphql_parser::parse_schema(gql_schema).unwrap();
        let built = Schema::from(gql_schema);
        assert_eq!(
            built.objects.get("Droid"),
            Some(&GqlObject {
                description: None,
                name: "Droid".to_string(),
                fields: vec![
                    GqlObjectField {
                        description: None,
                        name: TYPENAME_FIELD.to_string(),
                        type_: FieldType::Named(string_type()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "id".to_string(),
                        type_: FieldType::Named("ID".to_string()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "name".to_string(),
                        type_: FieldType::Named("String".to_string()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "friends".to_string(),
                        type_: FieldType::Optional(Box::new(FieldType::Vector(Box::new(
                            FieldType::Optional(Box::new(FieldType::Named("Character".to_string()))),
                        )))),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "friendsConnection".to_string(),
                        type_: FieldType::Named("FriendsConnection".to_string()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "appearsIn".to_string(),
                        type_: FieldType::Vector(Box::new(FieldType::Optional(Box::new(
                            FieldType::Named("Episode".to_string()),
                        )))),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "primaryFunction".to_string(),
                        type_: FieldType::Optional(Box::new(FieldType::Named("String".to_string()))),
                        deprecation: DeprecationStatus::Current,
                    },
                ],
            })
        )
    }
}
