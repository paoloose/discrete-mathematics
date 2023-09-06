set -xe

if ! which wasm-pack > /dev/null; then
    cargo install wasm-pack
fi

if ! node --version | grep -q "v18"; then
    fnm install 18
    fnm use 18
fi

# wasm-pack build --target web     # for using directly on the web
# wasm-pack build --target bundler # for publishing to npm
# cargo build --lib --release --target wasm32-unknown-unknown
