#!/bin/bash
WASM32=1 wasm-pack build --dev
mv ./pkg/neonao_parsers.d.ts ./neonao_parsers.gen.d.ts
WASM32=1 wasm-pack build --release
cp -f ./neonao_parsers.d.ts ./pkg/neonao_parsers.d.ts
wasm-opt -Oz pkg/neonao_parsers_bg.wasm -o pkg/neonao_parsers_bg.wasm
WASM32=1 wasm-pack pack