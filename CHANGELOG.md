# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- (breaking) Control over which types custom scalars deserialize to is given to the user: you now have to provide type aliases for the custom scalars in the scope of the struct under derive.
- (breaking) Support for multi-operations documents. You can select a particular operation by naming the struct under derive after it. In case there is no match, we revert to the current behaviour: select the first operation.
- (breaking) Support arbitrary derives on the generated response types via the `response_derives` option on the `graphql` attribute.

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
