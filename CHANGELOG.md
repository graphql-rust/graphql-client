# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased

## Added

- The introspection query response shape internally
  used by graphql-client is now its own crate,
  `graphql-introspection-query`.
- A new experimental `normalization` attribute, that renames all
  generated names to camel case. By default, the `"none"` naming conventions
  are used, however, if set to `"rust"`, Rust naming rules will be used
  instead. This may become the default in future versions, since not normalizing
  names can lead to invalid code. (thanks @markcatley!)
- `response_derives` now only applies to responses. Use `variables_derives` for
  variable structure derives.

## Fixed

- (BREAKING) In the CLI, there was a conflict between the short forms
  `--output-directory` and `--selected-operation` since they were both `-o`.
  After this fix, `-o` is the short form of `--output-directory`. (thanks @davidgraeff!)
- Catch more cases where a rust keyword in schemas or queries would break code generation (thanks @davidgraeff!)

## 0.8.0 - 2019-05-24

## Changed

- (BREAKING) This release sees the switch to the Rust 2018 edition - it is only
  compatible with Rust 1.31.0 and later. In particular, this changes how the
  derive macro is imported. See the README for more details.
- `graphql_client_web` is now deprecated. The browser client has been moved to
  the `graphql_client` crate, under the `web` module. **It is only available
  with the `web` feature enabled on the `wasm32-unknown-unknown` target.**

## 0.7.1

### Fixed

- In the CLI, both --selected-operation and --output used the -o shorthand. --output now uses -out.

## 0.7.0 - 2019-03-21

### Changed

- The `selected_operation` derive attribute is deprecated. **The name of the
  struct under derive now has to match one of the operations defined in the query
  file.**
- The CLI now takes the schema path as a required keyword argument.
- The CLI was revamped to generate separate modules for each operation in a given query file.

  The idea now is that if you generate code for the following document:

  ```graphql
  # In my_query.graphql

  query QueryA { .... }
  query QueryB { .... }
  mutation SomeMutation { ... }
    ```

  The CLI generated code will look like this:

  ```rust
  // In my_query.rs

  pub struct QueryA;

  pub mod query_a { ... }

  pub struct QueryB;

  pub mod query_b { ... }

  pub struct SomeMutation;

  pub mod some_mutation { ... }
    ```

  That way the operations don't live in the same module, there is no risk of names clashing anymore.

### Fixed

- Changes to query files will now always trigger code generation for the corresponding modules on the next build.

## 0.6.1 - 2019-01-19

### Added

- If there is no `schema` declaration in a schema, the root types will be matched by name (`Query`, `Mutation` and `Subscription`).

### Changed

- Enums now always derive PartialEq and Eq by default

### Fixed

- Code generation for fragments on unions was fixed
- Support for recursive fragments and input types
- The graphql-parser dependency version is no longer pinned

## 0.6.0 (2018-12-30)

### Added

- The CLI can now optionally format the generated code with rustfmt (enable the `rustfmt` feature).
- When deriving, the generated module now has the same visibility (private, `pub`, `pub(crate)` or `crate`) as the struct under derive.
- Codegen now supports type-refining fragments, i.e. fragments on interfaces or unions that only apply to one of the variants. Example:

  ```graphql
  type Pie {
    diameter: Integer
    name: String
  }

  type Sandwich {
    length: Float
    ingredients: [String]
  }

  union Food = Sandwich | Pie

  type Query {
    lunch: Food
  }

  fragment PieName on Pie {
    name
  }

  query Test {
    lunch {
      ...PieName
      ...on Sandwich {
        length
      }
    }
  }

  ```

### Changed

- (BREAKING) GraphQLQuery does not take a lifetime parameter anymore. This makes it easier to work with futures in async client, since futures expect everything they capture to have the 'static lifetime.
- (BREAKING) Removed the `Rust` prefix on the name of generated items.
- (BREAKING) If you don't set `--selected-operation` options with `graphql-client generate`, the cli generate all queries in query file.

### Fixed

- When using edition 2018, you no longer need to add `#[macro_use] extern crate serde_derive` to your crate for the generated modules to compile (thanks @aergonaut!)

## 0.5.1 (2018-10-07)

### Added

- Better error messages from the derive macro stwhen the schema or the query file path is not found.

## Fixed

- Support both full introspection responses (with "data") field and just the content of the "data" field in schema.json files.

## 0.5.0 (2018-10-03)

### Added

