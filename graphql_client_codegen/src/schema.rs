mod graphql_parser_conversion;
mod json_conversion;
use crate::field_type::GraphqlTypeQualifier;
use std::collections::HashMap;

// use crate::deprecation::DeprecationStatus;
// use crate::enums::{EnumVariant, GqlEnum};
// use crate::field_type::FieldType;
// use crate::inputs::GqlInput;
// use crate::interfaces::GqlInterface;
// use crate::objects::{GqlObject, GqlObjectField};
// use crate::scalars::Scalar;
// use crate::unions::GqlUnion;
// use failure::*;

pub(crate) const DEFAULT_SCALARS: &[&str] = &["ID", "String", "Int", "Float", "Boolean"];

pub(crate) type SchemaRef<'a> = &'a Schema;

#[derive(Debug, PartialEq, Clone)]
struct StoredObjectField {
    name: String,
    object: ObjectId,
}

#[derive(Debug, PartialEq, Clone)]
struct StoredObject {
    name: String,
    fields: Vec<StoredFieldId>,
    implements_interfaces: Vec<InterfaceId>,
}

#[derive(Debug, PartialEq, Clone)]
struct StoredField {
    name: String,
    r#type: StoredFieldType,
    parent: StoredFieldParent,
}

#[derive(Debug, PartialEq, Clone)]
enum StoredFieldParent {
    Object(ObjectId),
    Interface(InterfaceId),
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct ObjectId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct ObjectFieldId(usize);

// #[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
// pub(crate) struct InterfaceFieldId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct InterfaceId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct ScalarId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct UnionId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct EnumId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct InputId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct StoredFieldId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
struct InputFieldId(usize);

#[derive(Debug, Clone, PartialEq)]
struct StoredInterface {
    name: String,
    fields: Vec<StoredFieldId>,
}

#[derive(Debug, Clone, PartialEq)]
struct StoredInterfaceField {
    name: String,
    interface: InterfaceId,
    r#type: StoredFieldType,
}

#[derive(Debug, Clone, PartialEq)]
struct StoredFieldType {
    id: TypeId,
    qualifiers: Vec<GraphqlTypeQualifier>,
}

#[derive(Debug, Clone, PartialEq)]
struct StoredUnion {
    name: String,
    variants: Vec<TypeId>,
}

#[derive(Debug, Clone, PartialEq)]
struct StoredScalar {
    name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) enum TypeId {
    Object(ObjectId),
    Scalar(ScalarId),
    Interface(InterfaceId),
    Union(UnionId),
    Enum(EnumId),
    Input(InputId),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum TypeRef<'a> {
    Object(ObjectRef<'a>),
    Scalar(ScalarRef<'a>),
    Interface(InterfaceRef<'a>),
    Union(UnionRef<'a>),
    Enum(EnumRef<'a>),
    Input(InputRef<'a>),
}

impl TypeRef<'_> {
    pub(crate) fn type_id(&self) -> TypeId {
        todo!("TypeRef::type_id")
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScalarRef<'a> {
    scalar_id: ScalarId,
    schema: &'a Schema,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct UnionRef<'a> {
    scalar_id: UnionId,
    schema: &'a Schema,
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct EnumRef<'a> {
    scalar_id: EnumId,
    schema: &'a Schema,
}

impl TypeId {
    fn scalar(id: usize) -> Self {
        TypeId::Scalar(ScalarId(id))
    }

    fn r#enum(id: usize) -> Self {
        TypeId::Enum(EnumId(id))
    }

    fn interface(id: usize) -> Self {
        TypeId::Interface(InterfaceId(id))
    }

    fn union(id: usize) -> Self {
        TypeId::Union(UnionId(id))
    }

    fn object(id: usize) -> Self {
        TypeId::Object(ObjectId(id))
    }

    fn input(id: usize) -> Self {
        TypeId::Input(InputId(id))
    }

    fn as_interface_id(&self) -> Option<InterfaceId> {
        match self {
            TypeId::Interface(id) => Some(*id),
            _ => None,
        }
    }

    fn as_object_id(&self) -> Option<ObjectId> {
        match self {
            TypeId::Object(id) => Some(*id),
            _ => None,
        }
    }

    pub(crate) fn upgrade(self, schema: &Schema) -> TypeRef<'_> {
        match self {
            TypeId::Enum(id) => TypeRef::Enum(EnumRef {
                scalar_id: id,
                schema,
            }),
            TypeId::Interface(id) => TypeRef::Interface(InterfaceRef {
                interface_id: id,
                schema,
            }),
            TypeId::Object(id) => TypeRef::Object(ObjectRef {
                object_id: id,
                schema,
            }),
            TypeId::Scalar(id) => TypeRef::Scalar(ScalarRef {
                scalar_id: id,
                schema,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct StoredEnum {
    name: String,
    variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StoredInputFieldType {
    id: TypeId,
    qualifiers: Vec<GraphqlTypeQualifier>,
}

impl StoredInputFieldType {
    /// A type is indirected if it is a (flat or nested) list type, optional or not.
    ///
    /// We use this to determine whether a type needs to be boxed for recursion.
    pub fn is_indirected(&self) -> bool {
        self.qualifiers
            .iter()
            .any(|qualifier| qualifier == &GraphqlTypeQualifier::List)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct StoredInputType {
    name: String,
    fields: Vec<(String, StoredInputFieldType)>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum InputFieldTypeId {
    Scalar(ScalarId),
    InputObject(InputId),
}

/// Intermediate representation for a parsed GraphQL schema used during code generation.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Schema {
    stored_objects: Vec<StoredObject>,
    stored_fields: Vec<StoredField>,
    stored_interfaces: Vec<StoredInterface>,
    stored_unions: Vec<StoredUnion>,
    stored_scalars: Vec<StoredScalar>,
    stored_enums: Vec<StoredEnum>,
    stored_inputs: Vec<StoredInputType>,
    names: HashMap<String, TypeId>,

    pub(crate) query_type: Option<ObjectId>,
    pub(crate) mutation_type: Option<ObjectId>,
    pub(crate) subscription_type: Option<ObjectId>,
}

impl Schema {
    pub(crate) fn new() -> Schema {
        let mut schema = Schema {
            stored_objects: Vec::new(),
            stored_interfaces: Vec::new(),
            stored_fields: Vec::new(),
            stored_unions: Vec::new(),
            stored_scalars: Vec::with_capacity(DEFAULT_SCALARS.len()),
            stored_enums: Vec::new(),
            stored_inputs: Vec::new(),
            names: HashMap::new(),
            query_type: None,
            mutation_type: None,
            subscription_type: None,
        };

        schema.push_default_scalars();

        schema
    }

    fn push_default_scalars(&mut self) {
        for scalar in DEFAULT_SCALARS {
            let id = self.push_scalar(StoredScalar {
                name: (*scalar).to_owned(),
            });

            self.names.insert(scalar.to_string(), TypeId::Scalar(id));
        }
    }

    // pub(crate) fn ingest_interface_implementations(
    //     &mut self,
    //     impls: BTreeMap<&'schema str, Vec<&'schema str>>,
    // ) -> Result<(), failure::Error> {
    //     impls
    //         .into_iter()
    //         .map(|(iface_name, implementors)| {
    //             let iface = self
    //                 .interfaces
    //                 .get_mut(&iface_name)
    //                 .ok_or_else(|| format_err!("interface not found: {}", iface_name))?;
    //             iface.implemented_by = implementors.iter().cloned().collect();
    //             Ok(())
    //         })
    //         .collect()
    // }

    // pub(crate) fn require(&self, typename_: &str) {
    //     DEFAULT_SCALARS
    //         .iter()
    //         .find(|&&s| s == typename_)
    //         .map(|_| ())
    //         .or_else(|| {
    //             self.enums
    //                 .get(typename_)
    //                 .map(|enm| enm.is_required.set(true))
    //         })
    //         .or_else(|| self.inputs.get(typename_).map(|input| input.require(self)))
    //         .or_else(|| {
    //             self.objects
    //                 .get(typename_)
    //                 .map(|object| object.require(self))
    //         })
    //         .or_else(|| {
    //             self.scalars
    //                 .get(typename_)
    //                 .map(|scalar| scalar.is_required.set(true))
    //         });
    // }

    // pub(crate) fn contains_scalar(&self, type_name: &str) -> bool {
    //     DEFAULT_SCALARS.iter().any(|s| s == &type_name) || self.scalars.contains_key(type_name)
    // }

    // pub(crate) fn fragment_target(
    //     &self,
    //     target_name: &str,
    // ) -> Option<crate::fragments::FragmentTarget<'_>> {
    //     self.objects
    //         .get(target_name)
    //         .map(crate::fragments::FragmentTarget::Object)
    //         .or_else(|| {
    //             self.interfaces
    //                 .get(target_name)
    //                 .map(crate::fragments::FragmentTarget::Interface)
    //         })
    //         .or_else(|| {
    //             self.unions
    //                 .get(target_name)
    //                 .map(crate::fragments::FragmentTarget::Union)
    //         })
    // }

    fn get_object_mut(&mut self, object_id: ObjectId) -> &mut StoredObject {
        self.stored_objects.get_mut(object_id.0).unwrap()
    }

    fn get_interface_mut(&mut self, id: InterfaceId) -> &mut StoredInterface {
        self.stored_interfaces.get_mut(id.0).unwrap()
    }

    // fn get_interface_by_name_mut(
    //     &mut self,
    //     interface_name: &str,
    // ) -> Option<(InterfaceId, &mut StoredInterface)> {
    //     self.stored_interfaces
    //         .iter_mut()
    //         .enumerate()
    //         .find(|(idx, iface)| iface.name == interface_name)
    //         .map(|(idx, iface)| (InterfaceId(idx), iface))
    // }

    fn push_object(&mut self, object: StoredObject) -> ObjectId {
        let id = ObjectId(self.stored_objects.len());
        self.stored_objects.push(object);

        id
    }

    // fn push_object_field(&mut self, object_field: StoredObjectField) -> ObjectFieldId {
    //     let id = ObjectFieldId(self.stored_object_fields.len());

    //     self.stored_object_fields.push(object_field);

    //     id
    // }

    fn push_interface(&mut self, interface: StoredInterface) -> InterfaceId {
        let id = InterfaceId(self.stored_interfaces.len());

        self.stored_interfaces.push(interface);

        id
    }

    // fn push_interface_field(&mut self, interface_field: StoredInterfaceField) -> InterfaceFieldId {
    //     let id = InterfaceFieldId(self.stored_interface_fields.len());

    //     self.stored_interface_fields.push(interface_field);

    //     id
    // }

    fn push_scalar(&mut self, scalar: StoredScalar) -> ScalarId {
        let id = ScalarId(self.stored_scalars.len());

        self.stored_scalars.push(scalar);

        id
    }

    fn push_enum(&mut self, enm: StoredEnum) -> EnumId {
        let id = EnumId(self.stored_enums.len());

        self.stored_enums.push(enm);

        id
    }

    fn push_field(&mut self, field: StoredField) -> StoredFieldId {
        let id = StoredFieldId(self.stored_fields.len());

        self.stored_fields.push(field);

        id
    }

    // pub(crate) fn get_input_type_by_name(&self, name: &str) -> Option<InputRef<'_>> {
    //     self.stored_inputs
    //         .iter()
    //         .position(|input| input.name == name)
    //         .map(InputId)
    //         .map(|idx| InputRef {
    //             schema: self,
    //             input_id: idx,
    //         })
    // }

    // pub(crate) fn get_object_by_name(&self, name: &str) -> Option<()> {
    //     Some(())
    // }

    // pub(crate) fn lookup_type(&self, name: &str) -> Option<TypeId> {
    //     todo!()
    // }

    pub(crate) fn query_type(&self) -> ObjectRef<'_> {
        ObjectRef {
            object_id: self
                .query_type
                .expect("Query operation type must be defined"),
            schema: self,
        }
    }

    pub(crate) fn mutation_type(&self) -> ObjectRef<'_> {
        ObjectRef {
            object_id: self
                .mutation_type
                .expect("Mutation operation type must be defined"),
            schema: self,
        }
    }

    pub(crate) fn subscription_type(&self) -> ObjectRef<'_> {
        ObjectRef {
            object_id: self
                .mutation_type
                // TODO: make this return an option
                .expect("Subscription operation type must be defined"),
            schema: self,
        }
    }

    fn get_interface(&self, interface_id: InterfaceId) -> &StoredInterface {
        self.stored_interfaces.get(interface_id.0).unwrap()
    }

    fn get_stored_input(&self, input_id: InputId) -> &StoredInputType {
        self.stored_inputs.get(input_id.0).unwrap()
    }

    fn get_object(&self, object_id: ObjectId) -> &StoredObject {
        self.stored_objects.get(object_id.0).unwrap()
    }

    fn get_field(&self, field_id: StoredFieldId) -> &StoredField {
        self.stored_fields.get(field_id.0).unwrap()
    }

    fn get_enum(&self, enum_id: EnumId) -> &StoredEnum {
        self.stored_enums.get(enum_id.0).unwrap()
    }

    pub(crate) fn object(&self, id: ObjectId) -> ObjectRef<'_> {
        ObjectRef {
            object_id: id,
            schema: self,
        }
    }

    pub(crate) fn interface(&self, interface_id: InterfaceId) -> InterfaceRef<'_> {
        InterfaceRef {
            interface_id,
            schema: self,
        }
    }

    pub(crate) fn field(&self, field_id: StoredFieldId) -> FieldRef<'_> {
        FieldRef {
            field_id,
            schema: self,
        }
    }

    fn find_interface(&self, interface_name: &str) -> InterfaceId {
        self.find_type_id(interface_name).as_interface_id().unwrap()
    }

    pub(crate) fn find_type(&self, type_name: &str) -> Option<TypeId> {
        self.names.get(type_name).map(|id| *id)
    }

    fn find_type_id(&self, type_name: &str) -> TypeId {
        match self.names.get(type_name) {
            Some(id) => *id,
            None => {
                panic!(
                    "graphql-client-codegen internal error: failed to resolve TypeId for `{}°.",
                    type_name
                );
            }
        }
    }
}

pub(crate) struct FieldsRef<'a> {
    parent_type: StoredFieldParent,
    schema: SchemaRef<'a>,
    fields: &'a [StoredFieldId],
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub(crate) struct InterfaceRef<'a> {
    schema: SchemaRef<'a>,
    interface_id: InterfaceId,
}

impl<'a> InterfaceRef<'a> {
    fn get(&self) -> &'a StoredInterface {
        self.schema.get_interface(self.interface_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ObjectRef<'a> {
    schema: SchemaRef<'a>,
    object_id: ObjectId,
}

impl<'a> ObjectRef<'a> {
    fn get(&self) -> &'a StoredObject {
        self.schema.get_object(self.object_id)
    }

    fn fields<'b>(&'b self) -> impl Iterator<Item = FieldRef<'a>> + 'b {
        self.get().fields.iter().map(move |field| FieldRef {
            schema: self.schema,
            field_id: *field,
        })
    }

    pub(crate) fn name(&self) -> &'a str {
        &self.get().name
    }

    pub(crate) fn get_field_by_name(&self, name: &str) -> Option<FieldRef<'a>> {
        self.fields().find(|field| field.name() == name)
    }

    pub(crate) fn schema(&self) -> SchemaRef<'a> {
        self.schema
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FieldRef<'a> {
    schema: SchemaRef<'a>,
    field_id: StoredFieldId,
}

impl<'a> FieldRef<'a> {
    fn get(&self) -> &'a StoredField {
        self.schema.get_field(self.field_id)
    }

    pub(crate) fn id(&self) -> StoredFieldId {
        self.field_id
    }

    pub(crate) fn name(&self) -> &str {
        &self.get().name
    }

    pub(crate) fn type_id(&self) -> TypeId {
        self.get().r#type.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct InputRef<'a> {
    schema: SchemaRef<'a>,
    input_id: InputId,
}

impl<'a> InputRef<'a> {
    fn get(&self) -> &StoredInputType {
        self.schema.get_stored_input(self.input_id)
    }

    pub(crate) fn contains_type_without_indirection(&self, type_name: &str) -> bool {
        todo!("contains type without indirection")
        // let input = self.get();

        // // the input type is recursive if any of its members contains it, without indirection
        // input.fields.iter().any(|(name, r#type)| {
        //     // the field is indirected, so no boxing is needed
        //     if r#type.is_indirected() {
        //         return false;
        //     }

        //     let field_type_name = field.type_.inner_name_str();
        //     let input = self.schema.inputs.get(field_type_name);

        //     if let Some(input) = input {
        //         // the input contains itself, not indirected
        //         if input.name == type_name {
        //             return true;
        //         }

        //         // we check if the other input contains this one (without indirection)
        //         input.contains_type_without_indirection(context, type_name)
        //     } else {
        //         // the field is not referring to an input type
        //         false
        //     }
        // })
    }
}

impl std::convert::From<graphql_parser::schema::Document> for Schema {
    fn from(ast: graphql_parser::schema::Document) -> Schema {
        graphql_parser_conversion::build_schema(ast)
    }
}

impl std::convert::From<graphql_introspection_query::introspection_response::IntrospectionResponse>
    for Schema
{
    fn from(
        src: graphql_introspection_query::introspection_response::IntrospectionResponse,
    ) -> Self {
        json_conversion::build_schema(src)
    }
}

pub(crate) enum ParsedSchema {
    GraphQLParser(graphql_parser::schema::Document),
    Json(graphql_introspection_query::introspection_response::IntrospectionResponse),
}

impl From<ParsedSchema> for Schema {
    fn from(parsed_schema: ParsedSchema) -> Schema {
        match parsed_schema {
            ParsedSchema::GraphQLParser(s) => s.into(),
            ParsedSchema::Json(s) => s.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn build_schema_works() {
        let gql_schema = include_str!("tests/star_wars_schema.graphql");
        let gql_schema = graphql_parser::parse_schema(gql_schema).unwrap();
        let built = Schema::from(&gql_schema);
        assert_eq!(
            built.objects.get("Droid"),
            Some(&GqlObject {
                description: None,
                name: "Droid",
                fields: vec![
                    GqlObjectField {
                        description: None,
                        name: TYPENAME_FIELD,
                        type_: FieldType::new(string_type()),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "id",
                        type_: FieldType::new("ID").nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "name",
                        type_: FieldType::new("String").nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "friends",
                        type_: FieldType::new("Character").list(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "friendsConnection",
                        type_: FieldType::new("FriendsConnection").nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "appearsIn",
                        type_: FieldType::new("Episode").list().nonnull(),
                        deprecation: DeprecationStatus::Current,
                    },
                    GqlObjectField {
                        description: None,
                        name: "primaryFunction",
                        type_: FieldType::new("String"),
                        deprecation: DeprecationStatus::Current,
                    },
                ],
                is_required: false.into(),
            })
        )
    }
}
