#!/bin/sh

# This script builds project to Wasm and puts all created binary Wasm module into the artifacts dir
marine build --release

rm artifacts/* || true
mkdir -p artifacts

cp ./target/wasm32-wasi/release/greeting_marine_demo.wasm artifacts/
