#!/bin/bash

www_dir="./www"
serve_command="python3 -m http.server -d $www_dir"

if which live-server > /dev/null 2>&1; then
    serve_command="live-server $www_dir"
fi

wasm-pack build --target web -d $www_dir/pkg && $serve_command
