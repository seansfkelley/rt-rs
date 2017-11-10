#!/usr/bin/env bash

rm out/*.png 2> /dev/null
RUST_BACKTRACE=full cargo +nightly run --release && open out/*.png
