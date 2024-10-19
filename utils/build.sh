#!/usr/bin/env bash

set -xeo pipefail

PROFILE="${1:-release}"
TRIPLET="wasm32-unknown-unknown"
WASM="target/$TRIPLET/${PROFILE}/swc_plugin_minify_graphql.wasm"
OUT_DIR="lib"
OUT="${OUT_DIR}/swc_plugin_minify_graphql.wasm"

profile_arg=()

if [ "$PROFILE" != "debug" ]; then
	profile_arg+=("--${PROFILE}")
fi

cargo build --target "$TRIPLET" "${profile_arg[@]}"

mkdir -p "$OUT_DIR"
if [ "$PROFILE" = "release" ]; then
	pnpm wasm-opt --enable-bulk-memory -O4 "$WASM" -o "$OUT"
else
	cp "$WASM" "$OUT"
fi
