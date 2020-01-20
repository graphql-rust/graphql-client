use super::{Schema, TypeId};
use graphql_introspection_query::introspection_response::{
    FullType, IntrospectionResponse, Schema as JsonSchema, TypeRef, __TypeKind,
};
use std::collections::HashMap;

pub(super) fn build_schema(src: IntrospectionResponse) -> super::Schema {
    let converter = JsonSchemaConverter {
        src: src.into_schema().schema.unwrap(),
        schema: Schema::new(),
    };

    converter.convert()
}

struct JsonSchemaConverter {
    src: JsonSchema,
    schema: Schema,
}

impl JsonSchemaConverter {
    fn build_names_map(&mut self) {
        self.schema.names.reserve(types_mut(&mut self.src).count());
        let names = &mut self.schema.names;

        unions_mut(&mut self.src)
            .map(|u| u.name.as_ref().unwrap())
            .enumerate()
            .for_each(|(idx, name)| {
                names.insert(name.clone(), TypeId::union(idx));
            });

        interfaces_mut(&mut self.src)
            .map(|iface| iface.name.as_ref().unwrap())
            .enumerate()
            .for_each(|(idx, name)| {
                names.insert(name.clone(), TypeId::interface(idx));
            });

        objects_mut(&mut self.src)
            .map(|obj| obj.name.as_ref().unwrap())
            .enumerate()
            .for_each(|(idx, name)| {
                names.insert(name.clone(), TypeId::object(idx));
            });

        inputs_mut(&mut self.src)
            .map(|obj| obj.name.as_ref().unwrap())
            .enumerate()
            .for_each(|(idx, name)| {
                names.insert(name.clone(), TypeId::input(idx));
            });
    }

    fn convert(self) -> Schema {
        let JsonSchemaConverter {
            mut src,
            mut schema,
        } = self;

        schema.query_type = src.query_type.as_mut().and_then(|q| q.name.take()).take();
        schema.mutation_type = src
            .mutation_type
            .as_mut()
            .and_then(|m| m.name.take())
            .take();
        schema.subscription_type = src
            .subscription_type
            .as_mut()
            .and_then(|s| s.name.take())
            .take();

        for scalar in scalars_mut(&mut src) {
            ingest_scalar(&mut schema, scalar);
        }

        for enm in enums_mut(&mut src) {
            ingest_enum(&mut schema, enm)
        }

        for interface in interfaces_mut(&mut src) {
            ingest_interface(&mut schema, interface);
        }

        // for ty in src
        //     .types
        //     .as_ref()
        //     .expect("types in schema")
        //     .iter()
        //     .filter_map(|t| t.as_ref().map(|t| &t.full_type))
        // {
        //     let name: &str = ty
        //         .name
        //         .as_ref()
        //         .map(String::as_str)
        //         .expect("type definition name");

        //     match ty.kind {
        //         Some(__TypeKind::ENUM) => {
        //             // let variants: Vec<EnumVariant<'_>> = ty
        //             //     .enum_values
        //             //     .as_ref()
        //             //     .expect("enum variants")
        //             //     .iter()
        //             //     .map(|t| {
        //             //         t.as_ref().map(|t| EnumVariant {
        //             //             description: t.description.as_ref().map(String::as_str),
        //             //             name: t
        //             //                 .name
        //             //                 .as_ref()
        //             //                 .map(String::as_str)
        //             //                 .expect("enum variant name"),
        //             //         })
        //             //     })
        //             //     .filter_map(|t| t)
        //             //     .collect();
        //             // let enm = GqlEnum {
        //             //     name,
        //             //     description: ty.description.as_ref().map(String::as_str),
        //             //     variants,
        //             //     is_required: false.into(),
        //             // };
        //             // schema.enums.insert(name, enm);
        //         }
        //         Some(__TypeKind::SCALAR) => {
        //             // if DEFAULT_SCALARS.iter().find(|s| s == &&name).is_none() {
        //             //     schema.scalars.insert(
        //             //         name,
        //             //         Scalar {
        //             //             name,
        //             //             description: ty.description.as_ref().map(String::as_str),
        //             //             is_required: false.into(),
        //             //         },
        //             //     );
        //             // }
        //         }
        //         Some(__TypeKind::UNION) => {
        //             // let variants: BTreeSet<&str> = ty
        //             //     .possible_types
        //             //     .as_ref()
        //             //     .unwrap()
        //             //     .iter()
        //             //     .filter_map(|t| {
        //             //         t.as_ref()
        //             //             .and_then(|t| t.type_ref.name.as_ref().map(String::as_str))
        //             //     })
        //             //     .collect();
        //             // schema.unions.insert(
        //             //     name,
        //             //     GqlUnion {
        //             //         name: ty.name.as_ref().map(String::as_str).expect("unnamed union"),
        //             //         description: ty.description.as_ref().map(String::as_str),
        //             //         variants,
        //             //         is_required: false.into(),
        //             //     },
        //             // );
        //         }
        //         Some(__TypeKind::OBJECT) => {
        //             // for implementing in ty
        //             //     .interfaces
        //             //     .as_ref()
        //             //     .map(Vec::as_slice)
        //             //     .unwrap_or_else(|| &[])
        //             //     .iter()
        //             //     .filter_map(Option::as_ref)
        //             //     .map(|t| &t.type_ref.name)
        //             // {
        //             //     interface_implementations
        //             //         .entry(
        //             //             implementing
        //             //                 .as_ref()
        //             //                 .map(String::as_str)
        //             //                 .expect("interface name"),
        //             //         )
        //             //         .and_modify(|objects| objects.push(name))
        //             //         .or_insert_with(|| vec![name]);
        //             // }

        //             // schema
        //             //     .objects
        //             //     .insert(name, GqlObject::from_introspected_schema_json(ty));
        //         }
        //         Some(__TypeKind::INTERFACE) => {
        //             // let mut iface =
        //             //     GqlInterface::new(name, ty.description.as_ref().map(String::as_str));
        //             // iface.fields.extend(
        //             //     ty.fields
        //             //         .as_ref()
        //             //         .expect("interface fields")
        //             //         .iter()
        //             //         .filter_map(Option::as_ref)
        //             //         .map(|f| GqlObjectField {
        //             //             description: f.description.as_ref().map(String::as_str),
        //             //             name: f.name.as_ref().expect("field name").as_str(),
        //             //             type_: FieldType::from(f.type_.as_ref().expect("field type")),
        //             //             deprecation: DeprecationStatus::Current,
        //             //         }),
        //             // );
        //             // schema.interfaces.insert(name, iface);
        //         }
        //         Some(__TypeKind::INPUT_OBJECT) => {
        //             //     schema.inputs.insert(name, GqlInput::from(ty));
        //         }
        //         _ => unimplemented!("unimplemented definition"),
        //     }
        // }

        schema
    }
}

