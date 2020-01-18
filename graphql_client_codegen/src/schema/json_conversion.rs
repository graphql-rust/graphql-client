use super::Schema;
use graphql_introspection_query::introspection_response::{
    FullType, IntrospectionResponse, Schema as JsonSchema, __TypeKind,
};

pub(super) fn build_schema(src: IntrospectionResponse) -> super::Schema {
    let converter = JsonSchemaConverter {
        src: src.into_schema(),
        schema: Schema::new(),
    };

    converter.convert()
}

struct JsonSchemaConverter {
    src: JsonSchema,
    schema: Schema,
}

impl JsonSchemaConverter {
    fn find_type_id(&self, type_name: &str) -> super::TypeId {
        if let Some(id) = self
            .schema
            .stored_scalars
            .iter()
            .position(|scalar| scalar.name == type_name)
        {
            return super::TypeId::ScalarId(super::ScalarId(id));
        }

        if let Some(id) = self.objects_mut().position(|obj| obj.name == type_name) {
            return super::TypeId::ObjectId(super::ObjectId(id));
        }

        if let Some(id) = self.interfaces_mut().position(|obj| obj.name == type_name) {
            return super::TypeId::InterfaceId(super::InterfaceId(id));
        }

        if let Some(id) = self.unions_mut().position(|union| union.name == type_name) {
            return super::TypeId::UnionId(super::UnionId(id));
        }

        panic!(
            "graphql-client-codegen internal error: failed to resolve TypeId for `{}°.",
            type_name
        );
    }

    fn types_mut(&mut self) -> impl Iterator<Item = &mut FullType> {
        self.src.types.iter_mut()
    }

    fn objects_mut(&mut self) -> impl Iterator<Item = &mut FullType> {
        self.types().filter(|t| t.kind == __TypeKind::OBJECT)
    }

    fn enums_mut(&mut self) -> impl Iterator<Item = &mut FullType> {
        self.types().filter(|t| t.kind == __TypeKind::ENUM)
    }

    fn interfaces_mut(&mut self) -> impl Iterator<Item = &mut FullType> {
        self.types().filter(|t| t.kind == __TypeKind::INTERFACE)
    }

    fn unions_mut(&mut self) -> impl Iterator<Item = &mut FullType> {
        self.types().filter(|t| t.kind == __TypeKind::UNION)
    }

    fn input_objects_mut(&mut self) -> impl Iterator<Item = &mut FullType> {
        self.types().filter(|t| t.kind == __TypeKind::INPUT_OBJECT)
    }

    fn scalars_mut(&mut self) -> impl Iterator<Item = &mut FullType> {
        self.types()
            .filter(|t| t.kind == __TypeKind::SCALAR && !super::DEFAULT_SCALARS.contains(&t.name))
    }

    fn ingest_union(&mut self, union: &mut FullType) {
        let un = super::StoredUnion {
            name: std::mem::replace(&mut union.name, String::new()),
            variants: union
                .possible_types
                .unwrap()
                .into_iter()
                .map(|ft| self.find_type_id(&ft.name))
                .collect(),
        };

        self.schema.stored_unions.push(un);
    }

    fn resolve_field_type(&mut self, ref: &mut TypeRef) -> super::StoredFieldType {
        todo!()
    }

    fn ingest_interface(&mut self, iface: &mut FullType) {
        let interface = super::StoredInterface {
            name: std::mem::replace(iface.name.unwrap(), String::new()),
            implemented_by: Vec::new(),
            fields: Vec::new(),
        };

        let interface_id = self.schema.push_interface(interface);

        for field in iface.fields.as_mut().unwrap() {
            let field = field.unwrap();
            let f = super::StoredInterfaceField {
                interface: interface_id,
                name: std::mem::replace(&mut field.name, String::new()),
                r#type: self.resolve_field_type(field.type_.unwrap().type_ref),
            };

            let field_id = self.schema.push_interface_field(f);

            self.schema.get_interface_mut(interface_id).fields.push(field_id);
        }
    }

    fn ingest_object(&mut self, union: &mut FullType) -> {
        let object = super::StoredObject {
            name: std::mem::replace(iface.name.unwrap(), String::new()),
            implements_interfaces: Vec::new(),
            fields: Vec::new(),
        };

        for object in object.fields.as_mut().unwrap() {
            let field = field.unwrap();

            let f = super::StoredObjectField {
                object: object_id,
                name: std::mem::replace(&mut field.name, String::new()),
                r#type: self.resolve_field_type(fiedl.type_.unwrap().type_ref),
            };

            let field_id = self.schema.push_object_field(f);

            self.schema.get_object_mut(object_id).fields.push(field_id);
        }
    }

