mod graphql_parser_conversion;
mod json_conversion;

use crate::field_type::GraphqlTypeQualifier;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub(crate) struct WithSchema<'a, T> {
    schema: &'a Schema,
    pub(crate) item: T,
}

impl<'a, T> WithSchema<'a, T> {
    fn refocus<U>(&self, item: U) -> WithSchema<'a, U> {
        WithSchema {
            schema: self.schema,
            item,
        }
    }

    pub(crate) fn new(schema: &'a Schema, item: T) -> WithSchema<'a, T> {
        WithSchema { schema, item }
    }
}

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
    /// `Some(None)` should be interpreted as "deprecated, without reason"
    deprecation: Option<Option<String>>,
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

impl InputId {
    fn new(idx: usize) -> Self {
        InputId(idx)
    }
}

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
pub(crate) struct StoredFieldType {
    pub(crate) id: TypeId,
    pub(crate) qualifiers: Vec<GraphqlTypeQualifier>,
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

impl<'a> WithSchema<'a, TypeId> {
    pub(crate) fn name(&self) -> &'a str {
        match self.item {
            TypeId::Object(obj) => self.refocus(obj).name(),
            TypeId::Scalar(s) => self.refocus(s).name(),
            TypeId::Interface(s) => self.refocus(s).name(),
            TypeId::Union(s) => self.refocus(s).name(),
            TypeId::Enum(s) => self.refocus(s).name(),
            TypeId::Input(s) => self.refocus(s).name(),
        }
    }
}

pub(crate) type ScalarRef<'a> = WithSchema<'a, ScalarId>;

impl<'a> ScalarRef<'a> {
    fn get(&self) -> &'a StoredScalar {
        self.schema.get_scalar(self.item)
    }

    pub(crate) fn name(&self) -> &'a str {
        &self.get().name
    }
}

pub(crate) type UnionRef<'a> = WithSchema<'a, UnionId>;

impl<'a> UnionRef<'a> {
    fn get(&self) -> &'a StoredUnion {
        self.schema.stored_unions.get(self.item.0).unwrap()
    }

    pub(crate) fn name(&self) -> &'a str {
        &self.get().name
    }
}

pub(crate) type EnumRef<'a> = WithSchema<'a, EnumId>;

impl<'a> EnumRef<'a> {
    fn get(&self) -> &'a StoredEnum {
        self.schema.get_enum(self.item)
    }

    pub(crate) fn name(&self) -> &'a str {
        &self.get().name
    }

    pub(crate) fn variants(&self) -> &'a [String] {
        &self.get().variants
    }
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

    pub(crate) fn as_scalar_id(&self) -> Option<ScalarId> {
        match self {
            TypeId::Scalar(id) => Some(*id),
            _ => None,
        }
    }

    pub(crate) fn as_enum_id(&self) -> Option<EnumId> {
        match self {
            TypeId::Enum(id) => Some(*id),
            _ => None,
        }
    }

    // pub(crate) fn upgrade(self, schema: &Schema) -> TypeRef<'_> {
    //     match self {
    //         TypeId::Enum(id) => TypeRef::Enum(EnumRef {
    //             enum_id: id,
    //             schema,
    //         }),
    //         TypeId::Interface(id) => TypeRef::Interface(InterfaceRef {
    //             interface_id: id,
    //             schema,
    //         }),
    //         TypeId::Object(id) => TypeRef::Object(ObjectRef {
    //             object_id: id,
    //             schema,
    //         }),
    //         TypeId::Scalar(id) => TypeRef::Scalar(ScalarRef {
    //             scalar_id: id,
    //             schema,
    //         }),
    //         TypeId::Union(id) => TypeRef::Union(UnionRef {
    //             union_id: id,
    //             schema,
    //         }),
    //     }
    // }
}

#[derive(Debug, Clone, PartialEq)]
struct StoredEnum {
    name: String,
    variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StoredInputFieldType {
    id: TypeId,
    pub(crate) qualifiers: Vec<GraphqlTypeQualifier>,
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

    fn get_object_mut(&mut self, object_id: ObjectId) -> &mut StoredObject {
        self.stored_objects.get_mut(object_id.0).unwrap()
    }

    fn get_interface_mut(&mut self, id: InterfaceId) -> &mut StoredInterface {
        self.stored_interfaces.get_mut(id.0).unwrap()
    }

    fn push_object(&mut self, object: StoredObject) -> ObjectId {
        let id = ObjectId(self.stored_objects.len());
        self.stored_objects.push(object);

        id
    }

    fn push_interface(&mut self, interface: StoredInterface) -> InterfaceId {
        let id = InterfaceId(self.stored_interfaces.len());

        self.stored_interfaces.push(interface);

        id
    }

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

    pub(crate) fn query_type(&self) -> ObjectRef<'_> {
        WithSchema::new(
            self,
            self.query_type
                .expect("Query operation type must be defined"),
        )
    }

