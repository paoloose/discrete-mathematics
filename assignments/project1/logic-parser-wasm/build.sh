#!/bin/bash

set -xe

if ! which wasm-pack > /dev/null; then
    echo "wasm-pack need to be installed"
    exit 1
fi

wasm-pack build --target bundler -d ./www/pkg $@
wasm-pack pack ./www/pkg
