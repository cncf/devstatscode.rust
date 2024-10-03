#!/bin/bash
cd devstats || exit 1
rustfmt src/*.rs || exit 2
rustfmt src/bin/*.rs || exit 3
cargo run --bin runq -- file.sql arg1 arg2
