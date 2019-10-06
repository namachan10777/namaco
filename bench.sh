#!/usr/bin/bash
cd $(dirname $0)

cargo build --release

cat ./naist-jdic/naist-jdic.csv | shuf -n 10000 | nkf > ./short.csv

time ./target/release/namaco