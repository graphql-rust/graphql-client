use super::{EnumId, InputObjectId, InterfaceId, ObjectId, ScalarId, Schema, TypeId, UnionId};
use graphql_parser::schema::{self as parser, Definition, TypeDefinition, UnionType};

pub(super) fn build_schema(src: graphql_parser::schema::Document) -> super::Schema {
    let converter = GraphqlParserSchemaConverter {
        src,
        schema: Schema::new(),
    };

    converter.convert()
}

struct GraphqlParserSchemaConverter {
    src: graphql_parser::schema::Document,
    schema: Schema,
}

impl GraphqlParserSchemaConverter {
    // fn objects_mut(&mut self) -> impl Iterator<Item = &mut parser::ObjectType> {
    //     self.src.definitions.iter_mut().filter_map(|def| match def {
    //         Definition::TypeDefinition(TypeDefinition::Object(obj)) => Some(obj),
    //         _ => None,
    //     })
    // }

    // fn interfaces_mut(&mut self) -> impl Iterator<Item = &mut parser::InterfaceType> {
    //     self.src.definitions.iter_mut().filter_map(|def| match def {
    //         Definition::TypeDefinition(TypeDefinition::Interface(interface)) => Some(interface),
    //         _ => None,
    //     })
    // }

    // fn unions_mut(&mut self) -> impl Iterator<Item = &mut parser::UnionType> {
    //     self.src.definitions.iter_mut().filter_map(|def| match def {
    //         Definition::TypeDefinition(TypeDefinition::Union(union)) => Some(union),
    //         _ => None,
    //     })
    // }

    // fn enums_mut(&mut self) -> impl Iterator<Item = &mut parser::EnumType> {
    //     self.src.definitions.iter_mut().filter_map(|def| match def {
    //         Definition::TypeDefinition(TypeDefinition::Enum(r#enum)) => Some(r#enum),
    //         _ => None,
    //     })
    // }




    fn convert(self) -> Schema {
        let GraphqlParserSchemaConverter { src, mut schema } = self;
        populate_names_map(&mut schema, &src.definitions);
        // let definitions = &mut self.src.definitions;
        // self.src
        //     .definitions
        //     .iter_mut()
        //     .filter_map(|def| match def {
        //         Definition::TypeDefinition(TypeDefinition::Scalar(scalar)) => Some(scalar),
        //         _ => None,
        //     })
        //     .for_each(|scalar| self.ingest_graphql_parser_scalar(scalar));

        // self.enums_mut().for_each(|_| todo!());

        // self.unions_mut()
        //     .for_each(|union| self.ingest_graphql_parser_union(union));

        // self.interfaces_mut()
        //     .for_each(|iface| self.ingest_graphql_parser_interface(iface));

        // self.objects_mut()
        //     .for_each(|object| self.ingest_graphql_parser_object(object));

        // self.src
        //     .definitions
        //     .iter_mut()
        //     .filter_map(|def| match def {
        //         Definition::TypeDefinition(TypeDefinition::InputObject(input)) => Some(input),
        //         _ => None,
        //     })
        //     .for_each(|input_object| self.ingest_graphql_parser_input_object(input_object));

        // let schema_definition = self.src.definitions.iter_mut().find_map(|def| match def {
        //     Definition::SchemaDefinition(definition) => Some(definition),
        //     _ => None,
        // });

        // if let Some(schema_definition) = schema_definition {
        //     self.schema.query_type = schema_definition.query;
        //     self.schema.mutation_type = schema_definition.mutation;
        //     self.schema.subscription_type = schema_definition.subscription;
        // };

        schema
    }
}

