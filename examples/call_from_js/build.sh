set -ex

cargo +nightly build --target wasm32-unknown-unknown

wasm-bindgen \
  ../../target/wasm32-unknown-unknown/debug/call_from_js.wasm --out-dir .

npm install
npm run serve
