#!/usr/bin/env bash

rm out/*.png
RUST_BACKTRACE=full cargo +nightly run --release && open out/*.png