fn populate_names_map(schema: &mut Schema, definitions: &[Definition]) {
    definitions
        .iter()
        .filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Scalar(scalar)) => {
                Some(scalar.name.as_str())
            }
            _ => None,
        })
        .enumerate()
        .for_each(|(idx, scalar_name)| {
            schema.names.insert(scalar_name.into(), TypeId::scalar(idx));
        });

    definitions
        .iter()
        .filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Enum(enm)) => Some(enm.name.as_str()),
            _ => None,
        })
        .enumerate()
        .for_each(|(idx, enum_name)| {
            schema.names.insert(enum_name.into(), TypeId::r#enum(idx));
        });

    definitions
        .iter()
        .filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Object(object)) => {
                Some(object.name.as_str())
            }
            _ => None,
        })
        .enumerate()
        .for_each(|(idx, object_name)| {
            schema
                .names
                .insert(object_name.into(), TypeId::r#object(idx));
        });

    definitions
        .iter()
        .filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Interface(interface)) => {
                Some(interface.name.as_str())
            }
            _ => None,
        })
        .enumerate()
        .for_each(|(idx, interface_name)| {
            schema
                .names
                .insert(interface_name.into(), TypeId::interface(idx));
        });

    definitions
        .iter()
        .filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Union(union)) => Some(union.name.as_str()),
            _ => None,
        })
        .enumerate()
        .for_each(|(idx, union_name)| {
            schema.names.insert(union_name.into(), TypeId::union(idx));
        });

    definitions
        .iter()
        .filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::InputObject(input)) => {
                Some(input.name.as_str())
            }
            _ => None,
        })
        .enumerate()
        .for_each(|(idx, input_name)| {
            schema.names.insert(input_name.into(), TypeId::input(idx));
        });
}

fn ingest_union(schema: &mut Schema, union: &mut graphql_parser::schema::UnionType) {
    let stored_union = super::StoredUnion {
        name: std::mem::replace(&mut union.name, String::new()),
        variants: union
            .types
            .iter()
            .map(|name| schema.find_type_id(name))
            .collect(),
    };

    schema.stored_unions.push(stored_union);
}

fn ingest_object(schema: &mut Schema, obj: &mut graphql_parser::schema::ObjectType) {
    // Ingest the object itself
    let object = super::StoredObject {
        name: std::mem::replace(&mut obj.name, String::new()),
        fields: obj
            .fields
            .iter_mut()
            .map(|graphql_field| super::StoredField {
                name: std::mem::replace(&mut graphql_field.name, String::new()),
                r#type: resolve_field_type(schema, &graphql_field.field_type)
            })
            .collect(),
        implements_interfaces: obj
            .implements_interfaces
            .iter()
            .map(|iface_name| schema.find_interface(iface_name))
            .collect(),
    };

    let object_id = schema.push_object(object);

    todo!()

    // // Ingest fields
    // for graphql_field in &mut obj.fields {
    //     let field = super::StoredObjectField { object: object_id };

    //     let field_id = schema.push_object_field(field);

    //     schema.get_object_mut(object_id).fields.push(field_id);
    // }

    // TODO: this assumes we ingest interfaces before objects
    // for id in schema
    //     .get_object_mut(object_id)
    //     .implements_interfaces
    //     .iter()
    // {
    //     schema.get_interface_mut(*id).implemented_by.push(object_id);
    // }
}

fn resolve_field_type(
    schema: &mut Schema,
    inner: &graphql_parser::schema::Type,
) -> super::StoredFieldType {
    use crate::field_type::{graphql_parser_depth, GraphqlTypeQualifier};
    use graphql_parser::schema::Type::*;

    let qualifiers_depth = graphql_parser_depth(inner);
    let mut qualifiers = Vec::with_capacity(qualifiers_depth);

    let mut inner = inner;

    loop {
        match inner {
            ListType(new_inner) => {
                qualifiers.push(GraphqlTypeQualifier::List);
                inner = new_inner;
            }
            NonNullType(new_inner) => {
                qualifiers.push(GraphqlTypeQualifier::Required);
                inner = new_inner;
            }
            NamedType(name) => {
                return super::StoredFieldType {
                    id: schema.find_type_id(name),
                    qualifiers,
                }
            }
        }
    }
}

fn ingest_scalar(schema: &mut Schema, scalar: &mut graphql_parser::schema::ScalarType) {
    let scalar = super::StoredScalar {
        name: std::mem::replace(&mut scalar.name, String::new()),
    };

    schema.push_scalar(scalar);
}

fn ingest_interface(
    schema: &mut Schema,
    interface: &mut graphql_parser::schema::InterfaceType,
) {
    let new_interface = super::StoredInterface {
        name: std::mem::replace(&mut interface.name, String::new()),
        fields: todo!(),
        implemented_by: Vec::new(),
    };

    let interface_id = schema.push_interface(new_interface);

    // for field in &mut interface.fields {
    //     let interface_field = super::StoredInterfaceField {
    //         name: std::mem::replace(&mut field.name, String::new()),
    //         interface: interface_id,
    //         r#type: self.resolve_field_type(&field.field_type),
    //     };
    //     let field_id = schema.push_interface_field(interface_field);

    //     schema
    //         .get_interface_mut(interface_id)
    //         .fields
    //         .push(field_id);
    // }
}
