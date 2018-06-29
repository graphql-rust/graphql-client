set -ex

cargo +nightly build --target wasm32-unknown-unknown --release

wasm-bindgen \
  ../../target/wasm32-unknown-unknown/debug/call_from_js.wasm --out-dir .

npm install
npm run serve
