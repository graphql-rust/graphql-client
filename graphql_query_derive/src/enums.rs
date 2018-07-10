use proc_macro2::{Ident, Span, TokenStream};

pub const ENUMS_PREFIX: &str = "";

#[derive(Debug, PartialEq)]
pub struct EnumVariant {
    pub description: Option<String>,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct GqlEnum {
    pub description: Option<String>,
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

impl GqlEnum {
    pub fn to_rust(&self) -> TokenStream {
        let variant_names: Vec<TokenStream> = self
            .variants
            .iter()
            .map(|v| {
                let name = Ident::new(&v.name, Span::call_site());
                let description = &v.description;
                let description = description.as_ref().map(|d| quote!(#[doc = #d]));
                quote!(#description #name)
            })
            .collect();
        let variant_names = &variant_names;
        let name_ident = Ident::new(&format!("{}{}", ENUMS_PREFIX, self.name), Span::call_site());
        let constructors: Vec<_> = self
            .variants
            .iter()
            .map(|v| {
                let v = Ident::new(&v.name, Span::call_site());
                quote!(#name_ident::#v)
            })
            .collect();
        let constructors = &constructors;
        let variant_str: Vec<&str> = self.variants.iter().map(|v| v.name.as_str()).collect();
        let variant_str = &variant_str;

        let name = name_ident.clone();

        quote! {
            #[derive(Debug)]
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