    fn convert(mut self) -> Schema {
        let root = self.src;

        self.schema.query_type = root.query_type;
        self.schema.mutation_type = root.mutation_type;
        self.schema.subscription_type = root.subscription_type;

        for scalar in self.scalars_mut() {
            self.schema.stored_scalars.push(super::StoredScalar {
                name: std::mem::replace(&mut scalar.name, String::new()),
            });
        }

        for r#enum in self.enums_mut() {
            let variants = r#enum.enum_values.unwrap();

            let enm = super::StoredEnum {
                name: std::mem::replace(&mut r#enum.name, String::new()),
                variants,
            };

            self.schema.stored_enums.push(enm);
        }

        self.interfaces_mut()
            .for_each(|iface| self.ingest_interface(iface));

        for ty in root
            .types
            .as_ref()
            .expect("types in schema")
            .iter()
            .filter_map(|t| t.as_ref().map(|t| &t.full_type))
        {
            let name: &str = ty
                .name
                .as_ref()
                .map(String::as_str)
                .expect("type definition name");

            match ty.kind {
                Some(__TypeKind::ENUM) => {
                    // let variants: Vec<EnumVariant<'_>> = ty
                    //     .enum_values
                    //     .as_ref()
                    //     .expect("enum variants")
                    //     .iter()
                    //     .map(|t| {
                    //         t.as_ref().map(|t| EnumVariant {
                    //             description: t.description.as_ref().map(String::as_str),
                    //             name: t
                    //                 .name
                    //                 .as_ref()
                    //                 .map(String::as_str)
                    //                 .expect("enum variant name"),
                    //         })
                    //     })
                    //     .filter_map(|t| t)
                    //     .collect();
                    // let enm = GqlEnum {
                    //     name,
                    //     description: ty.description.as_ref().map(String::as_str),
                    //     variants,
                    //     is_required: false.into(),
                    // };
                    // schema.enums.insert(name, enm);
                }
                Some(__TypeKind::SCALAR) => {
                    // if DEFAULT_SCALARS.iter().find(|s| s == &&name).is_none() {
                    //     schema.scalars.insert(
                    //         name,
                    //         Scalar {
                    //             name,
                    //             description: ty.description.as_ref().map(String::as_str),
                    //             is_required: false.into(),
                    //         },
                    //     );
                    // }
                }
                Some(__TypeKind::UNION) => {
                    // let variants: BTreeSet<&str> = ty
                    //     .possible_types
                    //     .as_ref()
                    //     .unwrap()
                    //     .iter()
                    //     .filter_map(|t| {
                    //         t.as_ref()
                    //             .and_then(|t| t.type_ref.name.as_ref().map(String::as_str))
                    //     })
                    //     .collect();
                    // schema.unions.insert(
                    //     name,
                    //     GqlUnion {
                    //         name: ty.name.as_ref().map(String::as_str).expect("unnamed union"),
                    //         description: ty.description.as_ref().map(String::as_str),
                    //         variants,
                    //         is_required: false.into(),
                    //     },
                    // );
                }
                Some(__TypeKind::OBJECT) => {
                    for implementing in ty
                        .interfaces
                        .as_ref()
                        .map(Vec::as_slice)
                        .unwrap_or_else(|| &[])
                        .iter()
                        .filter_map(Option::as_ref)
                        .map(|t| &t.type_ref.name)
                    {
                        interface_implementations
                            .entry(
                                implementing
                                    .as_ref()
                                    .map(String::as_str)
                                    .expect("interface name"),
                            )
                            .and_modify(|objects| objects.push(name))
                            .or_insert_with(|| vec![name]);
                    }

                    schema
                        .objects
                        .insert(name, GqlObject::from_introspected_schema_json(ty));
                }
                Some(__TypeKind::INTERFACE) => {
                    let mut iface =
                        GqlInterface::new(name, ty.description.as_ref().map(String::as_str));
                    iface.fields.extend(
                        ty.fields
                            .as_ref()
                            .expect("interface fields")
                            .iter()
                            .filter_map(Option::as_ref)
                            .map(|f| GqlObjectField {
                                description: f.description.as_ref().map(String::as_str),
                                name: f.name.as_ref().expect("field name").as_str(),
                                type_: FieldType::from(f.type_.as_ref().expect("field type")),
                                deprecation: DeprecationStatus::Current,
                            }),
                    );
                    schema.interfaces.insert(name, iface);
                }
                Some(__TypeKind::INPUT_OBJECT) => {
                    schema.inputs.insert(name, GqlInput::from(ty));
                }
                _ => unimplemented!("unimplemented definition"),
            }
        }

        schema
    }
}
