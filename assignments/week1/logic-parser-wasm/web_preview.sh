#!/bin/bash

wasm-pack build --target web -d ./www/pkg && \
    python -m http.server -d ./www
