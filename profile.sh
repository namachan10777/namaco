#!/usr/bin/bash
cd $(dirname $0)

cargo build --release
valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes ./target/release/namaco
