#!/usr/bin/env bash

OUT_DIR="out/$(basename "$@" .scene)"
rm -r "$OUT_DIR" 2> /dev/null
RUST_BACKTRACE=full cargo run --release -- "$@" && open "$OUT_DIR"/*.png
