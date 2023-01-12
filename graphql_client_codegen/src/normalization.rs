use heck::ToUpperCamelCase;
use std::borrow::Cow;

/// Normalization conventions available for generated code.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Normalization {
    /// Use naming conventions from the schema.
    None,
    /// Use Rust naming conventions for generated code.
    Rust,
}

impl Normalization {
    fn camel_case(self, name: &str) -> Cow<'_, str> {
        match self {
            Self::None => name.into(),
            Self::Rust => name.to_upper_camel_case().into(),
        }
    }

    pub(crate) fn operation(self, op: &str) -> Cow<'_, str> {
        self.camel_case(op)
    }

    pub(crate) fn enum_variant(self, enm: &str) -> Cow<'_, str> {
        self.camel_case(enm)
    }

    pub(crate) fn enum_name(self, enm: &str) -> Cow<'_, str> {
        self.camel_case(enm)
    }

    fn field_type_impl(self, fty: &str) -> Cow<'_, str> {
        if fty == "ID" || fty.starts_with("__") {
            fty.into()
        } else {
            self.camel_case(fty)
        }
    }

    pub(crate) fn field_type(self, fty: &str) -> Cow<'_, str> {
        self.field_type_impl(fty)
    }

    pub(crate) fn input_name(self, inm: &str) -> Cow<'_, str> {
        self.camel_case(inm)
    }

    pub(crate) fn scalar_name(self, snm: &str) -> Cow<'_, str> {
        self.camel_case(snm)
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