    pub(crate) fn mutation_type(&self) -> ObjectRef<'_> {
        WithSchema::new(
            self,
            self.mutation_type
                .expect("Mutation operation type must be defined"),
        )
    }

    pub(crate) fn subscription_type(&self) -> ObjectRef<'_> {
        WithSchema::new(
            self,
            self.subscription_type
                // TODO: make this return an option
                .expect("Subscription operation type must be defined"),
        )
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

    fn get_scalar(&self, scalar_id: ScalarId) -> &StoredScalar {
        self.stored_scalars.get(scalar_id.0).unwrap()
    }

    pub(crate) fn object(&self, id: ObjectId) -> WithSchema<'_, ObjectId> {
        WithSchema::new(self, id)
    }

    pub(crate) fn interface(&self, interface_id: InterfaceId) -> InterfaceRef<'_> {
        WithSchema::new(self, interface_id)
    }

    pub(crate) fn field(&self, field_id: StoredFieldId) -> FieldRef<'_> {
        WithSchema::new(self, field_id)
    }

    pub(crate) fn scalar(&self, scalar_id: ScalarId) -> WithSchema<'_, ScalarId> {
        WithSchema::new(self, scalar_id)
    }

    pub(crate) fn r#enum(&self, enum_id: EnumId) -> EnumRef<'_> {
        WithSchema::new(self, enum_id)
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

    pub(crate) fn inputs<'a>(&'a self) -> impl Iterator<Item = WithSchema<'a, InputId>> + 'a {
        (0..self.stored_inputs.len()).map(move |id| WithSchema {
            schema: self,
            item: InputId(id),
        })
    }
}

type InterfaceRef<'a> = WithSchema<'a, InterfaceId>;

impl<'a> WithSchema<'a, InterfaceId> {
    fn get(&self) -> &'a StoredInterface {
        self.schema.get_interface(self.item)
    }

    pub(crate) fn name(&self) -> &'a str {
        &self.get().name
    }
}

type ObjectRef<'a> = WithSchema<'a, ObjectId>;

impl<'a> ObjectRef<'a> {
    fn get(&self) -> &'a StoredObject {
        self.schema.get_object(self.item)
    }

    fn fields<'b>(&'b self) -> impl Iterator<Item = WithSchema<'a, StoredFieldId>> + 'b {
        self.get()
            .fields
            .iter()
            .map(move |field| self.refocus(*field))
    }

    pub(crate) fn name(&self) -> &'a str {
        &self.get().name
    }

    pub(crate) fn get_field_by_name(&self, name: &str) -> Option<WithSchema<'a, StoredFieldId>> {
        self.fields().find(|field| field.name() == name)
    }

    pub(crate) fn schema(&self) -> SchemaRef<'a> {
        self.schema
    }

    pub(crate) fn id(&self) -> ObjectId {
        self.item
    }
}

pub(crate) type FieldRef<'a> = WithSchema<'a, StoredFieldId>;

impl<'a> FieldRef<'a> {
    fn get(&self) -> &'a StoredField {
        self.schema.get_field(self.item)
    }

    pub(crate) fn name(&self) -> &'a str {
        &self.get().name
    }

    pub(crate) fn field_type(&self) -> WithSchema<'a, TypeId> {
        self.refocus(self.get().r#type.id)
    }

    pub(crate) fn type_qualifiers(&self) -> &[GraphqlTypeQualifier] {
        &self.get().r#type.qualifiers
    }

    pub(crate) fn type_id(&self) -> TypeId {
        self.get().r#type.id
    }

    pub(crate) fn is_deprecated(&self) -> bool {
        self.get().deprecation.is_some()
    }

    pub(crate) fn deprecation_message(&self) -> Option<&'a str> {
        self.get()
            .deprecation
            .as_ref()
            .and_then(|item| item.as_ref().map(String::as_str))
    }
}

impl<'a> WithSchema<'a, InputId> {
    fn get(&self) -> &'a StoredInputType {
        self.schema.get_stored_input(self.item)
    }

    pub(crate) fn type_id(&self) -> TypeId {
        TypeId::Input(self.item)
    }

    pub(crate) fn name(&self) -> &'a str {
        &self.get().name
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

pub(crate) fn resolve_field_type(
    schema: &Schema,
    inner: &graphql_parser::schema::Type,
) -> StoredFieldType {
    use crate::field_type::graphql_parser_depth;

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
                return StoredFieldType {
                    id: schema.find_type_id(name),
                    qualifiers,
                }
            }
        }
    }
}

pub(crate) trait ObjectRefLike<'a> {
    fn name(&self) -> &'a str;

    fn get_field_by_name(&self, name: &str) -> Option<FieldRef<'a>>;

    fn schema(&self) -> SchemaRef<'a>;
}

impl<'a> ObjectRefLike<'a> for ObjectRef<'a> {
    fn name(&self) -> &'a str {
        self.name()
    }

    fn get_field_by_name(&self, name: &str) -> Option<FieldRef<'a>> {
        self.get_field_by_name(name)
    }

    fn schema(&self) -> SchemaRef<'a> {
        self.schema()
    }
}

impl<'a> ObjectRefLike<'a> for InterfaceRef<'a> {
    fn name(&self) -> &'a str {
        self.name()
    }

    fn get_field_by_name(&self, name: &str) -> Option<FieldRef<'a>> {
        self.get_field_by_name(name)
    }

    fn schema(&self) -> SchemaRef<'a> {
        InterfaceRef::schema(self)
    }
}
