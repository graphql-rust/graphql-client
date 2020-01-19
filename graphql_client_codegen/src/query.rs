use crate::deprecation::DeprecationStrategy;
use crate::fragments::GqlFragment;
use crate::normalization::Normalization;
use crate::schema::Schema;
use crate::selection::Selection;
use failure::*;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};
use syn::Ident;

/// This holds all the information we need during the code generation phase.
pub(crate) struct QueryContext<'query> {
    pub fragments: BTreeMap<&'query str, GqlFragment<'query>>,
    // pub schema: &'schema Schema,
    pub deprecation_strategy: DeprecationStrategy,
    pub normalization: Normalization,
    variables_derives: Vec<Ident>,
    response_derives: Vec<Ident>,
}

impl<'query, 'schema> QueryContext<'query> {
    /// Create a QueryContext with the given Schema.
    pub(crate) fn new(
        schema: &'schema Schema,
        deprecation_strategy: DeprecationStrategy,
        normalization: Normalization,
    ) -> QueryContext<'query> {
        QueryContext {
            fragments: BTreeMap::new(),
            // schema,
            deprecation_strategy,
            normalization,
            variables_derives: vec![Ident::new("Serialize", Span::call_site())],
            response_derives: vec![Ident::new("Deserialize", Span::call_site())],
        }
    }

    /// Mark a fragment as required, so code is actually generated for it.
    pub(crate) fn require_fragment(&self, typename_: &str) {
        if let Some(fragment) = self.fragments.get(typename_) {
            fragment.require(&self);
        }
    }

    /// For testing only. creates an empty QueryContext.
    #[cfg(test)]
    pub(crate) fn new_empty() -> QueryContext<'query> {
        QueryContext {
            fragments: BTreeMap::new(),
            // schema,
            deprecation_strategy: DeprecationStrategy::Allow,
            normalization: Normalization::None,
            variables_derives: vec![Ident::new("Serialize", Span::call_site())],
            response_derives: vec![Ident::new("Deserialize", Span::call_site())],
        }
    }

    /// Expand the deserialization data structures for the given field.
    pub(crate) fn maybe_expand_field(
        &self,
        ty: &str,
        selection: &Selection<'_>,
        prefix: &str,
    ) -> Result<Option<TokenStream>, failure::Error> {
        unimplemented!()
        // if self.schema.contains_scalar(ty) {
        //     Ok(None)
        // } else if let Some(enm) = self.schema.enums.get(ty) {
        //     enm.is_required.set(true);
        //     Ok(None) // we already expand enums separately
        // } else if let Some(obj) = self.schema.objects.get(ty) {
        //     obj.is_required.set(true);
        //     obj.response_for_selection(self, &selection, prefix)
        //         .map(Some)
        // } else if let Some(iface) = self.schema.interfaces.get(ty) {
        //     iface.is_required.set(true);
        //     iface
        //         .response_for_selection(self, &selection, prefix)
        //         .map(Some)
        // } else if let Some(unn) = self.schema.unions.get(ty) {
        //     unn.is_required.set(true);
        //     unn.response_for_selection(self, &selection, prefix)
        //         .map(Some)
        // } else {
        //     Err(format_err!("Unknown type: {}", ty))
        // }
    }

    pub(crate) fn ingest_response_derives(
        &mut self,
        attribute_value: &str,
    ) -> Result<(), failure::Error> {
        if self.response_derives.len() > 1 {
            return Err(format_err!(
                "ingest_response_derives should only be called once"
            ));
        }

        self.response_derives.extend(
            attribute_value
                .split(',')
                .map(str::trim)
                .map(|s| Ident::new(s, Span::call_site())),
        );
        Ok(())
    }

    pub(crate) fn ingest_variables_derives(
        &mut self,
        attribute_value: &str,
    ) -> Result<(), failure::Error> {
        if self.variables_derives.len() > 1 {
            return Err(format_err!(
                "ingest_variables_derives should only be called once"
            ));
        }

        self.variables_derives.extend(
            attribute_value
                .split(',')
                .map(str::trim)
                .map(|s| Ident::new(s, Span::call_site())),
        );
        Ok(())
    }

    pub(crate) fn variables_derives(&self) -> TokenStream {
        let derives: BTreeSet<&Ident> = self.variables_derives.iter().collect();
        let derives = derives.iter();

        quote! {
            #[derive( #(#derives),* )]
        }
    }

    pub(crate) fn response_derives(&self) -> TokenStream {
        let derives: BTreeSet<&Ident> = self.response_derives.iter().collect();
        let derives = derives.iter();
        quote! {
            #[derive( #(#derives),* )]
        }
    }

    pub(crate) fn response_enum_derives(&self) -> TokenStream {
        let always_derives = [
            Ident::new("Eq", Span::call_site()),
            Ident::new("PartialEq", Span::call_site()),
        ];
        let mut enum_derives: BTreeSet<_> = self
            .response_derives
            .iter()
            .filter(|derive| {
                // Do not apply the "Default" derive to enums.
                let derive = derive.to_string();
                derive != "Serialize" && derive != "Deserialize" && derive != "Default"
            })
            .collect();
        enum_derives.extend(always_derives.iter());
        quote! {
            #[derive( #(#enum_derives),* )]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_derives_ingestion_works() {
        let schema = crate::schema::Schema::new();
        let mut context = QueryContext::new_empty(&schema);

        context
            .ingest_response_derives("PartialEq, PartialOrd, Serialize")
            .unwrap();

        assert_eq!(
            context.response_derives().to_string(),
            "# [ derive ( Deserialize , PartialEq , PartialOrd , Serialize ) ]"
        );
    }

    #[test]
    fn response_enum_derives_does_not_produce_empty_list() {
        let schema = crate::schema::Schema::new();
        let context = QueryContext::new_empty(&schema);
        assert_eq!(
            context.response_enum_derives().to_string(),
            "# [ derive ( Eq , PartialEq ) ]"
        );
    }

    #[test]
    fn response_enum_derives_works() {
        let schema = crate::schema::Schema::new();
        let mut context = QueryContext::new_empty(&schema);

        context
            .ingest_response_derives("PartialEq, PartialOrd, Serialize")
            .unwrap();

        assert_eq!(
            context.response_enum_derives().to_string(),
            "# [ derive ( Eq , PartialEq , PartialOrd ) ]"
        );
    }

    #[test]
    fn response_derives_fails_when_called_twice() {
        let schema = crate::schema::Schema::new();
        let mut context = QueryContext::new_empty(&schema);

        assert!(context
            .ingest_response_derives("PartialEq, PartialOrd")
            .is_ok());
        assert!(context.ingest_response_derives("Serialize").is_err());
    }
}
