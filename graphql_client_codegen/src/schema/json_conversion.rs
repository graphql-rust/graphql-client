use super::{Schema, TypeId};
use graphql_introspection_query::introspection_response::{
    FullType, IntrospectionResponse, Schema as JsonSchema, TypeRef, __TypeKind,
};

pub(super) fn build_schema(src: IntrospectionResponse) -> Schema {
    let mut src = src.into_schema().schema.expect("could not find schema");
    let mut schema = Schema::new();
    build_names_map(&mut src, &mut schema);
    convert(&mut src, &mut schema);

    schema
}

fn build_names_map(src: &mut JsonSchema, schema: &mut Schema) {
    let mut names = &mut schema.names;
    names.reserve(types_mut(src).count());

    unions_mut(src)
        .map(|u| u.name.as_ref().expect("union name"))
        .enumerate()
        .for_each(|(idx, name)| {
            names.insert(name.clone(), TypeId::union(idx));
        });

    interfaces_mut(src)
        .map(|iface| iface.name.as_ref().expect("interface name"))
        .enumerate()
        .for_each(|(idx, name)| {
            names.insert(name.clone(), TypeId::interface(idx));
        });

    objects_mut(src)
        .map(|obj| obj.name.as_ref().expect("object name"))
        .enumerate()
        .for_each(|(idx, name)| {
            names.insert(name.clone(), TypeId::object(idx));
        });

    inputs_mut(src)
        .map(|obj| obj.name.as_ref().expect("input name"))
        .enumerate()
        .for_each(|(idx, name)| {
            names.insert(name.clone(), TypeId::input(idx));
        });
}

fn convert(src: &mut JsonSchema, schema: &mut Schema) {
    for scalar in scalars_mut(src) {
        ingest_scalar(schema, scalar);
    }

    for enm in enums_mut(src) {
        ingest_enum(schema, enm)
    }

    for interface in interfaces_mut(src) {
        ingest_interface(schema, interface);
    }

    for object in objects_mut(src) {
        ingest_object(schema, object);
    }

    for unn in unions_mut(src) {
        ingest_union(schema, unn)
    }

    // Define the root operations.
    {
        schema.query_type = src
            .query_type
            .as_mut()
            .and_then(|n| n.name.as_mut())
            .and_then(|n| schema.names.get(n))
            .and_then(|id| id.as_object_id());
        schema.mutation_type = src
            .mutation_type
            .as_mut()
            .and_then(|n| n.name.as_mut())
            .and_then(|n| schema.names.get(n))
            .and_then(|id| id.as_object_id());
        schema.subscription_type = src
            .mutation_type
            .as_mut()
            .and_then(|n| n.name.as_mut())
            .and_then(|n| schema.names.get(n))
            .and_then(|id| id.as_object_id());
    }
}

fn types_mut(schema: &mut JsonSchema) -> impl Iterator<Item = &mut FullType> {
    schema
        .types
        .as_mut()
        .expect("schema.types.as_mut()")
        .iter_mut()
        .filter_map(|t| -> Option<&mut FullType> { t.as_mut().map(|f| &mut f.full_type) })
}

fn objects_mut(schema: &mut JsonSchema) -> impl Iterator<Item = &mut FullType> {
    types_mut(schema).filter(|t| t.kind == Some(__TypeKind::OBJECT))
}

fn enums_mut(schema: &mut JsonSchema) -> impl Iterator<Item = &mut FullType> {
    types_mut(schema).filter(|t| t.kind == Some(__TypeKind::ENUM))
}

fn interfaces_mut(schema: &mut JsonSchema) -> impl Iterator<Item = &mut FullType> {
    types_mut(schema).filter(|t| t.kind == Some(__TypeKind::INTERFACE))
}

fn unions_mut(schema: &mut JsonSchema) -> impl Iterator<Item = &mut FullType> {
    types_mut(schema).filter(|t| t.kind == Some(__TypeKind::UNION))
}

fn inputs_mut(schema: &mut JsonSchema) -> impl Iterator<Item = &mut FullType> {
    types_mut(schema).filter(|t| t.kind == Some(__TypeKind::INPUT_OBJECT))
}

fn scalars_mut(schema: &mut JsonSchema) -> impl Iterator<Item = &mut FullType> {
    types_mut(schema).filter(|t| {
        t.kind == Some(__TypeKind::SCALAR)
            && !super::DEFAULT_SCALARS
                .contains(&t.name.as_ref().map(String::as_str).expect("FullType.name"))
    })
}

fn ingest_scalar(schema: &mut Schema, scalar: &mut FullType) {
    let name: String = scalar.name.take().expect("scalar.name");
    let names_name = name.clone();

    let id = schema.push_scalar(super::StoredScalar { name });

    schema.names.insert(names_name, TypeId::Scalar(id));
}

