use serde;

struct IntrospectionQuery;

type Boolean = bool;

type Float = f64;

type Int = i64;

#[derive(Debug)]
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

impl ::serde::Serialize for __DirectiveLocation {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            __DirectiveLocation::QUERY => "QUERY",
            __DirectiveLocation::MUTATION => "MUTATION",
            __DirectiveLocation::SUBSCRIPTION => "SUBSCRIPTION",
            __DirectiveLocation::FIELD => "FIELD",
            __DirectiveLocation::FRAGMENT_DEFINITION => "FRAGMENT_DEFINITION",
            __DirectiveLocation::FRAGMENT_SPREAD => "FRAGMENT_SPREAD",
            __DirectiveLocation::INLINE_FRAGMENT => "INLINE_FRAGMENT",
            __DirectiveLocation::SCHEMA => "SCHEMA",
            __DirectiveLocation::SCALAR => "SCALAR",
            __DirectiveLocation::OBJECT => "OBJECT",
            __DirectiveLocation::FIELD_DEFINITION => "FIELD_DEFINITION",
            __DirectiveLocation::ARGUMENT_DEFINITION => "ARGUMENT_DEFINITION",
            __DirectiveLocation::INTERFACE => "INTERFACE",
            __DirectiveLocation::UNION => "UNION",
            __DirectiveLocation::ENUM => "ENUM",
            __DirectiveLocation::ENUM_VALUE => "ENUM_VALUE",
            __DirectiveLocation::INPUT_OBJECT => "INPUT_OBJECT",
            __DirectiveLocation::INPUT_FIELD_DEFINITION => "INPUT_FIELD_DEFINITION",
            __DirectiveLocation::Other(ref s) => s.as_str(),
        })
    }
}

impl<'de> ::serde::Deserialize<'de> for __DirectiveLocation {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&'de str>::deserialize(deserializer)?;
        match s {
            "QUERY" => Ok(__DirectiveLocation::QUERY),
            "MUTATION" => Ok(__DirectiveLocation::MUTATION),
            "SUBSCRIPTION" => Ok(__DirectiveLocation::SUBSCRIPTION),
            "FIELD" => Ok(__DirectiveLocation::FIELD),
            "FRAGMENT_DEFINITION" => Ok(__DirectiveLocation::FRAGMENT_DEFINITION),
            "FRAGMENT_SPREAD" => Ok(__DirectiveLocation::FRAGMENT_SPREAD),
            "INLINE_FRAGMENT" => Ok(__DirectiveLocation::INLINE_FRAGMENT),
            "SCHEMA" => Ok(__DirectiveLocation::SCHEMA),
            "SCALAR" => Ok(__DirectiveLocation::SCALAR),
            "OBJECT" => Ok(__DirectiveLocation::OBJECT),
            "FIELD_DEFINITION" => Ok(__DirectiveLocation::FIELD_DEFINITION),
            "ARGUMENT_DEFINITION" => Ok(__DirectiveLocation::ARGUMENT_DEFINITION),
            "INTERFACE" => Ok(__DirectiveLocation::INTERFACE),
            "UNION" => Ok(__DirectiveLocation::UNION),
            "ENUM" => Ok(__DirectiveLocation::ENUM),
            "ENUM_VALUE" => Ok(__DirectiveLocation::ENUM_VALUE),
            "INPUT_OBJECT" => Ok(__DirectiveLocation::INPUT_OBJECT),
            "INPUT_FIELD_DEFINITION" => Ok(__DirectiveLocation::INPUT_FIELD_DEFINITION),
            _ => Ok(__DirectiveLocation::Other(s.to_string())),
        }
    }
}

#[derive(Debug)]
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

impl ::serde::Serialize for __TypeKind {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            __TypeKind::SCALAR => "SCALAR",
            __TypeKind::OBJECT => "OBJECT",
            __TypeKind::INTERFACE => "INTERFACE",
            __TypeKind::UNION => "UNION",
            __TypeKind::ENUM => "ENUM",
            __TypeKind::INPUT_OBJECT => "INPUT_OBJECT",
            __TypeKind::LIST => "LIST",
            __TypeKind::NON_NULL => "NON_NULL",
            __TypeKind::Other(ref s) => s.as_str(),
        })
    }
}

