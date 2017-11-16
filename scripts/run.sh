#!/usr/bin/env bash

rm out/*.png 2> /dev/null
RUST_BACKTRACE=full cargo run --release -- "$@" && open out/*.png
