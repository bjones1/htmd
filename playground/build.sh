#!/usr/bin/env bash
# Build the playground into dist/. Needs the wasm32-unknown-unknown target and
# wasm-bindgen-cli at the version in Cargo.lock; uses wasm-opt if installed.
set -euo pipefail
cd "$(dirname "$0")"

cargo build --release --locked --target wasm32-unknown-unknown \
    -p htmd-playground -p htmd-playground-engine

out=target/wasm32-unknown-unknown/release
rm -rf dist
mkdir -p dist/pkg
wasm-bindgen --target web --no-typescript --out-dir dist/pkg --out-name app "$out/htmd-playground.wasm"
wasm-bindgen --target web --no-typescript --out-dir dist/pkg --out-name engine "$out/htmd_playground_engine.wasm"
if command -v wasm-opt >/dev/null; then
    for wasm in dist/pkg/*.wasm; do
        wasm-opt -Os --enable-bulk-memory --enable-nontrapping-float-to-int \
            --enable-reference-types --enable-sign-ext --enable-multivalue \
            "$wasm" -o "$wasm"
    done
fi
cp static/* dist/
