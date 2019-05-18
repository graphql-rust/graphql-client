# call from JS example

This is a demo of the library compiled to WebAssembly for use in a browser.

## Build

You will need the Rust toolchain and the
[`wasm-pack`](https://rustwasm.github.io/wasm-pack/) CLI
(`cargo install --force wasm-pack`) for this to work. To build
the project, run the following command in this directory:

```bash
wasm-pack build --target=web
```

The compiled WebAssembly program and the glue JS code will be
located in the `./pkg` directory. To run the app, start an
HTTP server in this directory - it contains an `index.html`
file. For example, if you have Python 3:

```bash
python3 -m http.server 8000
```
