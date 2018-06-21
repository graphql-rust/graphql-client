use failure;
use fragments::GqlFragment;
use graphql_parser;
use proc_macro2::{Ident, Span, TokenStream};
use schema::Schema;
use selection::Selection;
use std::collections::BTreeMap;
use variables::Variable;

pub struct QueryContext {
    pub _subscription_root: Option<Vec<TokenStream>>,
    pub fragments: BTreeMap<String, GqlFragment>,
    pub mutation_root: Option<Vec<TokenStream>>,
    pub query_root: Option<Vec<TokenStream>>,
    pub schema: Schema,
    pub variables: Vec<Variable>,
}

impl QueryContext {
    pub fn new(schema: Schema) -> QueryContext {
        QueryContext {
            _subscription_root: None,
            fragments: BTreeMap::new(),
            mutation_root: None,
            query_root: None,
            schema,
            variables: Vec::new(),
        }
    }

    pub fn register_variables(&mut self, variables: &[graphql_parser::query::VariableDefinition]) {
        variables.iter().for_each(|variable| {
            self.variables.push(variable.clone().into());
        });
    }

    pub fn expand_variables(&self) -> TokenStream {
        if self.variables.is_empty() {
            return quote!(#[derive(Serialize)]
            pub struct Variables;);
        }

        let fields = self.variables.iter().map(|variable| {
            let name = &variable.name;
            let ty = variable.ty.to_rust(self, "");
            let name = Ident::new(name, Span::call_site());
            quote!(pub #name: #ty)
        });

        let default_constructors = self
            .variables
            .iter()
            .map(|variable| variable.generate_default_value_constructor(self));

        quote! {
            #[derive(Serialize)]
            pub struct Variables {
                #(#fields,)*
            }

            impl Variables {
                #(#default_constructors)*
            }
        }
    }

    /// For testing only. creates an empty QueryContext with an empty Schema.
    #[cfg(test)]
    pub fn new_empty() -> QueryContext {
        QueryContext {
            _subscription_root: None,
            fragments: BTreeMap::new(),
            mutation_root: None,
            query_root: None,
            schema: Schema::new(),
            variables: Vec::new(),
        }
    }

    pub fn maybe_expand_field(
        &self,
        ty: &str,
        selection: &Selection,
        prefix: &str,
    ) -> Result<TokenStream, failure::Error> {
        if let Some(_enm) = self.schema.enums.get(ty) {
            Ok(quote!()) // we already expand enums separately
        } else if let Some(obj) = self.schema.objects.get(ty) {
            obj.response_for_selection(self, &selection, prefix)
        } else if let Some(iface) = self.schema.interfaces.get(ty) {
            Ok(iface.response_for_selection(self, &selection, prefix))
        } else if let Some(unn) = self.schema.unions.get(ty) {
            unn.response_for_selection(self, &selection, prefix)
        } else {
            Ok(quote!())
        }
    }
}
