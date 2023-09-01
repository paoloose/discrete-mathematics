set -xe

fnm install 18
cargo install wasm-pack
# wasm-pack build --target web     # for using directly on the web
# wasm-pack build --target bundler # for publishing
# cargo build --lib --release --target wasm32-unknown-unknown
