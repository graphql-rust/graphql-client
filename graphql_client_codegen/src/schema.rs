mod graphql_parser_conversion;
mod json_conversion;

#[cfg(test)]
mod tests;

use crate::query::UsedTypes;
use crate::type_qualifiers::GraphqlTypeQualifier;
use std::collections::{HashMap, HashSet};

pub(crate) const DEFAULT_SCALARS: &[&str] = &["ID", "String", "Int", "Float", "Boolean"];

#[derive(Debug, PartialEq, Clone)]
struct StoredObjectField {
    name: String,
    object: ObjectId,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct StoredObject {
    pub(crate) name: String,
    pub(crate) fields: Vec<StoredFieldId>,
    pub(crate) implements_interfaces: Vec<InterfaceId>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct StoredField {
    pub(crate) name: String,
    pub(crate) r#type: StoredFieldType,
    pub(crate) parent: StoredFieldParent,
    /// `Some(None)` should be interpreted as "deprecated, without reason"
    pub(crate) deprecation: Option<Option<String>>,
}

impl StoredField {
    pub(crate) fn deprecation(&self) -> Option<Option<&str>> {
        self.deprecation.as_ref().map(|inner| inner.as_deref())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum StoredFieldParent {
    Object(ObjectId),
    Interface(InterfaceId),
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct ObjectId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct ObjectFieldId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct InterfaceId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct ScalarId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct UnionId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct EnumId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub(crate) struct InputId(u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct StoredFieldId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
struct InputFieldId(usize);

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StoredInterface {
    name: String,
    fields: Vec<StoredFieldId>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StoredFieldType {
    pub(crate) id: TypeId,
    /// An ordered list of qualifiers, from outer to inner.
    ///
    /// e.g. `[Int]!` would have `vec![List, Optional]`, but `[Int!]` would have `vec![Optional,
    /// List]`.
    pub(crate) qualifiers: Vec<GraphqlTypeQualifier>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StoredUnion {
    // TODO should this be a graphql_parser::query::Text type instead?
    pub(crate) name: String,
    pub(crate) variants: Vec<TypeId>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StoredScalar {
    // TODO should this be graphql_parser::query::Text?
    pub(crate) name: String,
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

impl TypeId {
    fn r#enum(id: usize) -> Self {
        TypeId::Enum(EnumId(id))
    }

    fn interface(id: usize) -> Self {
        TypeId::Interface(InterfaceId(id))
    }

    fn union(id: usize) -> Self {
        TypeId::Union(UnionId(id))
    }

    fn object(id: u32) -> Self {
        TypeId::Object(ObjectId(id))
    }

    fn input(id: u32) -> Self {
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

    pub(crate) fn as_input_id(&self) -> Option<InputId> {
        match self {
            TypeId::Input(id) => Some(*id),
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

    pub(crate) fn name<'a>(&self, schema: &'a Schema) -> &'a str {
        match self {
            TypeId::Object(obj) => schema.get_object(*obj).name.as_str(),
            TypeId::Scalar(s) => schema.get_scalar(*s).name.as_str(),
            TypeId::Interface(s) => schema.get_interface(*s).name.as_str(),
            TypeId::Union(s) => schema.get_union(*s).name.as_str(),
            TypeId::Enum(s) => schema.get_enum(*s).name.as_str(),
            TypeId::Input(s) => schema.get_input(*s).name.as_str(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StoredEnum {
    pub(crate) name: String,
    pub(crate) variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StoredInputFieldType {
    pub(crate) id: TypeId,
    pub(crate) qualifiers: Vec<GraphqlTypeQualifier>,
}

impl StoredInputFieldType {
    /// A type is indirected if it is a (flat or nested) list type, optional or not.
    ///
    /// We use this to determine whether a type needs to be boxed for recursion.
    pub(crate) fn is_indirected(&self) -> bool {
        self.qualifiers
            .iter()
            .any(|qualifier| qualifier == &GraphqlTypeQualifier::List)
    }

    pub(crate) fn is_optional(&self) -> bool {
        self.qualifiers
            .get(0)
            .map(|qualifier| !qualifier.is_required())
            .unwrap_or(true)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StoredInputType {
    pub(crate) name: String,
    pub(crate) fields: Vec<(String, StoredInputFieldType)>,
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

            self.names.insert((*scalar).to_owned(), TypeId::Scalar(id));
        }
    }

    fn push_object(&mut self, object: StoredObject) -> ObjectId {
        let id = ObjectId(self.stored_objects.len() as u32);
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

    pub(crate) fn query_type(&self) -> ObjectId {
        self.query_type
            .expect("Query operation type must be defined")
    }

    pub(crate) fn mutation_type(&self) -> Option<ObjectId> {
        self.mutation_type
    }

    pub(crate) fn subscription_type(&self) -> Option<ObjectId> {
        self.subscription_type
    }

    pub(crate) fn get_interface(&self, interface_id: InterfaceId) -> &StoredInterface {
        self.stored_interfaces.get(interface_id.0).unwrap()
    }

    pub(crate) fn get_input(&self, input_id: InputId) -> &StoredInputType {
        self.stored_inputs.get(input_id.0 as usize).unwrap()
    }

    pub(crate) fn get_object(&self, object_id: ObjectId) -> &StoredObject {
        self.stored_objects
            .get(object_id.0 as usize)
            .expect("Schema::get_object")
    }

    pub(crate) fn get_field(&self, field_id: StoredFieldId) -> &StoredField {
        self.stored_fields.get(field_id.0).unwrap()
    }

    pub(crate) fn get_enum(&self, enum_id: EnumId) -> &StoredEnum {
        self.stored_enums.get(enum_id.0).unwrap()
    }

    pub(crate) fn get_scalar(&self, scalar_id: ScalarId) -> &StoredScalar {
        self.stored_scalars.get(scalar_id.0).unwrap()
    }

    pub(crate) fn get_union(&self, union_id: UnionId) -> &StoredUnion {
        self.stored_unions
            .get(union_id.0)
            .expect("Schema::get_union")
    }

    fn find_interface(&self, interface_name: &str) -> InterfaceId {
        self.find_type_id(interface_name).as_interface_id().unwrap()
    }

    pub(crate) fn find_type(&self, type_name: &str) -> Option<TypeId> {
        self.names.get(type_name).copied()
    }

    pub(crate) fn objects(&self) -> impl Iterator<Item = (ObjectId, &StoredObject)> {
        self.stored_objects
            .iter()
            .enumerate()
            .map(|(idx, obj)| (ObjectId(idx as u32), obj))
    }

    pub(crate) fn inputs(&self) -> impl Iterator<Item = (InputId, &StoredInputType)> {
        self.stored_inputs
            .iter()
            .enumerate()
            .map(|(idx, obj)| (InputId(idx as u32), obj))
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

impl StoredInputType {
    pub(crate) fn used_input_ids_recursive(&self, used_types: &mut UsedTypes, schema: &Schema) {
        for type_id in self.fields.iter().map(|(_name, ty)| ty.id) {
            match type_id {
                TypeId::Input(input_id) => {
                    if used_types.types.contains(&type_id) {
                        continue;
                    } else {
                        used_types.types.insert(type_id);
                        let input = schema.get_input(input_id);
                        input.used_input_ids_recursive(used_types, schema);
                    }
                }
                TypeId::Enum(_) | TypeId::Scalar(_) => {
                    used_types.types.insert(type_id);
                }
                _ => (),
            }
        }
    }

    fn contains_type_without_indirection<'a>(
        &'a self,
        input_id: InputId,
        schema: &'a Schema,
        visited_types: &mut HashSet<&'a str>,
    ) -> bool {
        visited_types.insert(&self.name);
        // The input type is recursive if any of its members contains it, without indirection
        self.fields.iter().any(|(_name, field_type)| {
            // the field is indirected, so no boxing is needed
            if field_type.is_indirected() {
                return false;
            }

            let field_input_id = field_type.id.as_input_id();

            if let Some(field_input_id) = field_input_id {
                if field_input_id == input_id {
                    return true;
                }

                let input = schema.get_input(field_input_id);

                // no need to visit type twice (prevents infinite recursion)
                if visited_types.contains(&input.name.as_str()) {
                    return false;
                }

                // we check if the other input contains this one (without indirection)
                input.contains_type_without_indirection(input_id, schema, visited_types)
            } else {
                // the field is not referring to an input type
                false
            }
        })
    }
}

pub(crate) fn input_is_recursive_without_indirection(input_id: InputId, schema: &Schema) -> bool {
    let input = schema.get_input(input_id);
    let mut visited_types = HashSet::<&str>::new();
    input.contains_type_without_indirection(input_id, schema, &mut visited_types)
}
impl<'a, T> std::convert::From<graphql_parser::schema::Document<'a, T>> for Schema
where
    T: graphql_parser::query::Text<'a>,
{
    fn from(ast: graphql_parser::schema::Document<'a, T>) -> Schema {
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

pub(crate) fn resolve_field_type<'a, T>(
    schema: &Schema,
    inner: &graphql_parser::schema::Type<'a, T>,
) -> StoredFieldType
where
    T: graphql_parser::query::Text<'a>,
{
    use crate::type_qualifiers::graphql_parser_depth;
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
                    id: schema.find_type_id(name.as_ref()),
                    qualifiers,
                }
            }
        }
    }
}

pub(crate) trait ObjectLike {
    fn name(&self) -> &str;

    fn get_field_by_name<'a>(
        &'a self,
        name: &str,
        schema: &'a Schema,
    ) -> Option<(StoredFieldId, &'a StoredField)>;
}

impl ObjectLike for StoredObject {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_field_by_name<'a>(
        &'a self,
        name: &str,
        schema: &'a Schema,
    ) -> Option<(StoredFieldId, &'a StoredField)> {
        self.fields
            .iter()
            .map(|field_id| (*field_id, schema.get_field(*field_id)))
            .find(|(_, f)| f.name == name)
    }
}

impl ObjectLike for StoredInterface {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_field_by_name<'a>(
        &'a self,
        name: &str,
        schema: &'a Schema,
    ) -> Option<(StoredFieldId, &'a StoredField)> {
        self.fields
            .iter()
            .map(|field_id| (*field_id, schema.get_field(*field_id)))
            .find(|(_, field)| field.name == name)
    }
}
