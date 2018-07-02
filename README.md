# graphql_client

![CI Status](https://img.shields.io/travis/tomhoule/graphql-client.svg)
![docs](https://docs.rs/mio/badge.svg)
![crates.io](https://img.shields.io/crates/d/graphql-client.svg)
![license](https://img.shields.io/github/license/mashape/apistatus.svg)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)

Derive Rust code to safely and ergonomically manipulate GraphQL queries.

This library does not provide any networking, caching or other client functionality yet, it is just meant to make it easy to interact with a GraphQL query and the corresponding response in a strongly typed way. Making a request can be as simple as this:

```rust
#[derive(GraphQLQuery)]
#[graphql(
    // The paths are relative to the directory where your `Cargo.toml` is located.
    query_path = "src/graphql/queries/my_query.graphql",
    schema_path = "src/graphql/schema.json"
)]
struct MyQuery;
```

```rust
fn perform_my_query(variables: &my_query::Variables) -> Result<(), failure::Error> {
    let body = MyQuery::expand(variables);
    let client = reqwest::Client::new();
    let mut res: HttpResponse<graphql_client::Response<my_query::ResponseData>> = client.post("/graphql").json(&body).send()?;
    println!("{:#?}", res.text());
    Ok(())
}
```

The GraphQL schema language (`.graphql`) and `schema.json` are both supported as sources for the schema.

`serde_derive` needs to be visible in the context of the `GraphQLQuery` derive (add it as an `extern crate`).

## Features

- Strongly typed query variables
- Strongly typed responses
- Works in the browser (WebAssembly)

Integration with different HTTP libraries is planned, although building one yourself is trivial (just send the constructed request payload as JSON with a POST request to a GraphQL endpoint, modulo authentication).

There is an embryonic CLI for downloading schemas - the plan is to make it something similar to `apollo-codegen`.


## What is generated?

- A module named after the struct under derive, which contains:
  - A `ResponseData` struct implementing `serde::Deserialize`
  - A `Variables` struct meant to contain the variables expected by the query
- An impl for the `GraphQLQuery` trait for the struct under derive

See the [example generated module](https://www.tomhoule.com/docs/example_module/) for more details.

## Examples

See the examples directory in this project.

## Code of conduct

Anyone who interacts with this project in any space, including but not limited to
this GitHub repository, must follow our [code of conduct](https://github.com/tomhoule/graphql-client/blob/master/CODE_OF_CONDUCT.md).

## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.
