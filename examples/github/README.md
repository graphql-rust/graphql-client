# graphql-client GitHub API examples

The schema is taken from [this repo](https://raw.githubusercontent.com/octokit/graphql-schema/master/schema.graphql).

## How to run it

The example expects to find a valid GitHub API Token in the environment (`GITHUB_API_TOKEN`). See the [GitHub documentation](https://developer.github.com/v4/guides/forming-calls/#authenticating-with-graphql) on how to generate one.

Then just run the example with a repository name as argument. For example:

```bash
cargo run -- graphql-rust/graphql-client
```
