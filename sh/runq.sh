#!/bin/bash
cd devstats && cargo fmt && cargo run --bin runq -- file.sql arg1 arg2
