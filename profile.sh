#!/usr/bin/bash
cd $(dirname $0)

rm callgrind.out.*
cargo build

cat ./naist-jdic/naist-jdic.csv | shuf -n 100 | nkf > ./short.csv

valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes ./target/debug/namaco
mv callgrind.out.* callgrind.out
