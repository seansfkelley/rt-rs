#!/usr/bin/env bash

RUST_BACKTRACE=1 cargo +nightly run --release && open out.png
