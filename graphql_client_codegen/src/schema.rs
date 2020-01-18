mod graphql_parser_conversion;
mod json_conversion;

use crate::deprecation::DeprecationStatus;
use crate::enums::{EnumVariant, GqlEnum};
use crate::field_type::FieldType;
use crate::inputs::GqlInput;
use crate::interfaces::GqlInterface;
use crate::objects::{GqlObject, GqlObjectField};
use crate::scalars::Scalar;
use crate::unions::GqlUnion;
use failure::*;
use graphql_parser::{self, schema};
use std::collections::{BTreeMap, BTreeSet};

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
    fields: Vec<ObjectFieldId>,
    implements_interfaces: Vec<InterfaceId>,
}

impl StoredObject {}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ObjectId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
struct ObjectFieldId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
struct InterfaceFieldId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
struct InterfaceId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
struct ScalarId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
struct UnionId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
struct EnumId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
struct InputObjectId(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
struct InputFieldId(usize);

#[derive(Debug, Clone, PartialEq)]
struct StoredInterface {
    name: String,
    fields: Vec<InterfaceFieldId>,
    implemented_by: Vec<ObjectId>,
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
    qualifiers: Vec<crate::field_type::GraphqlTypeQualifier>,
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum TypeId {
    ObjectId(ObjectId),
    ScalarId(ScalarId),
    InterfaceId(InterfaceId),
    UnionId(UnionId),
    EnumId(EnumId),
}

#[derive(Debug, Clone, PartialEq)]
struct StoredEnum {
    name: String,
    variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct StoredInputFieldType {
    id: TypeId,
    qualifiers: Vec<crate::field_type::GraphqlTypeQualifier>,
}

impl StoredInputFieldType {
    /// A type is indirected if it is a (flat or nested) list type, optional or not.
    ///
    /// We use this to determine whether a type needs to be boxed for recursion.
    pub fn is_indirected(&self) -> bool {
        self.qualifiers
            .iter()
            .any(|qualifier| qualifier == &crate::Field_type::GraphqlTypeQualifier::List)
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
    InputObject(InputObjectId),
}

/// Intermediate representation for a parsed GraphQL schema used during code generation.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Schema {
    stored_objects: Vec<StoredObject>,
    stored_object_fields: Vec<StoredObjectField>,
    stored_interfaces: Vec<StoredInterface>,
    stored_interface_fields: Vec<StoredInterfaceField>,
    stored_unions: Vec<StoredUnion>,
    stored_scalars: Vec<StoredScalar>,
    stored_enums: Vec<StoredEnum>,
    stored_inputs: Vec<StoredInputType>,
    pub(crate) query_type: Option<String>,
    pub(crate) mutation_type: Option<String>,
    pub(crate) subscription_type: Option<String>,
}

impl Schema {
    fn default_scalars() -> Vec<StoredScalar> {
        let mut scalars = Vec::with_capacity(DEFAULT_SCALARS.len());

        for scalar in DEFAULT_SCALARS {
            scalars.push(StoredScalar {
                name: (*scalar).to_owned(),
            });
        }

        scalars
    }

    pub(crate) fn new() -> Schema {
        Schema {
            stored_objects: Vec::new(),
            stored_object_fields: Vec::new(),
            stored_interfaces: Vec::new(),
            stored_interface_fields: Vec::new(),
            stored_unions: Vec::new(),
            stored_scalars: Self::default_scalars(),
            stored_enums: Vec::new(),
            stored_inputs: Vec::new(),
            query_type: None,
            mutation_type: None,
            subscription_type: None,
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

    fn get_interface_by_name_mut(
        &mut self,
        interface_name: &str,
    ) -> Option<(InterfaceId, &mut StoredInterface)> {
        self.stored_interfaces
            .iter_mut()
            .enumerate()
            .find(|(idx, iface)| iface.name == interface_name)
            .map(|(idx, iface)| (InterfaceId(idx), iface))
    }

    fn push_object(&mut self, object: StoredObject) -> ObjectId {
        let id = ObjectId(self.stored_objects.len());
        self.stored_objects.push(object);

        id
    }

    fn push_object_field(&mut self, object_field: StoredObjectField) -> ObjectFieldId {
        let id = ObjectFieldId(self.stored_object_fields.len());

        self.stored_object_fields.push(object_field);

        id
    }

    fn push_interface(&mut self, interface: StoredInterface) -> InterfaceId {
        let id = InterfaceId(self.stored_interfaces.len());

        self.stored_interfaces.push(interface);

        id
    }

    fn push_interface_field(&mut self, interface_field: StoredInterfaceField) -> InterfaceFieldId {
        let id = InterfaceFieldId(self.stored_interface_fields.len());

        self.stored_interface_fields.push(interface_field);

        id
    }

    fn push_scalar(&mut self, scalar: StoredScalar) -> ScalarId {
        let id = ScalarId(self.stored_scalars.len());

        self.stored_scalars.push(scalar);

        id
    }

    pub(crate) fn get_input_type_by_name(&self, name: &str) -> Option<InputRef<'_>> {
        self.stored_inputs.iter().position(|input| input.name == name).map(InputObjectId).map(|idx| {
            InputRef {
                schema: self,
                input_id: idx
            }
        })
    }

    fn get_stored_input(&self, input_id: InputObjectId) -> &StoredInputType {
        self.stored_inputs.get(input_id.0).unwrap()
    }
}

struct InputFieldRef<'a> {
    schema: SchemaRef<'a>,
    input_field_id: Input
}

struct InputRef<'a> {
    schema: SchemaRef<'a>,
    input_id: InputObjectId,
}

impl<'a> InputRef<'a> {
    fn get(&self) -> &StoredInputType {
        self.schema.get_stored_input(self.input_id)
    }

    pub(crate) fn contains_type_without_indirection(
        &self,
        type_name: &str,
    ) -> bool {
        let input = self.get();


        // the input type is recursive if any of its members contains it, without indirection
        input.fields.iter().any(|(name, r#type)| {
            // the field is indirected, so no boxing is needed
            if r#type.is_indirected() {
                return false;
            }

            let field_type_name = field.type_.inner_name_str();
            let input = context.schema.inputs.get(field_type_name);

            if let Some(input) = input {
                // the input contains itself, not indirected
                if input.name == type_name {
                    return true;
                }

                // we check if the other input contains this one (without indirection)
                input.contains_type_without_indirection(context, type_name)
            } else {
                // the field is not referring to an input type
                false
            }
        })
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
