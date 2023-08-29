#!/bin/bash

verbosity=""

which wasm-pack > /dev/null

if [ $? != "0"]; then
    echo "wasm-pack need to be installed"
    exit 1
fi

for arg in $@; do
    case $arg in
        -v) verbosity="-v" ;;
    esac
done

echo $verbose

wasm-pack $verbosity build --target web -d ./www/pkg
wasm-pack $verbosity pack
