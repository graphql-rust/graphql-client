//! The rendering path. Goes from a fully resolved query to Rust code.

use crate::resolution::ResolvedQuery;
use quote::quote;

pub(crate) fn render(
    schema: &crate::schema::Schema,
    query: &ResolvedQuery,
) -> anyhow::Result<proc_macro2::TokenStream> {
    Ok(quote!())
}
