use proc_macro2::{Ident, Span, TokenStream};

#[derive(Debug, PartialEq)]
pub struct GqlEnum {
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
        let name_ident = Ident::new(&self.name, Span::call_site());
        let constructors: Vec<_> = variants.iter().map(|v| quote!(#name_ident::#v)).collect();
        let constructors = &constructors;
        let variant_str = &self.variants;

        let name = Ident::new(&self.name, Span::call_site());

        quote! {
            pub enum #name<'a> {
                #(#variants,)*
                Other(&'a str),
            }

            impl ::serde::Serialize for #name {
                fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                    serializer.serialize_str(match *self {
                        #(#constructors => #variant_str,)*,
                        #name::Other(s) => s,
                    })
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name {
                fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    let s = <&'de str>::deserialize(deserializer)?;

                    match s {
                        #(#variant_str => Ok(#constructors),)*
                        _ => #name::Other(s),
                    }
                }
            }
        }
    }
}