fn types_mut(schema: &mut JsonSchema) -> impl Iterator<Item = &mut FullType> {
    schema
        .types
        .as_mut()
        .unwrap()
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
            && !super::DEFAULT_SCALARS.contains(&t.name.as_ref().map(String::as_str).unwrap())
    })
}

fn ingest_scalar(schema: &mut Schema, scalar: &mut FullType) {
    let name: String = scalar.name.take().unwrap();
    let names_name = name.clone();

    let id = schema.push_scalar(super::StoredScalar { name });

    schema.names.insert(names_name, TypeId::Scalar(id));
}

fn ingest_enum(schema: &mut Schema, enm: &mut FullType) {
    let name = enm.name.take().unwrap();
    let names_name = name.clone();

    let variants = enm
        .enum_values
        .as_mut()
        .unwrap()
        .into_iter()
        .map(|v| std::mem::replace(v.name.as_mut().take().unwrap(), String::new()))
        .collect();

    let enm = super::StoredEnum { name, variants };

    let id = schema.push_enum(enm);

    schema.names.insert(names_name, TypeId::Enum(id));
}

fn ingest_interface(schema: &mut Schema, iface: &mut FullType) {
    let interface = super::StoredInterface {
        name: std::mem::replace(iface.name.as_mut().unwrap(), String::new()),
        fields: todo!(),
    };

    let interface_id = schema.push_interface(interface);

    // for field in iface.fields.as_mut().unwrap() {
    //     let field = field.as_mut().unwrap();
    //     let f = super::StoredInterfaceField {
    //         interface: interface_id,
    //         name: std::mem::replace(field.name.as_mut().unwrap(), String::new()),
    //         r#type: Self::resolve_field_type(&mut field.type_.as_mut().unwrap().type_ref),
    //     };

    //     let field_id = schema.push_interface_field(f);

    //     schema.get_interface_mut(interface_id).fields.push(field_id);
    // }
}

fn ingest_object(schema: &mut Schema, object: &mut FullType) {
    let object = super::StoredObject {
        name: object.name.take().unwrap(),
        implements_interfaces: Vec::new(),
        fields: object
            .fields
            .as_mut()
            .unwrap()
            .iter_mut()
            .map(|json_field| super::StoredField {
                name: json_field.name.take().unwrap(),
                r#type: resolve_field_type(
                    schema,
                    &mut json_field.type_.as_mut().unwrap().type_ref,
                ),
            })
            .collect(),
    };

    schema.push_object(object);
}

fn ingest_union(schema: &mut Schema, union: &mut FullType) {
    let variants = union
        .possible_types
        .as_ref()
        .unwrap()
        .iter()
        .map(|variant| schema.find_type_id(variant.type_ref.name.as_ref().unwrap()))
        .collect();
    let un = super::StoredUnion {
        name: union.name.take().unwrap(),
        variants,
    };

    schema.stored_unions.push(un);
}

fn resolve_field_type(schema: &mut Schema, typeref: &mut TypeRef) -> super::StoredFieldType {
    todo!()
}
