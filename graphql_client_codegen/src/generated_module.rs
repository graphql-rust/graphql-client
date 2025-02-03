use crate::{
    codegen_options::*,
    query::{BoundQuery, OperationId},
    BoxError,
};
use heck::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[derive(Debug, thiserror::Error)]
#[error("Could not find an operation named {operation_name} in the query document.")]
struct OperationNotFound {
    operation_name: String,
}

/// This struct contains the parameters necessary to generate code for a given operation.
pub(crate) struct GeneratedModule<'a> {
    pub operation: &'a str,
    pub query_string: &'a str,
    pub resolved_query: &'a crate::query::Query,
    pub schema: &'a crate::schema::Schema,
    pub options: &'a crate::GraphQLClientCodegenOptions,
}

impl GeneratedModule<'_> {
    /// Generate the items for the variables and the response that will go inside the module.
    fn build_impls(&self) -> Result<TokenStream, BoxError> {
        Ok(crate::codegen::response_for_query(
            self.root()?,
            self.options,
            BoundQuery {
                query: self.resolved_query,
                schema: self.schema,
            },
        )?)
    }

    fn root(&self) -> Result<OperationId, OperationNotFound> {
        let op_name = self.options.normalization().operation(self.operation);
        self.resolved_query
            .select_operation(&op_name, *self.options.normalization())
            .map(|op| op.0)
            .ok_or_else(|| OperationNotFound {
                operation_name: op_name.into(),
            })
    }

    /// Generate the module and all the code inside.
    pub(crate) fn to_token_stream(&self) -> Result<TokenStream, BoxError> {
        let module_name = Ident::new(&self.operation.to_snake_case(), Span::call_site());
        let module_visibility = &self.options.module_visibility();
        let operation_name = self.operation;
        let operation_name_ident = self.options.normalization().operation(self.operation);
        let operation_name_ident = Ident::new(&operation_name_ident, Span::call_site());

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
            .unwrap_or_default();

        let query_string = &self.query_string;
        let impls = self.build_impls()?;

        let struct_declaration: Option<_> = match self.options.mode {
            CodegenMode::Cli => Some(quote!(#module_visibility struct #operation_name_ident;)),
            // The struct is already present in derive mode.
            CodegenMode::Derive => None,
        };

        Ok(quote!(
            #struct_declaration

            #module_visibility mod #module_name {
                #![allow(dead_code)]

                use std::result::Result;

                pub const OPERATION_NAME: &str = #operation_name;
                pub const QUERY: &str = #query_string;

                #query_include

                #impls
            }

            impl graphql_client::GraphQLQuery for #operation_name_ident {
                type Variables = #module_name::Variables;
                type ResponseData = #module_name::ResponseData;

                fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
                    graphql_client::QueryBody {
                        variables,
                        query: #module_name::QUERY,
                        operation_name: #module_name::OPERATION_NAME,
                    }

                }
            }
        ))
    }
}
