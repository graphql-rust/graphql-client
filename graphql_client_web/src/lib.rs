#![deprecated(
    note = "graphql_client_web is deprecated. The web client is now part of the graphql_client crate, with the default \"client\" feature."
)]

pub use graphql_client::client::*;
pub use graphql_client::{self, *};
