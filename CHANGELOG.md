# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.1.0] - 2018-07-04

This is the initial public release in which the library is considered usable.

### Added

- Support generating a `Variables` struct for a given query and schema through a custom derive, corresponding to the expected variables.
- Support generating a `ResponseData` struct for a given query and schema through a custom derive, corresponding to the shape of the expected response.
- Various utility traits and structs for working with GraphQL query. This notably does not include code to actually perform the network operations. This may be part of future releases.
- Docs and examples
