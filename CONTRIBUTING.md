
# Contributing

All contributors are expected to follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## Pull requests

Before opening large pull requests, it is prefered that the change be discussed in a github issue first. This helps keep everyone on the same page, and facilitates a smoother code review process.

## Testing

The CI system conducts a few different tests for various releases of rust. In addition to the normal cargo tests, code formatting is checked with [fmt](https://github.com/rust-lang-nursery/rustfmt), and linting is checked with [clippy](https://github.com/rust-lang-nursery/rust-clippy). Whereas cargo tests are run for all rust release channels, `fmt` and `clippy` are only run on the stable channel.

| Channel | fmt | clippy | test |
|---------|-----|--------|------|
| stable  | x   | x      | x    |
| beta    |     |        | x    |
| nightly |     |        | x    |

To avoid any surprises by CI while merging, it's recommended you run these locally after making changes. Setup and testing only takes a couple minutes at most.

### Setup

Rust does not have `fmt` or `clippy` installed by default, so you will have to add them manually. The installation process is unlikely to change, but if it does, specific installation instructions can be found on the READMEs for [fmt](https://github.com/rust-lang-nursery/rustfmt#quick-start) and [clippy](https://github.com/rust-lang-nursery/rust-clippy#step-2-install-clippy).

```
rustup component add rustfmt-preview clippy-preview
```

If you want install to a different toolchain (if for instance your default is set to nightly, but you need to test stable), you can provide the 'toolchain' argument:

```
rustup component add rustfmt-preview clippy-preview --toolchain stable
```

We are using [Prettier](https://prettier.io) to check `.json|.graphql` files. To have it on your local machine you need to install [Node.js](https://nodejs.org) first.
Our build is now using latest LTS version of Node.js. We're using `npm` and global install here:

```bash
npm install --global prettier
```

### Running

Verify you are using the stable channel (output of `rustc --version` does not contain "nightly" or "beta"). Then run fmt, clippy, and test as they are invoked in the `.travis.yml` file.

If you are on the stable channel, then you can run fmt, clippy, and test as they are invoked in the `.travis.yml` file.

```
cargo fmt --all -- --check
cargo clippy
cargo test --all
```

If your default channel is something other than stable, you can force the use of stable by providing the channel option:

```
cargo +stable fmt --all -- --check
cargo +stable clippy
cargo +stable test --all
```
