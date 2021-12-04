use crate::{
    codegen::render_derives, codegen_options::GraphQLClientCodegenOptions, query::BoundQuery,
};
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
    all_used_types: &'a crate::query::UsedTypes,
    options: &'a GraphQLClientCodegenOptions,
    query: BoundQuery<'schema>,
) -> impl Iterator<Item = TokenStream> + 'a {
    let traits = options
        .all_response_derives()
        .chain(options.all_variable_derives())
        .filter(|d| !&["Serialize", "Deserialize", "Default"].contains(d))
        // Use BTreeSet instead of HashSet for a stable ordering.
        .collect::<std::collections::BTreeSet<_>>();
    let derives = render_derives(traits.into_iter());
    let normalization = options.normalization();

    all_used_types.enums(query.schema)
        .filter(move |(_id, r#enum)| !options.extern_enums().contains(&r#enum.name))
        .map(move |(_id, r#enum)| {
        let variant_names: Vec<TokenStream> = r#enum
            .variants
            .iter()
            .map(|v| {
                let safe_name = super::shared::keyword_replace(v.as_str());
                let name = normalization.enum_variant(safe_name.as_ref());
                let name = Ident::new(&name, Span::call_site());

                quote!(#name)
            })
            .collect();
        let variant_names = &variant_names;
        let name_ident = normalization.enum_name(r#enum.name.as_str());
        let name_ident = Ident::new(&name_ident, Span::call_site());
        let constructors: Vec<_> = r#enum
            .variants
            .iter()
            .map(|v| {
                let safe_name = super::shared::keyword_replace(v);
                let name = normalization.enum_variant(safe_name.as_ref());
                let v = Ident::new(&name, Span::call_site());

                quote!(#name_ident::#v)
            })
            .collect();
        let constructors = &constructors;
        let variant_str: Vec<&str> = r#enum.variants.iter().map(|s| s.as_str()).collect();
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
                    let s: String = ::serde::Deserialize::deserialize(deserializer)?;

                    match s.as_str() {
                        #(#variant_str => Ok(#constructors),)*
                        _ => Ok(#name::Other(s)),
                    }
                }
            }
        }})
}