- Support for `@deprecated` field annotations. You can configure how
  deprecations are handled via the `deprecated` argument in the `GraphQLQuery`
  derive:

  ```rust
  #[derive(GraphQLQuery)]
  #[graphql(
      schema_path = "src/graphql/schema.json",
      query_path = "src/graphql/queries/my_query.graphql",
      deprecated = "warn"
  )]
  pub struct MyQuery;
  ```

  Valid values are:

  - `allow`: the response struct fields are not marked as deprecated.
  - `warn`: the response struct fields are marked as `#[deprecated]`.
  - `deny`: The struct fields are not included in the response struct and
    using them is a compile error.

  The default is `warn`.

  This is a *breaking change* if you have the `#[deny(deprecated)]` compiler
  lint and you use deprecated fields in your queries. The quick solution is to
  annotate the relevant queries with `depracated = "allow"` as shown above.

- The CLI now supports the `--authorization` flag to pass the contents of an `Authorization` header. Thanks to @h-michael for the [PR](https://github.com/graphql-rust/graphql-client/pull/92)!

- Improved some codegen error messages, giving more context. Thanks @mathstuf!

- Aliases in queries are now supported. Beyond the ergonomics, this is an important feature since it allows to query the same field on an object multiple times with different arguments, as shown in the [official guide](https://graphql.org/learn/queries/#aliases). Thanks a lot @mathstuf!

- The traits in `response_derives` are now used for input types (variables) as well. In the future we may want to separate the options, but we will first experiment this way.

- Most of the `graphql-query-derive` crate was factored out into a new `graphql-client-codegen` crate that should enable code generation through means other than custom derives (CLI, build scripts...). Thanks @h-michael for this important refactoring!

- Top-level exported types have been renamed. Types no longer have a `GraphQL` prefix. e.g. `GraphQLQuery` -> `Query`, `GraphQLError` -> `Error`.

- For several classes of items that we generate, we only generate those that are actually used by the query. This way, you do not need to define mappings for _every_ scalar in the schema you are querying - even for small queries - anymore. This also improves compile times a lot in some scenarios. (#116 - thanks @mathstuf!)

- `GraphQLError` now implements the `Display` trait.

- Basic support for fragments on interfaces. See #154 for what is not supported yet.

### Fixed

- Handle all Rust keywords as field names in codegen by appending `_` to the generated names, so a field called `type` in a GraphQL query will become a `type_` field in the generated struct. Thanks to @scrogson!

- [Some error message improvements](https://github.com/graphql-rust/graphql-client/pull/100).

- The `operationName` field is now correctly set on request bodies.

## [0.4.0] - 2018-08-23

There are a number of breaking changes due to the new features, read the `Added` section attentively if you are upgrading.

### Added

- (breaking) Control over which types custom scalars deserialize to is given to the user: you now have to provide type aliases for the custom scalars in the scope of the struct under derive.
- (breaking) Support for multi-operations documents. You can select a particular operation by naming the struct under derive after it. In case there is no match, we revert to the current behaviour: select the first operation.
- (breaking) Support arbitrary derives on the generated response types via the `response_derives` option on the `graphql` attribute. If you were relying on the `Debug` impl on generated structs before, you need to add `response_derives = "Debug"` in the `#[graphql()]` attributes in your structs.

### Fixed

- Fixed codegen of fields with leading underscores - they were ignored, leading to wrong derived types for deserialization.
- Made the CLI dump introspected schemas directly without trying to validate them.

## [0.3.0] - 2018-07-24

### Added

- Implemented support for the `extensions` field on errors from the June 2018 spec (#64).
- Improved documentation crate docs, added doctests and examples

### Fixed

- `Location` fields on errors were not public.
- The field names on input objects were not properly converted between snake and camel case.

### Changed

- `serde_json` is now a dependency, because the `extensions` field on errors can be contain arbitrary JSON.

## [0.2.0] - 2018-07-22

### Added

- Copy documentation from the GraphQL schema to the generated types (including their fields) as normal Rust documentation. Documentation will show up in the generated docs as well as IDEs that support expanding derive macros (which does not include the RLS yet).
- Implement and test deserializing subscription responses. We also try to provide helpful error messages when a subscription query is not valid (i.e. when it has more than one top-level field).
- Support the [new top-level errors shape from the June 2018 spec](https://github.com/facebook/graphql/blob/master/spec/Section%207%20--%20Response.md), except for the `extensions` field (see issue #64).

### Fixed

- The generated `ResponseData` structs did not convert between snake and camel case.

## [0.1.0] - 2018-07-04

This is the initial public release in which the library is considered usable.

### Added

- Support generating a `Variables` struct for a given query and schema through a custom derive, corresponding to the expected variables.
- Support generating a `ResponseData` struct for a given query and schema through a custom derive, corresponding to the shape of the expected response.
- Various utility traits and structs for working with GraphQL query. This notably does not include code to actually perform the network operations. This may be part of future releases.
- Docs and examples
