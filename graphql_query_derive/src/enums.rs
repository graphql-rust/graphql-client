use proc_macro2::{Ident, Span, TokenStream};

pub const ENUMS_PREFIX: &str = "";

#[derive(Debug, PartialEq)]
pub struct GqlEnum {
    pub description: Option<String>,
    pub name: String,
    pub variants: Vec<String>,
}

impl GqlEnum {
    pub fn to_rust(&self) -> TokenStream {
        let variants: Vec<Ident> = self
            .variants
            .iter()
            .map(|v| Ident::new(v, Span::call_site()))
            .collect();
        let variants = &variants;
        let name_ident = Ident::new(&format!("{}{}", ENUMS_PREFIX, self.name), Span::call_site());
        let constructors: Vec<_> = variants.iter().map(|v| quote!(#name_ident::#v)).collect();
        let constructors = &constructors;
        let variant_str = &self.variants;

        let name = name_ident.clone();

        quote! {
            #[derive(Debug)]
            pub enum #name {
                #(#variants,)*
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
