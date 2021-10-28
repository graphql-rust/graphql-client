use super::{Schema, StoredInputFieldType, TypeId};
use crate::schema::resolve_field_type;
use graphql_parser::schema::{self as parser, Definition, Document, TypeDefinition, UnionType};

pub(super) fn build_schema<'doc, T>(
    mut src: graphql_parser::schema::Document<'doc, T>,
) -> super::Schema
where
    T: graphql_parser::query::Text<'doc>,
{
    let mut schema = Schema::new();
    convert(&mut src, &mut schema);
    schema
}

fn convert<'a, 'doc: 'a, T>(
    src: &'a mut graphql_parser::schema::Document<'doc, T>,
    schema: &mut Schema,
) where
    T: graphql_parser::query::Text<'doc>,
{
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
            .and_then(|n| schema.names.get(n.as_ref()))
            .and_then(|id| id.as_object_id());
        schema.mutation_type = schema_definition
            .mutation
            .as_mut()
            .and_then(|n| schema.names.get(n.as_ref()))
            .and_then(|id| id.as_object_id());
        schema.subscription_type = schema_definition
            .subscription
            .as_mut()
            .and_then(|n| schema.names.get(n.as_ref()))
            .and_then(|id| id.as_object_id());
    } else {
        schema.query_type = schema.names.get("Query").and_then(|id| id.as_object_id());

        schema.mutation_type = schema
            .names
            .get("Mutation")
            .and_then(|id| id.as_object_id());

        schema.subscription_type = schema
            .names
            .get("Subscription")
            .and_then(|id| id.as_object_id());
    };
}

fn populate_names_map<'a, T>(schema: &mut Schema, definitions: &[Definition<'a, T>])
where
    T: graphql_parser::query::Text<'a>,
{
    definitions
        .iter()
        .filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Enum(enm)) => Some(enm.name.as_ref()),
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
                Some(object.name.as_ref())
            }
            _ => None,
        })
        .enumerate()
        .for_each(|(idx, object_name)| {
            schema
                .names
                .insert(object_name.into(), TypeId::r#object(idx as u32));
        });

    definitions
        .iter()
        .filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Interface(interface)) => {
                Some(interface.name.as_ref())
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
            Definition::TypeDefinition(TypeDefinition::Union(union)) => Some(union.name.as_ref()),
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
                Some(input.name.as_ref())
            }
            _ => None,
        })
        .enumerate()
        .for_each(|(idx, input_name)| {
            schema
                .names
                .insert(input_name.into(), TypeId::input(idx as u32));
        });
}

fn ingest_union<'doc, T>(schema: &mut Schema, union: &mut UnionType<'doc, T>)
where
    T: graphql_parser::query::Text<'doc>,
{
    let stored_union = super::StoredUnion {
        name: union.name.as_ref().to_string(),
        variants: union
            .types
            .iter()
            .map(|name| schema.find_type_id(name.as_ref()))
            .collect(),
    };

    schema.stored_unions.push(stored_union);
}

fn ingest_object<'a, T>(schema: &mut Schema, obj: &mut graphql_parser::schema::ObjectType<'a, T>)
where
    T: graphql_parser::query::Text<'a>,
{
    let object_id = schema
        .find_type_id(obj.name.as_ref())
        .as_object_id()
        .unwrap();
    let mut field_ids = Vec::with_capacity(obj.fields.len());

    for field in obj.fields.iter_mut() {
        let field = super::StoredField {
            name: field.name.as_ref().to_string(),
            r#type: resolve_field_type(schema, &field.field_type),
            parent: super::StoredFieldParent::Object(object_id),
            deprecation: find_deprecation(&field.directives),
        };

        field_ids.push(schema.push_field(field));
    }

    // Ingest the object itself
    let object = super::StoredObject {
        name: obj.name.as_ref().to_string(),
        fields: field_ids,
        implements_interfaces: obj
            .implements_interfaces
            .iter()
            .map(|iface_name| schema.find_interface(iface_name.as_ref()))
            .collect(),
    };

    schema.push_object(object);
}

fn ingest_scalar<'a, T>(schema: &mut Schema, scalar: &mut graphql_parser::schema::ScalarType<'a, T>)
where
    T: graphql_parser::query::Text<'a>,
{
    let name = scalar.name.as_ref().to_string();

    let scalar = super::StoredScalar {
        name: name.to_string(),
    };

    let scalar_id = schema.push_scalar(scalar);

    schema.names.insert(name, TypeId::Scalar(scalar_id));
}