fn ingest_enum(schema: &mut Schema, enm: &mut FullType) {
    let name = enm.name.take().expect("enm.name");
    let names_name = name.clone();

    let variants = enm
        .enum_values
        .as_mut()
        .expect("enm.enum_values.as_mut()")
        .into_iter()
        .map(|v| {
            std::mem::replace(
                v.name
                    .as_mut()
                    .take()
                    .expect("variant.name.as_mut().take()"),
                String::new(),
            )
        })
        .collect();

    let enm = super::StoredEnum { name, variants };

    let id = schema.push_enum(enm);

    schema.names.insert(names_name, TypeId::Enum(id));
}

fn ingest_interface(schema: &mut Schema, iface: &mut FullType) {
    let interface_id = schema
        .find_type_id(iface.name.as_ref().expect("iface.name"))
        .as_interface_id()
        .expect("iface type id as interface id");
    let fields = iface.fields.as_mut().expect("interface.fields");
    let mut field_ids = Vec::with_capacity(fields.len());

    for field in fields.iter_mut() {
        let field = super::StoredField {
            parent: super::StoredFieldParent::Interface(interface_id),
            name: field.name.take().expect("take field name"),
            r#type: resolve_field_type(
                schema,
                &mut field.type_.as_mut().expect("take field type").type_ref,
            ),
            deprecation: Some(None)
                .filter(|_: &Option<()>| !field.is_deprecated.unwrap_or(false))
                .map(|_: Option<()>| field.deprecation_reason.clone()),
        };

        field_ids.push(schema.push_field(field));
    }

    let interface = super::StoredInterface {
        name: std::mem::replace(
            iface.name.as_mut().expect("iface.name.as_mut"),
            String::new(),
        ),
        fields: field_ids,
    };

    schema.push_interface(interface);
}

fn ingest_object(schema: &mut Schema, object: &mut FullType) {
    let object_id = schema
        .find_type_id(object.name.as_ref().expect("object.name"))
        .as_object_id()
        .expect("ingest_object > as_object_id");

    let fields = object.fields.as_mut().expect("object.fields.as_mut()");
    let mut field_ids = Vec::with_capacity(fields.len());

    for field in fields.iter_mut() {
        let field = super::StoredField {
            parent: super::StoredFieldParent::Object(object_id),
            name: field.name.take().expect("take field name"),
            r#type: resolve_field_type(
                schema,
                &mut field.type_.as_mut().expect("take field type").type_ref,
            ),
            deprecation: Some(None)
                .filter(|_: &Option<()>| !field.is_deprecated.unwrap_or(false))
                .map(|_: Option<()>| field.deprecation_reason.clone()),
        };

        field_ids.push(schema.push_field(field));
    }

    let object = super::StoredObject {
        name: object.name.take().expect("take object name"),
        implements_interfaces: Vec::new(),
        fields: field_ids,
    };

    schema.push_object(object);
}

fn ingest_union(schema: &mut Schema, union: &mut FullType) {
    let variants = union
        .possible_types
        .as_ref()
        .expect("union.possible_types")
        .iter()
        .map(|variant| {
            schema.find_type_id(
                variant
                    .type_ref
                    .name
                    .as_ref()
                    .expect("variant.type_ref.name"),
            )
        })
        .collect();
    let un = super::StoredUnion {
        name: union.name.take().expect("union.name.take"),
        variants,
    };

    schema.stored_unions.push(un);
}

fn resolve_field_type(schema: &mut Schema, typeref: &mut TypeRef) -> super::StoredFieldType {
    from_json_type_inner(schema, typeref)
}

fn json_type_qualifiers_depth(typeref: &mut TypeRef) -> usize {
    use graphql_introspection_query::introspection_response::*;

    match (typeref.kind.as_mut(), typeref.of_type.as_mut()) {
        (Some(__TypeKind::NON_NULL), Some(inner)) => 1 + json_type_qualifiers_depth(inner),
        (Some(__TypeKind::LIST), Some(inner)) => 1 + json_type_qualifiers_depth(inner),
        (Some(_), None) => 0,
        _ => panic!("Non-convertible type in JSON schema: {:?}", typeref),
    }
}

fn from_json_type_inner(schema: &mut Schema, inner: &mut TypeRef) -> super::StoredFieldType {
    use crate::field_type::GraphqlTypeQualifier;
    use graphql_introspection_query::introspection_response::*;

    let qualifiers_depth = json_type_qualifiers_depth(inner);
    let mut qualifiers = Vec::with_capacity(qualifiers_depth);

    let mut inner = inner;

    loop {
        match (
            inner.kind.as_mut(),
            inner.of_type.as_mut(),
            inner.name.as_mut(),
        ) {
            (Some(__TypeKind::NON_NULL), Some(new_inner), _) => {
                qualifiers.push(GraphqlTypeQualifier::Required);
                inner = new_inner.as_mut();
            }
            (Some(__TypeKind::LIST), Some(new_inner), _) => {
                qualifiers.push(GraphqlTypeQualifier::List);
                inner = new_inner.as_mut();
            }
            (Some(_), None, Some(name)) => {
                return super::StoredFieldType {
                    id: *schema.names.get(name).expect("schema.names.get(name)"),
                    qualifiers,
                }
            }
            _ => panic!("Non-convertible type in JSON schema"),
        }
    }
}
