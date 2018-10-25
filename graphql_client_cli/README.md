# GraphQL client CLI

This is still a WIP, the main use for it now is to download the `schema.json` from a GraphQL endpoint, which you can also do with [apollo-codegen](https://github.com/apollographql/apollo-cli).

## Install

```
cargo install graphql-client-cli
```

## introsect schema

```
USAGE:
    graphql-client introspect-schema [OPTIONS] <schema_location>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --authorization <authorization>
        --output <output>                  Where to write the JSON for the introspected schema.

ARGS:
    <schema_location>    The URL of a GraphQL endpoint to introspect.
```

## generate client code

```
USAGE:
    graphql-client generate [FLAGS] [OPTIONS] <query_path> <schema_path> <module_name> <output>

FLAGS:
    -h, --help             Prints help information
        --no-formatting    If you don't want to execute rustfmt to generated code, set this option. Default value is
                           false. Formating feature is disabled as default installation.
    -V, --version          Prints version information

OPTIONS:
    -a, --additional-derives <additional_derives>
            Additional derives that will be added to the generated structs and enums for the response and the variables.
            --additional-derives='Serialize,PartialEq'
    -d, --deprecation-strategy <deprecation_strategy>
            You can choose deprecation strategy from allow, deny, or warn. Default value is warn.

    -o, --selected-operation <selected_operation>
            Name of target query. If you don't set this parameter, cli generate all queries in query file.


ARGS:
    <query_path>     Path to graphql query file.
    <schema_path>    Path to graphql schema file.
    <module_name>    Name of module.
    <output>         Path you want to output to.
```

If you want to use formatting feature, you should install like this.

`cargo install graphql-client-cli --features rustfmt`
