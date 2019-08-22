#!/bin/bash

HERE=$(cd $(dirname $0); pwd)

cd $HERE
cargo clean
RUSTFLAGS="-C link-dead-code" cargo test --no-run
cargo kcov --no-clean-rebuild --verbose --all --no-fail-fast
