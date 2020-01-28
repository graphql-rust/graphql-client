use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::cell::Cell;

pub const ENUMS_PREFIX: &str = "";

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant<'schema> {
    pub description: Option<&'schema str>,
    pub name: &'schema str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GqlEnum<'schema> {
    pub description: Option<&'schema str>,
    pub name: &'schema str,
    pub variants: Vec<EnumVariant<'schema>>,
    pub is_required: Cell<bool>,
}

impl<'schema> GqlEnum<'schema> {
    pub(crate) fn to_rust(&self, query_context: &crate::query::QueryContext<'_>) -> TokenStream {
        let derives = query_context.response_enum_derives();
        let norm = query_context.normalization;
        let variant_names: Vec<TokenStream> = self
            .variants
            .iter()
            .map(|v| {
                let name = norm.enum_variant(crate::shared::keyword_replace(&v.name));
                let name = Ident::new(&name, Span::call_site());

                let description = &v.description;
                let description = description.as_ref().map(|d| quote!(#[doc = #d]));

                quote!(#description #name)
            })
            .collect();
        let variant_names = &variant_names;
        let name_ident = norm.enum_name(format!("{}{}", ENUMS_PREFIX, self.name));
        let name_ident = Ident::new(&name_ident, Span::call_site());
        let constructors: Vec<_> = self
            .variants
            .iter()
            .map(|v| {
                let name = norm.enum_variant(crate::shared::keyword_replace(&v.name));
                let v = Ident::new(&name, Span::call_site());

                quote!(#name_ident::#v)
            })
            .collect();
        let constructors = &constructors;
        let variant_str: Vec<&str> = self.variants.iter().map(|v| v.name).collect();
        let variant_str = &variant_str;

        let name = name_ident;

        quote! {
            #derives
            pub enum #name {
                #(#variant_names,)*
                Other(String),
            }

            impl ::serde::Serialize for #name {
                fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                    ser.serialize_str(match *self {
                        #(#constructors => #variant_str,)*
                        #name::Other(ref s) => &s,
                    })
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name {
                fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    let s = <String>::deserialize(deserializer)?;

                    match s.as_str() {
                        #(#variant_str => Ok(#constructors),)*
                        _ => Ok(#name::Other(s)),
                    }
                }
            }
        }
    }
}