impl<'de> ::serde::Deserialize<'de> for __TypeKind {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&'de str>::deserialize(deserializer)?;
        match s {
            "SCALAR" => Ok(__TypeKind::SCALAR),
            "OBJECT" => Ok(__TypeKind::OBJECT),
            "INTERFACE" => Ok(__TypeKind::INTERFACE),
            "UNION" => Ok(__TypeKind::UNION),
            "ENUM" => Ok(__TypeKind::ENUM),
            "INPUT_OBJECT" => Ok(__TypeKind::INPUT_OBJECT),
            "LIST" => Ok(__TypeKind::LIST),
            "NON_NULL" => Ok(__TypeKind::NON_NULL),
            _ => Ok(__TypeKind::Other(s.to_string())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FullType {
    kind: Option<__TypeKind>,
    name: Option<String>,
    description: Option<String>,
    fields: Option<Vec<Option<FullTypeFields>>>,
    inputFields: Option<Vec<Option<FullTypeInputFields>>>,
    interfaces: Option<Vec<Option<FullTypeInterfaces>>>,
    enumValues: Option<Vec<Option<FullTypeEnumValues>>>,
    possibleTypes: Option<Vec<Option<FullTypePossibleTypes>>>,
}

#[derive(Debug, Deserialize)]
pub struct FullTypeFieldsArgs {
    #[serde(flatten)]
    input_value: InputValue,
}

#[derive(Debug, Deserialize)]
pub struct FullTypeFieldsType {
    #[serde(flatten)]
    type_ref: TypeRef,
}

#[derive(Debug, Deserialize)]
pub struct FullTypeFields {
    name: Option<String>,
    description: Option<String>,
    args: Option<Vec<Option<FullTypeFieldsArgs>>>,
    #[serde(rename = "type")]
    type_: Option<FullTypeFieldsType>,
    isDeprecated: Option<Boolean>,
    deprecationReason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FullTypeInputFields {
    #[serde(flatten)]
    input_value: InputValue,
}

#[derive(Debug, Deserialize)]
pub struct FullTypeInterfaces {
    #[serde(flatten)]
    type_ref: TypeRef,
}

#[derive(Debug, Deserialize)]
pub struct FullTypeEnumValues {
    name: Option<String>,
    description: Option<String>,
    isDeprecated: Option<Boolean>,
    deprecationReason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FullTypePossibleTypes {
    #[serde(flatten)]
    type_ref: TypeRef,
}

#[derive(Debug, Deserialize)]
pub struct InputValue {
    name: Option<String>,
    description: Option<String>,
    #[serde(rename = "type")]
    type_: Option<InputValueType>,
    defaultValue: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InputValueType {
    #[serde(flatten)]
    type_ref: TypeRef,
}

#[derive(Debug, Deserialize)]
pub struct TypeRef {
    kind: Option<__TypeKind>,
    name: Option<String>,
    ofType: Option<TypeRefOfType>,
}

#[derive(Debug, Deserialize)]
pub struct TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfTypeOfType {
    kind: Option<__TypeKind>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfType {
    kind: Option<__TypeKind>,
    name: Option<String>,
    ofType: Option<TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfTypeOfType>,
}

#[derive(Debug, Deserialize)]
pub struct TypeRefOfTypeOfTypeOfTypeOfTypeOfType {
    kind: Option<__TypeKind>,
    name: Option<String>,
    ofType: Option<TypeRefOfTypeOfTypeOfTypeOfTypeOfTypeOfType>,
}

#[derive(Debug, Deserialize)]
pub struct TypeRefOfTypeOfTypeOfTypeOfType {
    kind: Option<__TypeKind>,
    name: Option<String>,
    ofType: Option<TypeRefOfTypeOfTypeOfTypeOfTypeOfType>,
}

#[derive(Debug, Deserialize)]
pub struct TypeRefOfTypeOfTypeOfType {
    kind: Option<__TypeKind>,
    name: Option<String>,
    ofType: Option<TypeRefOfTypeOfTypeOfTypeOfType>,
}

#[derive(Debug, Deserialize)]
pub struct TypeRefOfTypeOfType {
    kind: Option<__TypeKind>,
    name: Option<String>,
    ofType: Option<TypeRefOfTypeOfTypeOfType>,
}

#[derive(Debug, Deserialize)]
pub struct TypeRefOfType {
    kind: Option<__TypeKind>,
    name: Option<String>,
    ofType: Option<TypeRefOfTypeOfType>,
}

#[derive(Debug, Deserialize)]
pub struct RustIntrospectionQuerySchemaQueryType {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RustIntrospectionQuerySchemaMutationType {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RustIntrospectionQuerySchemaSubscriptionType {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RustIntrospectionQuerySchemaTypes {
    #[serde(flatten)]
    full_type: FullType,
}

#[derive(Debug, Deserialize)]
pub struct RustIntrospectionQuerySchemaDirectivesArgs {
    #[serde(flatten)]
    input_value: InputValue,
}

#[derive(Debug, Deserialize)]
pub struct RustIntrospectionQuerySchemaDirectives {
    name: Option<String>,
    description: Option<String>,
    locations: Option<Vec<Option<__DirectiveLocation>>>,
    args: Option<Vec<Option<RustIntrospectionQuerySchemaDirectivesArgs>>>,
}

#[derive(Debug, Deserialize)]
pub struct RustIntrospectionQuerySchema {
    queryType: Option<RustIntrospectionQuerySchemaQueryType>,
    mutationType: Option<RustIntrospectionQuerySchemaMutationType>,
    subscriptionType: Option<RustIntrospectionQuerySchemaSubscriptionType>,
    types: Option<Vec<Option<RustIntrospectionQuerySchemaTypes>>>,
    directives: Option<Vec<Option<RustIntrospectionQuerySchemaDirectives>>>,
}

#[derive(Debug, Deserialize)]
pub struct Schema {
    __Schema: Option<RustIntrospectionQuerySchema>,
}
