#[macro_use]
extern crate serde_derive;
extern crate serde;

    pub enum RustEnum___DirectiveLocation {
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
    impl ::serde::Serialize for RustEnum___DirectiveLocation {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                RustEnum___DirectiveLocation::QUERY => "QUERY",
                RustEnum___DirectiveLocation::MUTATION => "MUTATION",
                RustEnum___DirectiveLocation::SUBSCRIPTION => "SUBSCRIPTION",
                RustEnum___DirectiveLocation::FIELD => "FIELD",
                RustEnum___DirectiveLocation::FRAGMENT_DEFINITION => "FRAGMENT_DEFINITION",
                RustEnum___DirectiveLocation::FRAGMENT_SPREAD => "FRAGMENT_SPREAD",
                RustEnum___DirectiveLocation::INLINE_FRAGMENT => "INLINE_FRAGMENT",
                RustEnum___DirectiveLocation::SCHEMA => "SCHEMA",
                RustEnum___DirectiveLocation::SCALAR => "SCALAR",
                RustEnum___DirectiveLocation::OBJECT => "OBJECT",
                RustEnum___DirectiveLocation::FIELD_DEFINITION => "FIELD_DEFINITION",
                RustEnum___DirectiveLocation::ARGUMENT_DEFINITION => "ARGUMENT_DEFINITION",
                RustEnum___DirectiveLocation::INTERFACE => "INTERFACE",
                RustEnum___DirectiveLocation::UNION => "UNION",
                RustEnum___DirectiveLocation::ENUM => "ENUM",
                RustEnum___DirectiveLocation::ENUM_VALUE => "ENUM_VALUE",
                RustEnum___DirectiveLocation::INPUT_OBJECT => "INPUT_OBJECT",
                RustEnum___DirectiveLocation::INPUT_FIELD_DEFINITION => "INPUT_FIELD_DEFINITION",
                RustEnum___DirectiveLocation::Other(ref s) => s.as_str(),
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RustEnum___DirectiveLocation {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <&'de str>::deserialize(deserializer)?;
            match s {
                "QUERY" => Ok(RustEnum___DirectiveLocation::QUERY),
                "MUTATION" => Ok(RustEnum___DirectiveLocation::MUTATION),
                "SUBSCRIPTION" => Ok(RustEnum___DirectiveLocation::SUBSCRIPTION),
                "FIELD" => Ok(RustEnum___DirectiveLocation::FIELD),
                "FRAGMENT_DEFINITION" => Ok(RustEnum___DirectiveLocation::FRAGMENT_DEFINITION),
                "FRAGMENT_SPREAD" => Ok(RustEnum___DirectiveLocation::FRAGMENT_SPREAD),
                "INLINE_FRAGMENT" => Ok(RustEnum___DirectiveLocation::INLINE_FRAGMENT),
                "SCHEMA" => Ok(RustEnum___DirectiveLocation::SCHEMA),
                "SCALAR" => Ok(RustEnum___DirectiveLocation::SCALAR),
                "OBJECT" => Ok(RustEnum___DirectiveLocation::OBJECT),
                "FIELD_DEFINITION" => Ok(RustEnum___DirectiveLocation::FIELD_DEFINITION),
                "ARGUMENT_DEFINITION" => Ok(RustEnum___DirectiveLocation::ARGUMENT_DEFINITION),
                "INTERFACE" => Ok(RustEnum___DirectiveLocation::INTERFACE),
                "UNION" => Ok(RustEnum___DirectiveLocation::UNION),
                "ENUM" => Ok(RustEnum___DirectiveLocation::ENUM),
                "ENUM_VALUE" => Ok(RustEnum___DirectiveLocation::ENUM_VALUE),
                "INPUT_OBJECT" => Ok(RustEnum___DirectiveLocation::INPUT_OBJECT),
                "INPUT_FIELD_DEFINITION" => {
                    Ok(RustEnum___DirectiveLocation::INPUT_FIELD_DEFINITION)
                }
                _ => Ok(RustEnum___DirectiveLocation::Other(s.to_string())),
            }
        }
    }
    pub enum RustEnum___TypeKind {
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
    impl ::serde::Serialize for RustEnum___TypeKind {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                RustEnum___TypeKind::SCALAR => "SCALAR",
                RustEnum___TypeKind::OBJECT => "OBJECT",
                RustEnum___TypeKind::INTERFACE => "INTERFACE",
                RustEnum___TypeKind::UNION => "UNION",
                RustEnum___TypeKind::ENUM => "ENUM",
                RustEnum___TypeKind::INPUT_OBJECT => "INPUT_OBJECT",
                RustEnum___TypeKind::LIST => "LIST",
                RustEnum___TypeKind::NON_NULL => "NON_NULL",
                RustEnum___TypeKind::Other(ref s) => s.as_str(),
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RustEnum___TypeKind {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <&'de str>::deserialize(deserializer)?;
            match s {
                "SCALAR" => Ok(RustEnum___TypeKind::SCALAR),
                "OBJECT" => Ok(RustEnum___TypeKind::OBJECT),
                "INTERFACE" => Ok(RustEnum___TypeKind::INTERFACE),
                "UNION" => Ok(RustEnum___TypeKind::UNION),
                "ENUM" => Ok(RustEnum___TypeKind::ENUM),
                "INPUT_OBJECT" => Ok(RustEnum___TypeKind::INPUT_OBJECT),
                "LIST" => Ok(RustEnum___TypeKind::LIST),
                "NON_NULL" => Ok(RustEnum___TypeKind::NON_NULL),
                _ => Ok(RustEnum___TypeKind::Other(s.to_string())),
            }
        }
    }
    pub struct FullType;
    pub struct InputValue;
    pub struct TypeRef;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for TypeRef {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                TypeRef => {
                    let mut debug_trait_builder = f.debug_tuple("TypeRef");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_TypeRef: () = {
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TypeRef {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor;
                impl<'de> _serde::de::Visitor<'de> for __Visitor {
                    type Value = TypeRef;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "unit struct TypeRef")
                    }
                    #[inline]
                    fn visit_unit<__E>(self) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(TypeRef)
                    }
                }
                _serde::Deserializer::deserialize_unit_struct(__deserializer, "TypeRef", __Visitor)
            }
        }
    };
    pub struct RustIntrospectionQuerySchemaQueryType {
        name: Option<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for RustIntrospectionQuerySchemaQueryType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                RustIntrospectionQuerySchemaQueryType {
                    name: ref __self_0_0,
                } => {
                    let mut debug_trait_builder =
                        f.debug_struct("RustIntrospectionQuerySchemaQueryType");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_RustIntrospectionQuerySchemaQueryType: () = {
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RustIntrospectionQuerySchemaQueryType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<RustIntrospectionQuerySchemaQueryType>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RustIntrospectionQuerySchemaQueryType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "struct RustIntrospectionQuerySchemaQueryType",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct RustIntrospectionQuerySchemaQueryType with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchemaQueryType { name: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<
                            Option<String>,
                        > = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("name") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchemaQueryType { name: __field0 })
                    }
                }
                const FIELDS: &'static [&'static str] = &["name"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RustIntrospectionQuerySchemaQueryType",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<RustIntrospectionQuerySchemaQueryType>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    pub struct RustIntrospectionQuerySchemaMutationType {
        name: Option<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for RustIntrospectionQuerySchemaMutationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                RustIntrospectionQuerySchemaMutationType {
                    name: ref __self_0_0,
                } => {
                    let mut debug_trait_builder =
                        f.debug_struct("RustIntrospectionQuerySchemaMutationType");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_RustIntrospectionQuerySchemaMutationType: () = {
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RustIntrospectionQuerySchemaMutationType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<RustIntrospectionQuerySchemaMutationType>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RustIntrospectionQuerySchemaMutationType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "struct RustIntrospectionQuerySchemaMutationType",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"struct RustIntrospectionQuerySchemaMutationType with 1 element"));
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchemaMutationType {
                            name: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<
                            Option<String>,
                        > = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("name") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchemaMutationType {
                            name: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["name"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RustIntrospectionQuerySchemaMutationType",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<
                            RustIntrospectionQuerySchemaMutationType,
                        >,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    pub struct RustIntrospectionQuerySchemaSubscriptionType {
        name: Option<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for RustIntrospectionQuerySchemaSubscriptionType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                RustIntrospectionQuerySchemaSubscriptionType {
                    name: ref __self_0_0,
                } => {
                    let mut debug_trait_builder =
                        f.debug_struct("RustIntrospectionQuerySchemaSubscriptionType");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_RustIntrospectionQuerySchemaSubscriptionType: () = {
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RustIntrospectionQuerySchemaSubscriptionType {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker:
                        _serde::export::PhantomData<RustIntrospectionQuerySchemaSubscriptionType>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RustIntrospectionQuerySchemaSubscriptionType;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "struct RustIntrospectionQuerySchemaSubscriptionType",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"struct RustIntrospectionQuerySchemaSubscriptionType with 1 element"));
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchemaSubscriptionType {
                            name: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<
                            Option<String>,
                        > = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("name") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchemaSubscriptionType {
                            name: __field0,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["name"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RustIntrospectionQuerySchemaSubscriptionType",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<
                            RustIntrospectionQuerySchemaSubscriptionType,
                        >,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    pub struct RustIntrospectionQuerySchemaTypes {
        #[serde(flatten)]
        full_type: FullType,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for RustIntrospectionQuerySchemaTypes {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                RustIntrospectionQuerySchemaTypes {
                    full_type: ref __self_0_0,
                } => {
                    let mut debug_trait_builder =
                        f.debug_struct("RustIntrospectionQuerySchemaTypes");
                    let _ = debug_trait_builder.field("full_type", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_RustIntrospectionQuerySchemaTypes: () = {
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RustIntrospectionQuerySchemaTypes {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __other(_serde::private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::Bool(
                            __value,
                        )))
                    }
                    fn visit_i8<__E>(self, __value: i8) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::I16(
                            __value,
                        )))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::I32(
                            __value,
                        )))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::I64(
                            __value,
                        )))
                    }
                    fn visit_u8<__E>(self, __value: u8) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::U16(
                            __value,
                        )))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::U32(
                            __value,
                        )))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::U64(
                            __value,
                        )))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::F32(
                            __value,
                        )))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::F64(
                            __value,
                        )))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::Char(
                            __value,
                        )))
                    }
                    fn visit_unit<__E>(self) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::Unit))
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = _serde::private::de::Content::Str(__value);
                                _serde::export::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = _serde::private::de::Content::Bytes(__value);
                                _serde::export::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value =
                                    _serde::private::de::Content::String(__value.to_string());
                                _serde::export::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value =
                                    _serde::private::de::Content::ByteBuf(__value.to_vec());
                                _serde::export::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<RustIntrospectionQuerySchemaTypes>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RustIntrospectionQuerySchemaTypes;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "struct RustIntrospectionQuerySchemaTypes",
                        )
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __collect = _serde::export::Vec::<
                            _serde::export::Option<(
                                _serde::private::de::Content,
                                _serde::private::de::Content,
                            )>,
                        >::new();
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__other(__name) => {
                                    __collect.push(_serde::export::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field0: FullType = match _serde::de::Deserialize::deserialize(
                            _serde::private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::export::PhantomData,
                            ),
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchemaTypes {
                            full_type: __field0,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::export::PhantomData::<RustIntrospectionQuerySchemaTypes>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    pub struct RustIntrospectionQuerySchemaDirectivesArgs {
        #[serde(flatten)]
        input_value: InputValue,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for RustIntrospectionQuerySchemaDirectivesArgs {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                RustIntrospectionQuerySchemaDirectivesArgs {
                    input_value: ref __self_0_0,
                } => {
                    let mut debug_trait_builder =
                        f.debug_struct("RustIntrospectionQuerySchemaDirectivesArgs");
                    let _ = debug_trait_builder.field("input_value", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_RustIntrospectionQuerySchemaDirectivesArgs: () = {
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RustIntrospectionQuerySchemaDirectivesArgs {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field<'de> {
                    __other(_serde::private::de::Content<'de>),
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::Bool(
                            __value,
                        )))
                    }
                    fn visit_i8<__E>(self, __value: i8) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::I8(
                            __value,
                        )))
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::I16(
                            __value,
                        )))
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::I32(
                            __value,
                        )))
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::I64(
                            __value,
                        )))
                    }
                    fn visit_u8<__E>(self, __value: u8) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::U8(
                            __value,
                        )))
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::U16(
                            __value,
                        )))
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::U32(
                            __value,
                        )))
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::U64(
                            __value,
                        )))
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::F32(
                            __value,
                        )))
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::F64(
                            __value,
                        )))
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::Char(
                            __value,
                        )))
                    }
                    fn visit_unit<__E>(self) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::export::Ok(__Field::__other(_serde::private::de::Content::Unit))
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = _serde::private::de::Content::Str(__value);
                                _serde::export::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value = _serde::private::de::Content::Bytes(__value);
                                _serde::export::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value =
                                    _serde::private::de::Content::String(__value.to_string());
                                _serde::export::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            _ => {
                                let __value =
                                    _serde::private::de::Content::ByteBuf(__value.to_vec());
                                _serde::export::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<RustIntrospectionQuerySchemaDirectivesArgs>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RustIntrospectionQuerySchemaDirectivesArgs;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "struct RustIntrospectionQuerySchemaDirectivesArgs",
                        )
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __collect = _serde::export::Vec::<
                            _serde::export::Option<(
                                _serde::private::de::Content,
                                _serde::private::de::Content,
                            )>,
                        >::new();
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__other(__name) => {
                                    __collect.push(_serde::export::Some((
                                        __name,
                                        match _serde::de::MapAccess::next_value(&mut __map) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    )));
                                }
                            }
                        }
                        let __field0: InputValue = match _serde::de::Deserialize::deserialize(
                            _serde::private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::export::PhantomData,
                            ),
                        ) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchemaDirectivesArgs {
                            input_value: __field0,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::export::PhantomData::<
                            RustIntrospectionQuerySchemaDirectivesArgs,
                        >,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    pub struct RustIntrospectionQuerySchemaDirectives {
        name: Option<String>,
        description: Option<String>,
        locations: Option<Vec<Option<RustEnum___DirectiveLocation>>>,
        args: Option<Vec<Option<RustIntrospectionQuerySchemaDirectivesArgs>>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for RustIntrospectionQuerySchemaDirectives {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                RustIntrospectionQuerySchemaDirectives {
                    name: ref __self_0_0,
                    description: ref __self_0_1,
                    locations: ref __self_0_2,
                    args: ref __self_0_3,
                } => {
                    let mut debug_trait_builder =
                        f.debug_struct("RustIntrospectionQuerySchemaDirectives");
                    let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("description", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("locations", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("args", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_RustIntrospectionQuerySchemaDirectives: () = {
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RustIntrospectionQuerySchemaDirectives {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 4",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::export::Ok(__Field::__field0),
                            "description" => _serde::export::Ok(__Field::__field1),
                            "locations" => _serde::export::Ok(__Field::__field2),
                            "args" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::export::Ok(__Field::__field0),
                            b"description" => _serde::export::Ok(__Field::__field1),
                            b"locations" => _serde::export::Ok(__Field::__field2),
                            b"args" => _serde::export::Ok(__Field::__field3),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<RustIntrospectionQuerySchemaDirectives>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RustIntrospectionQuerySchemaDirectives;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "struct RustIntrospectionQuerySchemaDirectives",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"struct RustIntrospectionQuerySchemaDirectives with 4 elements"));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                     &"struct RustIntrospectionQuerySchemaDirectives with 4 elements"));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<Vec<Option<RustEnum___DirectiveLocation>>>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                     &"struct RustIntrospectionQuerySchemaDirectives with 4 elements"));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<Vec<Option<RustIntrospectionQuerySchemaDirectivesArgs>>>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                     &"struct RustIntrospectionQuerySchemaDirectives with 4 elements"));
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchemaDirectives {
                            name: __field0,
                            description: __field1,
                            locations: __field2,
                            args: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<
                            Option<String>,
                        > = _serde::export::None;
                        let mut __field1: _serde::export::Option<
                            Option<String>,
                        > = _serde::export::None;
                        let mut __field2: _serde::export::Option<
                            Option<Vec<Option<RustEnum___DirectiveLocation>>>,
                        > = _serde::export::None;
                        let mut __field3: _serde::export::Option<
                            Option<Vec<Option<RustIntrospectionQuerySchemaDirectivesArgs>>>,
                        > = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "name",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "description",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<Option<String>>(
                                            &mut __map,
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "locations",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<Vec<Option<RustEnum___DirectiveLocation>>>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "args",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<
                                                Vec<
                                                    Option<
                                                        RustIntrospectionQuerySchemaDirectivesArgs,
                                                    >,
                                                >,
                                            >,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("name") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("description") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("locations") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("args") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchemaDirectives {
                            name: __field0,
                            description: __field1,
                            locations: __field2,
                            args: __field3,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["name", "description", "locations", "args"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RustIntrospectionQuerySchemaDirectives",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<RustIntrospectionQuerySchemaDirectives>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    pub struct RustIntrospectionQuerySchema {
        queryType: Option<RustIntrospectionQuerySchemaQueryType>,
        mutationType: Option<RustIntrospectionQuerySchemaMutationType>,
        subscriptionType: Option<RustIntrospectionQuerySchemaSubscriptionType>,
        types: Option<Vec<Option<RustIntrospectionQuerySchemaTypes>>>,
        directives: Option<Vec<Option<RustIntrospectionQuerySchemaDirectives>>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for RustIntrospectionQuerySchema {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                RustIntrospectionQuerySchema {
                    queryType: ref __self_0_0,
                    mutationType: ref __self_0_1,
                    subscriptionType: ref __self_0_2,
                    types: ref __self_0_3,
                    directives: ref __self_0_4,
                } => {
                    let mut debug_trait_builder = f.debug_struct("RustIntrospectionQuerySchema");
                    let _ = debug_trait_builder.field("queryType", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("mutationType", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("subscriptionType", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("types", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("directives", &&(*__self_0_4));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_RustIntrospectionQuerySchema: () = {
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RustIntrospectionQuerySchema {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 5",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "queryType" => _serde::export::Ok(__Field::__field0),
                            "mutationType" => _serde::export::Ok(__Field::__field1),
                            "subscriptionType" => _serde::export::Ok(__Field::__field2),
                            "types" => _serde::export::Ok(__Field::__field3),
                            "directives" => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"queryType" => _serde::export::Ok(__Field::__field0),
                            b"mutationType" => _serde::export::Ok(__Field::__field1),
                            b"subscriptionType" => _serde::export::Ok(__Field::__field2),
                            b"types" => _serde::export::Ok(__Field::__field3),
                            b"directives" => _serde::export::Ok(__Field::__field4),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<RustIntrospectionQuerySchema>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RustIntrospectionQuerySchema;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "struct RustIntrospectionQuerySchema",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Option<RustIntrospectionQuerySchemaQueryType>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct RustIntrospectionQuerySchema with 5 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<RustIntrospectionQuerySchemaMutationType>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct RustIntrospectionQuerySchema with 5 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<RustIntrospectionQuerySchemaSubscriptionType>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct RustIntrospectionQuerySchema with 5 elements",
                                ));
                            }
                        };
                        let __field3 = match match _serde::de::SeqAccess::next_element::<
                            Option<Vec<Option<RustIntrospectionQuerySchemaTypes>>>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct RustIntrospectionQuerySchema with 5 elements",
                                ));
                            }
                        };
                        let __field4 = match match _serde::de::SeqAccess::next_element::<
                            Option<Vec<Option<RustIntrospectionQuerySchemaDirectives>>>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct RustIntrospectionQuerySchema with 5 elements",
                                ));
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchema {
                            queryType: __field0,
                            mutationType: __field1,
                            subscriptionType: __field2,
                            types: __field3,
                            directives: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<
                            Option<RustIntrospectionQuerySchemaQueryType>,
                        > = _serde::export::None;
                        let mut __field1: _serde::export::Option<
                            Option<RustIntrospectionQuerySchemaMutationType>,
                        > = _serde::export::None;
                        let mut __field2: _serde::export::Option<
                            Option<RustIntrospectionQuerySchemaSubscriptionType>,
                        > = _serde::export::None;
                        let mut __field3: _serde::export::Option<
                            Option<Vec<Option<RustIntrospectionQuerySchemaTypes>>>,
                        > = _serde::export::None;
                        let mut __field4: _serde::export::Option<
                            Option<Vec<Option<RustIntrospectionQuerySchemaDirectives>>>,
                        > = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "queryType",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<RustIntrospectionQuerySchemaQueryType>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "mutationType",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<RustIntrospectionQuerySchemaMutationType>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "subscriptionType",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<RustIntrospectionQuerySchemaSubscriptionType>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "types",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<Vec<Option<RustIntrospectionQuerySchemaTypes>>>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "directives",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<
                                                Vec<Option<RustIntrospectionQuerySchemaDirectives>>,
                                            >,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("queryType") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::export::Some(__field1) => __field1,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("mutationType") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::export::Some(__field2) => __field2,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("subscriptionType") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::export::Some(__field3) => __field3,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("types") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::export::Some(__field4) => __field4,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("directives") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(RustIntrospectionQuerySchema {
                            queryType: __field0,
                            mutationType: __field1,
                            subscriptionType: __field2,
                            types: __field3,
                            directives: __field4,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &[
                    "queryType",
                    "mutationType",
                    "subscriptionType",
                    "types",
                    "directives",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RustIntrospectionQuerySchema",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<RustIntrospectionQuerySchema>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    pub struct Variables;
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Variables: () = {
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
        #[automatically_derived]
        impl _serde::Serialize for Variables {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_unit_struct(__serializer, "Variables")
            }
        }
    };
    pub struct ResponseData {
        __Schema: Option<RustIntrospectionQuerySchema>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_ResponseData: () = {
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ResponseData {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "__Schema" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"__Schema" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<ResponseData>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ResponseData;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "struct ResponseData")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            Option<RustIntrospectionQuerySchema>,
                        >(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ResponseData with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(ResponseData { __Schema: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::export::Option<
                            Option<RustIntrospectionQuerySchema>,
                        > = _serde::export::None;
                        while let _serde::export::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0) {
                                        return _serde::export::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "__Schema",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::export::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<RustIntrospectionQuerySchema>,
                                        >(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::export::Some(__field0) => __field0,
                            _serde::export::None => {
                                match _serde::private::de::missing_field("__Schema") {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::export::Ok(ResponseData { __Schema: __field0 })
                    }
                }
                const FIELDS: &'static [&'static str] = &["__Schema"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ResponseData",
                    FIELDS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<ResponseData>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
}
impl<'de> ::graphql_query::GraphQLQuery<'de> for IntrospectionQuery {
    type Variables = introspection_query::Variables;
    type ResponseData = introspection_query::ResponseData;
    fn build_query(
        variables: Self::Variables,
    ) -> ::graphql_query::GraphQLQueryBody<Self::Variables> {
        ::graphql_query::GraphQLQueryBody {
            variables,
            query: introspection_query::QUERY,
        }
    }
}
