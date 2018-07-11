use failure;
use graphql_config::*;
use serde_json;

pub fn read_graphql_config() -> Result<GraphQLConfiguration, failure::Error> {
    let cargo_manifest_dir =
        ::std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env variable is defined");

    let config_path = ::std::path::Path::new(&cargo_manifest_dir).join(".graphqlconfig");

    let config_file = ::std::fs::File::open(&config_path).map_err(|io_err| {
        let err: failure::Error = io_err.into();
        err.context(format!(
            "Could not find .graphqlconfig in: {:?}",
            config_path
        ))
    })?;

    serde_json::from_reader::<::std::fs::File, GraphQLConfiguration>(config_file)
        .map_err(|_| format_err!("Failed to parse .graphqlconfig"))
}
