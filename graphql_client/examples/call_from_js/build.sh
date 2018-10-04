set -ex

export CARGO_INCREMENTAL=0

cargo +nightly build --target wasm32-unknown-unknown --release

wasm-bindgen \
  ./target/wasm32-unknown-unknown/release/call_from_js.wasm --out-dir .

npm install
npm run serve
