#!/bin/bash

set -xe
verbosity=""

which wasm-pack > /dev/null

if [ $? != "0" ]; then
    echo "wasm-pack need to be installed"
    exit 1
fi

echo $verbose

wasm-pack build --target bundler -d ./www/pkg $@
wasm-pack pack
