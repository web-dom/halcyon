build:
	cargo +nightly build --target wasm32-unknown-unknown
	wasm-bindgen ../../target/wasm32-unknown-unknown/debug/example_simple_hello_world.wasm --out-dir .
setup:
	npm install
serve:
	npm run serve
