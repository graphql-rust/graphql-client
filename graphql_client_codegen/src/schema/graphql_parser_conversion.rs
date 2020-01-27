use super::{EnumId, InputObjectId, InterfaceId, ObjectId, ScalarId, Schema, TypeId, UnionId};
use graphql_parser::schema::{self as parser, Definition, Document, TypeDefinition, UnionType};

pub(super) fn build_schema(mut src: graphql_parser::schema::Document) -> super::Schema {
    let mut schema = Schema::new();
    convert(&mut src, &mut schema);
    schema
}

fn convert(src: &mut graphql_parser::schema::Document, schema: &mut Schema) {
    populate_names_map(schema, &src.definitions);

    src.definitions
        .iter_mut()
        .filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Scalar(scalar)) => Some(scalar),
            _ => None,
        })
        .for_each(|scalar| ingest_scalar(schema, scalar));

    enums_mut(src).for_each(|enm| ingest_enum(schema, enm));

    unions_mut(src).for_each(|union| ingest_union(schema, union));

    interfaces_mut(src).for_each(|iface| ingest_interface(schema, iface));

    objects_mut(src).for_each(|obj| ingest_object(schema, obj));

    inputs_mut(src).for_each(|input| ingest_input(schema, input));

    let schema_definition = src.definitions.iter_mut().find_map(|def| match def {
        Definition::SchemaDefinition(definition) => Some(definition),
        _ => None,
    });

    if let Some(schema_definition) = schema_definition {
        schema.query_type = schema_definition
            .query
            .as_mut()
            .and_then(|n| schema.names.get(n))
            .and_then(|id| id.as_object_id());
        schema.mutation_type = schema_definition
            .mutation
            .as_mut()
            .and_then(|n| schema.names.get(n))
            .and_then(|id| id.as_object_id());
        schema.subscription_type = schema_definition
            .subscription
            .as_mut()
            .and_then(|n| schema.names.get(n))
            .and_then(|id| id.as_object_id());
    };
}

fn populate_names_map(schema: &mut Schema, definitions: &[Definition]) {
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
    let object_id = schema.find_type_id(&obj.name).as_object_id().unwrap();
    let mut field_ids = Vec::with_capacity(obj.fields.len());

    for field in obj.fields.iter_mut() {
        let field = super::StoredField {
            name: std::mem::replace(&mut field.name, String::new()),
            r#type: resolve_field_type(schema, &field.field_type),
            parent: super::StoredFieldParent::Object(object_id),
        };

        field_ids.push(schema.push_field(field));
    }


    // Ingest the object itself
    let object = super::StoredObject {
        name: std::mem::replace(&mut obj.name, String::new()),
        fields: field_ids,
        implements_interfaces: obj
            .implements_interfaces
            .iter()
            .map(|iface_name| schema.find_interface(iface_name))
            .collect(),
    };

    schema.push_object(object);
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
    let name = std::mem::replace(&mut scalar.name, String::new());
    let name_for_names = name.clone();

    let scalar = super::StoredScalar { name };

    let scalar_id = schema.push_scalar(scalar);

    schema
        .names
        .insert(name_for_names, TypeId::Scalar(scalar_id));
}

fn ingest_enum(schema: &mut Schema, enm: &mut graphql_parser::schema::EnumType) {
    let enm = super::StoredEnum {
        name: std::mem::replace(&mut enm.name, String::new()),
        variants: enm
            .values
            .iter_mut()
            .map(|value| std::mem::replace(&mut value.name, String::new()))
            .collect(),
    };

    schema.push_enum(enm);
}

fn ingest_interface(schema: &mut Schema, interface: &mut graphql_parser::schema::InterfaceType) {
    let interface_id = schema
        .find_type_id(&interface.name)
        .as_interface_id()
        .unwrap();

        let mut field_ids = Vec::with_capacity(interface.fields.len());

    for field in interface.fields.iter_mut() {
        let field = super::StoredField {
            name: std::mem::replace(&mut field.name, String::new()),
            r#type: resolve_field_type(schema, &field.field_type),
            parent: super::StoredFieldParent::Interface(interface_id),
        };

        field_ids.push(schema.push_field(field));
    }

    let new_interface = super::StoredInterface {
        name: std::mem::replace(&mut interface.name, String::new()),
        fields: field_ids,
    };

    schema.push_interface(new_interface);
}

fn ingest_input(schema: &mut Schema, input: &mut parser::InputObjectType) {
    unimplemented!()
}

fn objects_mut(doc: &mut Document) -> impl Iterator<Item = &mut parser::ObjectType> {
    doc.definitions.iter_mut().filter_map(|def| match def {
        Definition::TypeDefinition(TypeDefinition::Object(obj)) => Some(obj),
        _ => None,
    })
}

fn interfaces_mut(doc: &mut Document) -> impl Iterator<Item = &mut parser::InterfaceType> {
    doc.definitions.iter_mut().filter_map(|def| match def {
        Definition::TypeDefinition(TypeDefinition::Interface(interface)) => Some(interface),
        _ => None,
    })
}

fn unions_mut(doc: &mut Document) -> impl Iterator<Item = &mut parser::UnionType> {
    doc.definitions.iter_mut().filter_map(|def| match def {
        Definition::TypeDefinition(TypeDefinition::Union(union)) => Some(union),
        _ => None,
    })
}

fn enums_mut(doc: &mut Document) -> impl Iterator<Item = &mut parser::EnumType> {
    doc.definitions.iter_mut().filter_map(|def| match def {
        Definition::TypeDefinition(TypeDefinition::Enum(r#enum)) => Some(r#enum),
        _ => None,
    })
}

fn inputs_mut(doc: &mut Document) -> impl Iterator<Item = &mut parser::InputObjectType> {
    doc.definitions.iter_mut().filter_map(|def| match def {
        Definition::TypeDefinition(TypeDefinition::InputObject(input)) => Some(input),
        _ => None,
    })
}
