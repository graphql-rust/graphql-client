use failure;
use graphql_config::*;
use std::io::prelude::*;

pub fn parse_graphqlconfig() -> Result<GraphQLConfiguration, failure::Error> {
    let cargo_manifest_dir =
        ::std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env variable is defined");

    let config_path = ::std::path::Path::new(&cargo_manifest_dir).join(".graphqlconfig");

    let config_file = ::std::fs::File::open(&path).map_err(|io_err| {
        let err: failure::Error = io_err.into();
        err.context(format!("Could not find .graphqlconfig in: {:?}", path))
    })?;

    Ok(serde_json::from_reader::<GraphQLConfiguration>(config_file).map_err(|| format_err!("Failed to parse .graphqlconfig")));   
}
