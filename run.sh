#!/usr/bin/env bash

RUST_BACKTRACE=1 RUST_LOG=rt=info cargo run && open out.png
