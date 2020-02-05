# graphql_client

[![Github actions Status](https://github.com/graphql-rust/graphql-client/workflows/CI/badge.svg?branch=master&event=push)](https://github.com/graphql-rust/graphql-client/actions)
[![Build Status](https://travis-ci.org/graphql-rust/graphql-client.svg?branch=master)](https://travis-ci.org/graphql-rust/graphql-client)
[![docs](https://docs.rs/graphql_client/badge.svg)](https://docs.rs/graphql_client/latest/graphql_client/)
[![crates.io](https://img.shields.io/crates/v/graphql_client.svg)](https://crates.io/crates/graphql_client)
[![Join the chat](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/juniper-graphql/graphql-client)

A typed GraphQL client library for Rust.

## Features

- Precise types for query variables and responses.
- Supports GraphQL fragments, objects, unions, inputs, enums, custom scalars and input objects.
- Works in the browser (WebAssembly).
- Subscriptions support (serialization-deserialization only at the moment).
- Copies documentation from the GraphQL schema to the generated Rust code.
- Arbitrary derives on the generated responses.
- Arbitrary custom scalars.
- Supports multiple operations per query document.
- Supports setting GraphQL fields as deprecated and having the Rust compiler check
  their use.
- [web client](./graphql_client_web) for boilerplate-free API calls from browsers.

## Getting started

- If you are not familiar with GraphQL, the [official website](https://graphql.org/) provides a very good and comprehensive introduction.

- Once you have written your query (most likely in something like [graphiql](https://github.com/graphql/graphiql)), save it in a `.graphql` file in your project.

- In order to provide precise types for a response, graphql_client needs to read the query and the schema at compile-time.

  To download the schema, you have multiple options. This projects provides a [CLI](https://github.com/graphql-rust/graphql-client/tree/master/graphql_client_cli), however it does not matter what tool you use, the resulting `schema.json` is the same.

- We now have everything we need to derive Rust types for our query. This is achieved through a procedural macro, as in the following snippet:

  ```rust
  use graphql_client::GraphQLQuery;

  // The paths are relative to the directory where your `Cargo.toml` is located.
  // Both json and the GraphQL schema language are supported as sources for the schema
  #[derive(GraphQLQuery)]
  #[graphql(
      schema_path = "tests/unions/union_schema.graphql",
      query_path = "tests/unions/union_query.graphql",
  )]
  pub struct UnionQuery;
  ```

  The `derive` will generate a module named `union_query` in this example - the name is the struct's name, but in snake case.

  That module contains all the struct and enum definitions necessary to deserialize a response to that query.

  The root type for the response is named `ResponseData`. The GraphQL response will take the form of a `Response<ResponseData>` (the [Response](https://docs.rs/graphql_client/latest/graphql_client/struct.Response.html) type is always the same).

  The module also contains a struct called `Variables` representing the variables expected by the query.

* We now need to create the complete payload that we are going to send to the server. For convenience, the [GraphQLQuery trait](https://docs.rs/graphql_client/latest/graphql_client/trait.GraphQLQuery.html), is implemented for the struct under derive, so a complete query body can be created this way:

  ```rust
  use graphql_client::{GraphQLQuery, Response};

  #[derive(GraphQLQuery)]
  #[graphql(
      schema_path = "tests/unions/union_schema.graphql",
      query_path = "tests/unions/union_query.graphql",
      response_derives = "Debug",
  )]
  pub struct UnionQuery;

  fn perform_my_query(variables: union_query::Variables) -> Result<(), anyhow::Error> {

      // this is the important line
      let request_body = UnionQuery::build_query(variables);

      let client = reqwest::Client::new();
      let mut res = client.post("/graphql").json(&request_body).send()?;
      let response_body: Response<union_query::ResponseData> = res.json()?;
      println!("{:#?}", response_body);
      Ok(())
  }
  ```

[A complete example using the GitHub GraphQL API is available](https://github.com/graphql-rust/graphql-client/tree/master/examples/github), as well as sample [rustdoc output](https://www.tomhoule.com/docs/example_module/).

## Deriving specific traits on the response

The generated response types always derive `serde::Deserialize` but you may want to print them (`Debug`), compare them (`PartialEq`) or derive any other trait on it. You can achieve this with the `response_derives` option of the `graphql` attribute. Example:

```rust
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/unions/union_schema.graphql",
    query_path = "tests/unions/union_query.graphql",
    response_derives = "Serialize,PartialEq",
)]
struct UnionQuery;
```

## Custom scalars

The generated code will reference the scalar types as defined in the server schema. This means you have to provide matching rust types in the scope of the struct under derive. It can be as simple as declarations like `type Email = String;`. This gives you complete freedom on how to treat custom scalars, as long as they can be deserialized.

## Deprecations

The generated code has support for [`@deprecated`](http://facebook.github.io/graphql/June2018/#sec-Field-Deprecation)
field annotations. You can configure how deprecations are handled via the `deprecated` argument in the `GraphQLQuery` derive:

```rust
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "tests/unions/union_schema.graphql",
  query_path = "tests/unions/union_query.graphql",
  deprecated = "warn"
)]
pub struct UnionQuery;
```

Valid values are:

- `allow`: the response struct fields are not marked as deprecated.
- `warn`: the response struct fields are marked as `#[deprecated]`.
- `deny`: The struct fields are not included in the response struct and
  using them is a compile error.

The default is `warn`.

## Query documents with multiple operations

You can write multiple operations in one query document (one `.graphql` file). You can then select one by naming the struct you `#[derive(GraphQLQuery)]` on with the same name as one of the operations. This is neat, as it allows sharing fragments between operations.

Note that the struct and the operation in the GraphQL file *must* have the same name. We enforce this to make the generated code more predictable.

```rust
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "tests/unions/union_schema.graphql",
    query_path = "tests/unions/union_query.graphql",
)]
pub struct UnionQuery;
```

There is an example [in the tests](./graphql_client/tests/operation_selection).

## Documentation for the generated modules

You can use `cargo doc --document-private-items` to generate rustdoc documentation on the generated code.

## Make cargo recompile when .graphql files have changed

There is an [`include`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields-optional) option you can add to your `Cargo.toml`. It currently has issues however (see [this issue](https://github.com/rust-lang/cargo/issues/6031#issuecomment-422160178)).

## Examples

See the [examples directory](https://github.com/graphql-rust/graphql-client/tree/master/examples) in this repository.

## Contributors

Warmest thanks to all those who contributed in any way (not only code) to this project:

- Alex Vlasov (@indifferentalex)
- Ben Boeckel (@mathstuf)
- Chris Fung (@aergonaut)
- Christian Legnitto (@LegNeato)
- David Gräff (@davidgraeff)
- Dirkjan Ochtman (@djc)
- Fausto Nunez Alberro (@brainlessdeveloper)
- Hirokazu Hata (@h-michael)
- Peter Gundel (@peterfication)
- Sonny Scroggin (@scrogson)
- Sooraj Chandran (@SoorajChandran)
- Tom Houlé (@tomhoule)

## Code of conduct

Anyone who interacts with this project in any space, including but not limited to
this GitHub repository, must follow our [code of conduct](https://github.com/graphql-rust/graphql-client/blob/master/CODE_OF_CONDUCT.md).

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
