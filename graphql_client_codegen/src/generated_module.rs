use crate::codegen_options::*;
use heck::*;
use proc_macro2::{Ident, Span, TokenStream};

/// This struct contains the parameters necessary to generate code for a given operation.
pub(crate) struct GeneratedModule<'a> {
    pub operation: &'a crate::operations::Operation<'a>,
    pub query_string: &'a str,
    pub query_document: &'a graphql_parser::query::Document,
    pub schema: &'a crate::schema::Schema<'a>,
    pub options: &'a crate::GraphQLClientCodegenOptions,
}

impl<'a> GeneratedModule<'a> {
    /// Generate the items for the variables and the response that will go inside the module.
    fn build_impls(&self) -> Result<TokenStream, failure::Error> {
        Ok(crate::codegen::response_for_query(
            &self.schema,
            &self.query_document,
            &self.operation,
            &self.options,
        )?)
    }

    /// Generate the module and all the code inside.
    pub(crate) fn to_token_stream(&self) -> Result<TokenStream, failure::Error> {
        let module_name = Ident::new(&self.operation.name.to_snake_case(), Span::call_site());
        let module_visibility = &self.options.module_visibility();
        let operation_name_literal = &self.operation.name;
        let operation_name_ident = Ident::new(&self.operation.name, Span::call_site());

        // Force cargo to refresh the generated code when the query file changes.
        let query_include = self
            .options
            .query_file()
            .map(|path| {
                let path = path.to_str();
                quote!(
                    const __QUERY_WORKAROUND: &str = include_str!(#path);
                )
            })
            .unwrap_or_else(|| quote! {});

        let query_string = &self.query_string;
        let impls = self.build_impls()?;

        let struct_declaration = match self.options.mode {
            CodegenMode::Cli => quote!(#module_visibility struct #operation_name_ident;),
            // The struct is already present in derive mode.
            CodegenMode::Derive => quote!(),
        };

        Ok(quote!(
            #struct_declaration

            #module_visibility mod #module_name {
                #![allow(non_camel_case_types)]
                #![allow(non_snake_case)]
                #![allow(dead_code)]

                use serde;

                pub const OPERATION_NAME: &'static str = #operation_name_literal;
                pub const QUERY: &'static str = #query_string;

                #query_include

                #impls

                impl ::graphql_client::GraphQLQuery for super::#operation_name_ident {
                    type Variables = Variables;
                    type ResponseData = ResponseData;

                    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
                        ::graphql_client::QueryBody {
                            variables,
                            query: QUERY,
                            operation_name: OPERATION_NAME,
                        }

                    }
                }
            }
        ))
    }
}
