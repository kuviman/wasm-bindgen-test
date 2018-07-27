#!/bin/env bash

mkdir -p tmp
mkdir -p out
cp $1 tmp/test.wasm
wasm-bindgen tmp/test.wasm --nodejs --out-dir out
echo "module.exports.wasm = wasm;" >> out/test.js
node runner.js
