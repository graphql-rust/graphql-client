#![deprecated(
    note = "graphql_client_web is deprecated. The web client is now part of the graphql_client crate, with the \"web\" feature."
)]

pub use graphql_client::web::*;
pub use graphql_client::{self, *};
