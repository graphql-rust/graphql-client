use crate::codegen::render_derives;
use crate::codegen_options::GraphQLClientCodegenOptions;
use crate::resolution::{OperationId, OperationRef};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/**
 * About rust keyword escaping: variant_names and constructors must be escaped,
 * variant_str not.
 * Example schema:                  enum AnEnum { where \n self }
 * Generated "variant_names" enum:  pub enum AnEnum { where_, self_, Other(String), }
 * Generated serialize line: "AnEnum::where_ => "where","
 */
pub(super) fn generate_enum_definitions<'a, 'schema: 'a>(
    operation: &OperationRef<'schema>,
    all_used_types: &'a crate::resolution::UsedTypes,
    options: &'a GraphQLClientCodegenOptions,
) -> impl Iterator<Item = TokenStream> + 'a {
    let derives = render_derives(
        options
            .all_response_derives()
            .filter(|d| !&["Serialize", "Deserialize"].contains(d)),
    );
    let normalization = options.normalization();

    all_used_types.enums(operation.schema()).map(move |r#enum| {
        let variant_names: Vec<TokenStream> = r#enum
            .variants()
            .iter()
            .map(|v| {
                let name = normalization.enum_variant(crate::shared::keyword_replace(v.as_str()));
                let name = Ident::new(&name, Span::call_site());

                // let description = &v.description;
                // let description = description.as_ref().map(|d| quote!(#[doc = #d]));

                // quote!(#description #name)
                quote!(#name)
            })
            .collect();
        let variant_names = &variant_names;
        let name_ident = normalization.enum_name(r#enum.name());
        let name_ident = Ident::new(&name_ident, Span::call_site());
        let constructors: Vec<_> = r#enum
            .variants()
            .iter()
            .map(|v| {
                let name = normalization.enum_variant(crate::shared::keyword_replace(v));
                let v = Ident::new(&name, Span::call_site());

                quote!(#name_ident::#v)
            })
            .collect();
        let constructors = &constructors;
        let variant_str: Vec<&str> = r#enum.variants().iter().map(|s| s.as_str()).collect();
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
        }})
}