fn ingest_enum<'a, T>(schema: &mut Schema, enm: &mut graphql_parser::schema::EnumType<'a, T>)
where
    T: graphql_parser::query::Text<'a>,
{
    let enm = super::StoredEnum {
        name: enm.name.as_ref().to_string(),
        variants: enm
            .values
            .iter_mut()
            .map(|value| value.name.as_ref().to_string())
            .collect(),
    };

    schema.push_enum(enm);
}

fn ingest_interface<'a, T>(
    schema: &mut Schema,
    interface: &mut graphql_parser::schema::InterfaceType<'a, T>,
) where
    T: graphql_parser::query::Text<'a>,
{
    let interface_id = schema
        .find_type_id(interface.name.as_ref())
        .as_interface_id()
        .unwrap();

    let mut field_ids = Vec::with_capacity(interface.fields.len());

    for field in interface.fields.iter_mut() {
        let field = super::StoredField {
            name: field.name.as_ref().to_string(),
            r#type: resolve_field_type(schema, &field.field_type),
            parent: super::StoredFieldParent::Interface(interface_id),
            deprecation: find_deprecation(&field.directives),
        };

        field_ids.push(schema.push_field(field));
    }

    let new_interface = super::StoredInterface {
        name: interface.name.as_ref().to_string(),
        fields: field_ids,
    };

    schema.push_interface(new_interface);
}

fn find_deprecation<'a, T>(directives: &[parser::Directive<'a, T>]) -> Option<Option<String>>
where
    T: graphql_parser::query::Text<'a>,
{
    directives
        .iter()
        .find(|directive| directive.name.as_ref() == "deprecated")
        .map(|directive| {
            directive
                .arguments
                .iter()
                .find(|(name, _)| name.as_ref() == "reason")
                .and_then(|(_, value)| match value {
                    graphql_parser::query::Value::String(s) => Some(s.clone()),
                    _ => None,
                })
        })
}

fn ingest_input<'a, T>(schema: &mut Schema, input: &mut parser::InputObjectType<'a, T>)
where
    T: graphql_parser::query::Text<'a>,
{
    let input = super::StoredInputType {
        name: input.name.as_ref().to_string(),
        fields: input
            .fields
            .iter_mut()
            .map(|val| {
                let field_type = super::resolve_field_type(schema, &val.value_type);
                (
                    val.name.as_ref().to_string(),
                    StoredInputFieldType {
                        qualifiers: field_type.qualifiers,
                        id: field_type.id,
                    },
                )
            })
            .collect(),
    };

    schema.stored_inputs.push(input);
}

fn objects_mut<'a, 'doc: 'a, T>(
    doc: &'a mut Document<'doc, T>,
) -> impl Iterator<Item = &'a mut parser::ObjectType<'doc, T>>
where
    T: graphql_parser::query::Text<'doc>,
{
    doc.definitions.iter_mut().filter_map(|def| match def {
        Definition::TypeDefinition(TypeDefinition::Object(obj)) => Some(obj),
        _ => None,
    })
}

fn interfaces_mut<'a, 'doc: 'a, T>(
    doc: &'a mut Document<'doc, T>,
) -> impl Iterator<Item = &'a mut parser::InterfaceType<'doc, T>>
where
    T: graphql_parser::query::Text<'doc>,
{
    doc.definitions.iter_mut().filter_map(|def| match def {
        Definition::TypeDefinition(TypeDefinition::Interface(interface)) => Some(interface),
        _ => None,
    })
}

fn unions_mut<'a, 'doc: 'a, T>(
    doc: &'a mut Document<'doc, T>,
) -> impl Iterator<Item = &'a mut parser::UnionType<'doc, T>>
where
    T: graphql_parser::query::Text<'doc>,
{
    doc.definitions.iter_mut().filter_map(|def| match def {
        Definition::TypeDefinition(TypeDefinition::Union(union)) => Some(union),
        _ => None,
    })
}

fn enums_mut<'a, 'doc: 'a, T>(
    doc: &'a mut Document<'doc, T>,
) -> impl Iterator<Item = &'a mut parser::EnumType<'doc, T>>
where
    T: graphql_parser::query::Text<'doc>,
{
    doc.definitions.iter_mut().filter_map(|def| match def {
        Definition::TypeDefinition(TypeDefinition::Enum(r#enum)) => Some(r#enum),
        _ => None,
    })
}

fn inputs_mut<'a, 'doc: 'a, T>(
    doc: &'a mut Document<'doc, T>,
) -> impl Iterator<Item = &'a mut parser::InputObjectType<'doc, T>>
where
    T: graphql_parser::query::Text<'doc>,
{
    doc.definitions.iter_mut().filter_map(|def| match def {
        Definition::TypeDefinition(TypeDefinition::InputObject(input)) => Some(input),
        _ => None,
    })
}
