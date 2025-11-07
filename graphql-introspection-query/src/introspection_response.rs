#![allow(non_camel_case_types)]

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug)]
pub enum __DirectiveLocation {
    QUERY,
    MUTATION,
    SUBSCRIPTION,
    FIELD,
    FRAGMENT_DEFINITION,
    FRAGMENT_SPREAD,
    INLINE_FRAGMENT,
    SCHEMA,
    SCALAR,
    OBJECT,
    FIELD_DEFINITION,
    ARGUMENT_DEFINITION,
    INTERFACE,
    UNION,
    ENUM,
    ENUM_VALUE,
    INPUT_OBJECT,
    INPUT_FIELD_DEFINITION,
    Other(String),
}

impl Serialize for __DirectiveLocation {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            Self::QUERY => "QUERY",
            Self::MUTATION => "MUTATION",
            Self::SUBSCRIPTION => "SUBSCRIPTION",
            Self::FIELD => "FIELD",
            Self::FRAGMENT_DEFINITION => "FRAGMENT_DEFINITION",
            Self::FRAGMENT_SPREAD => "FRAGMENT_SPREAD",
            Self::INLINE_FRAGMENT => "INLINE_FRAGMENT",
            Self::SCHEMA => "SCHEMA",
            Self::SCALAR => "SCALAR",
            Self::OBJECT => "OBJECT",
            Self::FIELD_DEFINITION => "FIELD_DEFINITION",
            Self::ARGUMENT_DEFINITION => "ARGUMENT_DEFINITION",
            Self::INTERFACE => "INTERFACE",
            Self::UNION => "UNION",
            Self::ENUM => "ENUM",
            Self::ENUM_VALUE => "ENUM_VALUE",
            Self::INPUT_OBJECT => "INPUT_OBJECT",
            Self::INPUT_FIELD_DEFINITION => "INPUT_FIELD_DEFINITION",
            Self::Other(ref s) => s.as_str(),
        })
    }
}

