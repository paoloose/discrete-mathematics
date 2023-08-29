#!/bin/bash

if [ -n $1 ]; then
    wasm-pack publish --tag $1
else
    wasm-pack publish
fi
