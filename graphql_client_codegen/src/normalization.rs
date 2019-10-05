use heck::{CamelCase, SnakeCase};
use std::borrow::Cow;

/// Normalization conventions available for generated code.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Normalization {
    /// Use naming conventions from the schema.
    None,
    /// Use Rust naming conventions for generated code.
    Rust,
}

impl Normalization {
    fn camel_case(self, name: Cow<'_, str>) -> Cow<'_, str> {
        match self {
            Self::None => name,
            Self::Rust => name.to_camel_case().into(),
        }
    }

    fn snake_case(self, name: Cow<'_, str>) -> Cow<'_, str> {
        match self {
            Self::None => name,
            Self::Rust => name.to_snake_case().into(),
        }
    }

    pub(crate) fn operation<'a, S>(self, op: S) -> Cow<'a, str>
    where
        S: Into<Cow<'a, str>>,
    {
        self.camel_case(op.into())
    }

    pub(crate) fn enum_variant<'a, S>(self, enm: S) -> Cow<'a, str>
    where
        S: Into<Cow<'a, str>>,
    {
        self.camel_case(enm.into())
    }

    pub(crate) fn enum_name<'a, S>(self, enm: S) -> Cow<'a, str>
    where
        S: Into<Cow<'a, str>>,
    {
        self.camel_case(enm.into())
    }

    fn field_type_impl(self, fty: Cow<'_, str>) -> Cow<'_, str> {
        if fty == "ID" || fty.starts_with("__") {
            fty
        } else {
            self.camel_case(fty)
        }
    }

    pub(crate) fn field_type<'a, S>(self, fty: S) -> Cow<'a, str>
    where
        S: Into<Cow<'a, str>>,
    {
        self.field_type_impl(fty.into())
    }

    pub(crate) fn field_name<'a, S>(self, fnm: S) -> Cow<'a, str>
    where
        S: Into<Cow<'a, str>>,
    {
        self.snake_case(fnm.into())
    }

    pub(crate) fn input_name<'a, S>(self, inm: S) -> Cow<'a, str>
    where
        S: Into<Cow<'a, str>>,
    {
        self.camel_case(inm.into())
    }

    pub(crate) fn scalar_name<'a, S>(self, snm: S) -> Cow<'a, str>
    where
        S: Into<Cow<'a, str>>,
    {
        self.camel_case(snm.into())
    }
}

impl std::str::FromStr for Normalization {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.trim() {
            "none" => Ok(Normalization::None),
            "rust" => Ok(Normalization::Rust),
            _ => Err(()),
        }
    }
}