impl<'de> Deserialize<'de> for __DirectiveLocation {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&'de str>::deserialize(deserializer)?;
        match s {
            "QUERY" => Ok(Self::QUERY),
            "MUTATION" => Ok(Self::MUTATION),
            "SUBSCRIPTION" => Ok(Self::SUBSCRIPTION),
            "FIELD" => Ok(Self::FIELD),
            "FRAGMENT_DEFINITION" => Ok(Self::FRAGMENT_DEFINITION),
            "FRAGMENT_SPREAD" => Ok(Self::FRAGMENT_SPREAD),
            "INLINE_FRAGMENT" => Ok(Self::INLINE_FRAGMENT),
            "SCHEMA" => Ok(Self::SCHEMA),
            "SCALAR" => Ok(Self::SCALAR),
            "OBJECT" => Ok(Self::OBJECT),
            "FIELD_DEFINITION" => Ok(Self::FIELD_DEFINITION),
            "ARGUMENT_DEFINITION" => Ok(Self::ARGUMENT_DEFINITION),
            "INTERFACE" => Ok(Self::INTERFACE),
            "UNION" => Ok(Self::UNION),
            "ENUM" => Ok(Self::ENUM),
            "ENUM_VALUE" => Ok(Self::ENUM_VALUE),
            "INPUT_OBJECT" => Ok(Self::INPUT_OBJECT),
            "INPUT_FIELD_DEFINITION" => Ok(Self::INPUT_FIELD_DEFINITION),
            _ => Ok(Self::Other(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum __TypeKind {
    SCALAR,
    OBJECT,
    INTERFACE,
    UNION,
    ENUM,
    INPUT_OBJECT,
    LIST,
    NON_NULL,
    Other(String),
}

impl Serialize for __TypeKind {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            Self::SCALAR => "SCALAR",
            Self::OBJECT => "OBJECT",
            Self::INTERFACE => "INTERFACE",
            Self::UNION => "UNION",
            Self::ENUM => "ENUM",
            Self::INPUT_OBJECT => "INPUT_OBJECT",
            Self::LIST => "LIST",
            Self::NON_NULL => "NON_NULL",
            Self::Other(ref s) => s.as_str(),
        })
    }
}

impl<'de> Deserialize<'de> for __TypeKind {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&'de str>::deserialize(deserializer)?;
        match s {
            "SCALAR" => Ok(Self::SCALAR),
            "OBJECT" => Ok(Self::OBJECT),
            "INTERFACE" => Ok(Self::INTERFACE),
            "UNION" => Ok(Self::UNION),
            "ENUM" => Ok(Self::ENUM),
            "INPUT_OBJECT" => Ok(Self::INPUT_OBJECT),
            "LIST" => Ok(Self::LIST),
            "NON_NULL" => Ok(Self::NON_NULL),
            _ => Ok(Self::Other(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullType {
    pub kind: Option<__TypeKind>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub fields: Option<Vec<FullTypeFields>>,
    pub input_fields: Option<Vec<FullTypeInputFields>>,
    pub interfaces: Option<Vec<FullTypeInterfaces>>,
    pub enum_values: Option<Vec<FullTypeEnumValues>>,
    pub possible_types: Option<Vec<FullTypePossibleTypes>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeFieldsArgs {
    #[serde(flatten)]
    #[allow(dead_code)]
    input_value: InputValue,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeFieldsType {
    #[serde(flatten)]
    pub type_ref: TypeRef,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeFields {
    pub name: Option<String>,
    pub description: Option<String>,
    pub args: Option<Vec<Option<FullTypeFieldsArgs>>>,
    #[serde(rename = "type")]
    pub type_: Option<FullTypeFieldsType>,
    pub is_deprecated: Option<bool>,
    pub deprecation_reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeInputFields {
    #[serde(flatten)]
    pub input_value: InputValue,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeInterfaces {
    #[serde(flatten)]
    pub type_ref: TypeRef,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypeEnumValues {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_deprecated: Option<bool>,
    pub deprecation_reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullTypePossibleTypes {
    #[serde(flatten)]
    pub type_ref: TypeRef,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputValue {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: InputValueType,
    pub default_value: Option<String>,
    #[serde(default)]
    pub is_deprecated: Option<bool>,
    #[serde(default)]
    pub deprecation_reason: Option<String>,
}

type InputValueType = TypeRef;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeRef {
    pub kind: Option<__TypeKind>,
    pub name: Option<String>,
    pub of_type: Option<Box<TypeRef>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaQueryType {
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaMutationType {
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaSubscriptionType {
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaTypes {
    #[serde(flatten)]
    pub full_type: FullType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaDirectivesArgs {
    #[serde(flatten)]
    #[allow(dead_code)]
    input_value: InputValue,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaDirectives {
    pub name: Option<String>,
    pub description: Option<String>,
    pub locations: Option<Vec<Option<__DirectiveLocation>>>,
    pub args: Option<Vec<Option<SchemaDirectivesArgs>>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub query_type: Option<SchemaQueryType>,
    pub mutation_type: Option<SchemaMutationType>,
    pub subscription_type: Option<SchemaSubscriptionType>,
    pub types: Option<Vec<Option<SchemaTypes>>>,
    #[allow(dead_code)]
    directives: Option<Vec<Option<SchemaDirectives>>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SchemaContainer {
    #[serde(rename = "__schema")]
    pub schema: Option<Schema>,
}

#[derive(Deserialize, Debug)]
pub struct FullResponse<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum IntrospectionResponse {
    FullResponse(FullResponse<SchemaContainer>),
    Schema(SchemaContainer),
}

impl IntrospectionResponse {
    pub fn as_schema(&self) -> &SchemaContainer {
        match self {
            Self::FullResponse(full_response) => &full_response.data,
            Self::Schema(schema) => schema,
        }
    }

    pub fn into_schema(self) -> SchemaContainer {
        match self {
            Self::FullResponse(full_response) => full_response.data,
            Self::Schema(schema) => schema,
        }
    }
}
