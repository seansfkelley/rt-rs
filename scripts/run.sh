#!/usr/bin/env bash

RUST_BACKTRACE=full cargo +nightly run --release && open out.png
