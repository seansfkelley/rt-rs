#!/usr/bin/env bash

RUST_BACKTRACE=1 RUST_LOG=rt=info cargo run --release && open out.png
