NOTE: This is a WIP - I plan to write proper documentation and make a formal release soon.

Derive Rust code to safely interact with queries written in the GraphQL query language.

This library does not provide any networking, caching or other client functionality, it is just meant to make it easy to interact with a GraphQL query and the corresponding response in a strongly typed way. Building a client can be as simple as this:

```rust
#[derive(GraphQLQuery)]
#[gql(
    query = "/graphql/queries/my_query.graphql",
    schema = "/graphql/schema.graphql"
)]
struct MyQuery;

fn perform_my_query(variables: &my_query::Variables) -> Result<(), failure::Error> {
    let body = MyQuery::expand(variables);
    let client = reqwest::Client::new();
    let res: HttpResponse<graphql_client::Response<my_query::ResponseData>> = client.post("/graphql", body)?;
    println!("{:#?}", res.body);
}
```

## Features

* Strongly typed query variables
* Strongly typed response

### Planned features

* Strongly typed subscriptions
* Query string minification (e.g. for embedding in a browser wasm app, and for minimizing payload size)
* A command line interface in addition to the custom derive for generating code and downloading schemas

## What is generated?

* A module named after the struct under derive, which contains:
  * A `ResponseData` struct implementing `serde::Deserialize`
  * A `Variables` struct meant to contain the variables expected by the query
* An impl for the `GraphQLQuery` trait for the struct under derive

See the example generated module for a full example.

## Examples

See the examples directory in this project.
