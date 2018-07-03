# graphql_client

[![Build Status](https://travis-ci.org/tomhoule/graphql-client.svg?branch=master)](https://travis-ci.org/tomhoule/graphql-client)
[![docs](https://docs.rs/graphql_client/badge.svg)](https://docs.rs/graphql_client/0.0.1/graphql_client/)
[![crates.io](https://img.shields.io/crates/v/graphql_client.svg)](https://crates.io/crates/graphql_client)

A typed, ergonomic GraphQL client library for Rust.

## Getting started

In order to provide precise types for a response, graphql_client needs to read the query and the schema at compile-time.

This is achieved through a procedural macro, as in the following snippet:

```rust
// The paths are relative to the directory where your `Cargo.toml` is located.
// Both json and the GraphQL schema language are supported as sources for the schema
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/my_query.graphql",
)]
pub struct MyQuery;
```

The `derive` will generate a module named `my_query` in this example - the name is the struct's name, but in snake case.

That module contains all the struct and enum definitions necessary to deserialize a response to that query.

The root type for the response is named `ResponseData`. The GraphQL response will take  the form of a `GraphQLResponse<ResponseData>` (the [GraphQLResponse](https://docs.rs/graphql_client/latest/graphql_client/struct.GraphQLResponse.html) type is always the same).

The module also contains a struct called `Variables` representing the variables expected by the query.

[A full example is available](https://github.com/tomhoule/graphql-client/tree/master/examples/example_module), including [rustdoc output](https://www.tomhoule.com/docs/example_module/).

NOTE: `serde` and `serde_derive` need to be imported in the current crate with `extern crate`.

For convenience, the [GraphQLQuery trait](https://docs.rs/graphql_client/latest/graphql_client/trait.GraphQLQuery.html), is implemented for the struct under derive, so it can be used this way:

```rust
fn perform_my_query(variables: &my_query::Variables) -> Result<(), failure::Error> {
    let request_body = MyQuery::expand(variables);
    let client = reqwest::Client::new();
    let mut res = client.post("/graphql").json(&request_body).send()?;
    let response_body: GraphQLResponse<my_query::ResponseData> = res.json()?;
    println!("{:#?}", response_body);
    Ok(())
}
```

## Features

- Strongly typed query variables
- Strongly typed responses
- Works in the browser (WebAssembly)
- Supports GraphQL fragments, objects, unions, inputs, enums, custom scalars and input objects

### Roadmap

A lot of desired features have been defined in issues.

graphql_client does not provide any networking, caching or other client functionality yet. Integration with different HTTP libraries is planned, although building one yourself is trivial (just send the constructed request payload as JSON with a POST request to a GraphQL endpoint, modulo authentication).

There is an embryonic CLI for downloading schemas - the plan is to make it something similar to `apollo-codegen`.


## Examples

See the examples directory in this repository.

## Code of conduct

Anyone who interacts with this project in any space, including but not limited to
this GitHub repository, must follow our [code of conduct](https://github.com/tomhoule/graphql-client/blob/master/CODE_OF_CONDUCT.md).

## Contributors

Many thanks go to all our contributors:

|                                            |              |
| ------------------------------------------ | ------------ |
| Alex Vlasov (@indifferentalex)             | 👀           |
| Fausto Nuñez Alberro (@brainlessdeveloper) | 👀           |
| Peter Gundel (@peterfication)              | 👀           |
| Tom Houlé (@tomhoule)                      | 💻📖🐛💡🔧👀 |

This project follows the [all-contributors](https://github.com/kentcdodds/all-contributors) specification.
Contributions of any kind are welcome!

## License

Licensed under either of these:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  https://opensource.org/licenses/MIT)

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.
