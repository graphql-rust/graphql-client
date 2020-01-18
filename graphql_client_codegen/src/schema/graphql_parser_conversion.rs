use super::Schema;
use graphql_parser::schema::{self as parser, Definition, TypeDefinition};

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
    fn objects_mut(&mut self) -> impl Iterator<Item = &mut parser::ObjectType> {
        self.src.definitions.iter_mut().filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Object(obj)) => Some(obj),
            _ => None,
        })
    }

    fn interfaces_mut(&mut self) -> impl Iterator<Item = &mut parser::InterfaceType> {
        self.src.definitions.iter_mut().filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Interface(interface)) => Some(interface),
            _ => None,
        })
    }

    fn unions_mut(&mut self) -> impl Iterator<Item = &mut parser::UnionType> {
        self.src.definitions.iter_mut().filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Union(union)) => Some(union),
            _ => None,
        })
    }

    fn enums_mut(&mut self) -> impl Iterator<Item = &mut parser::EnumType> {
        self.src.definitions.iter_mut().filter_map(|def| match def {
            Definition::TypeDefinition(TypeDefinition::Enum(r#enum)) => Some(r#enum),
            _ => None,
        })
    }

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

    fn resolve_field_type(
        &mut self,
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
                        id: self.find_type_id(name),
                        qualifiers,
                    }
                }
            }
        }
    }

    fn ingest_graphql_parser_scalar(&mut self, scalar: &mut graphql_parser::schema::ScalarType) {
        let scalar = super::StoredScalar {
            name: std::mem::replace(&mut scalar.name, String::new()),
        };

        self.schema.push_scalar(scalar);
    }

    fn ingest_graphql_parser_interface(
        &mut self,
        interface: &mut graphql_parser::schema::InterfaceType,
    ) {
        let new_interface = super::StoredInterface {
            name: std::mem::replace(&mut interface.name, String::new()),
            fields: Vec::new(),
            implemented_by: Vec::new(),
        };

        let interface_id = self.schema.push_interface(new_interface);

        for field in &mut interface.fields {
            let interface_field = super::StoredInterfaceField {
                name: std::mem::replace(&mut field.name, String::new()),
                interface: interface_id,
                r#type: self.resolve_field_type(&field.field_type),
            };
            let field_id = self.schema.push_interface_field(interface_field);

            self.schema
                .get_interface_mut(interface_id)
                .fields
                .push(field_id);
        }
    }

    fn ingest_graphql_parser_object(&mut self, obj: &mut graphql_parser::schema::ObjectType) {
        // Ingest the object itself
        let object = super::StoredObject {
            name: std::mem::replace(&mut obj.name, String::new()),
            fields: Vec::new(),
            implements_interfaces: obj
                .implements_interfaces
                .iter()
                .map(|iface_name| {
                    self.interfaces_mut()
                        .position(|iface| iface.name.as_str() == iface_name)
                        .unwrap()
                })
                .map(super::InterfaceId)
                .collect(),
        };

        let object_id = self.schema.push_object(object);

        // Ingest fields
        for graphql_field in &mut obj.fields {
            let field = super::StoredObjectField {
                name: std::mem::replace(&mut graphql_field.name, String::new()),
                object: object_id,
            };

            let field_id = self.schema.push_object_field(field);

            self.schema.get_object_mut(object_id).fields.push(field_id);
        }

        for id in self
            .schema
            .get_object_mut(object_id)
            .implements_interfaces
            .iter()
        {
            self.schema
                .get_interface_mut(*id)
                .implemented_by
                .push(object_id);
        }
    }

    fn ingest_graphql_parser_union(&mut self, union: &mut graphql_parser::schema::UnionType) {
        let stored_union = super::StoredUnion {
            name: std::mem::replace(&mut union.name, String::new()),
            variants: union
                .types
                .iter()
                .map(|name| self.find_type_id(name))
                .collect(),
        };

        self.schema.stored_unions.push(stored_union);
    }

    fn ingest_graphql_parser_input_object(
        &mut self,
        input: &mut graphql_parser::schema::InputObjectType,
    ) {
        unimplemented!()
    }

    fn convert(mut self) -> Schema {
        self.src
            .definitions
            .iter_mut()
            .filter_map(|def| match def {
                Definition::TypeDefinition(TypeDefinition::Scalar(scalar)) => Some(scalar),
                _ => None,
            })
            .for_each(|scalar| self.ingest_graphql_parser_scalar(scalar));

        self.src.enums_mut().for_each(|_| todo!());

        self.unions_mut()
            .for_each(|union| self.ingest_graphql_parser_union(union));

        self.interfaces_mut()
            .for_each(|iface| self.ingest_graphql_parser_interface(iface));

        self.objects_mut()
            .for_each(|object| self.ingest_graphql_parser_object(object));

        self.src
            .definitions
            .iter_mut()
            .filter_map(|def| match def {
                Definition::TypeDefinition(TypeDefinition::InputObject(input)) => Some(input),
                _ => None,
            })
            .for_each(|input_object| self.ingest_graphql_parser_input_object(input_object));

        let schema_definition = self.src.definitions.iter_mut().find_map(|def| match def {
            Definition::SchemaDefinition(definition) => Some(definition),
            _ => None,
        });

        if let Some(schema_definition) = schema_definition {
            self.schema.query_type = schema_definition.query;
            self.schema.mutation_type = schema_definition.mutation;
            self.schema.subscription_type = schema_definition.subscription;
        };

        self.schema
    }
}
