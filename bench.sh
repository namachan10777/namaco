#!/usr/bin/bash
cd $(dirname $0)

cargo build --release

cat ./naist-jdic/naist-jdic.csv | shuf -n 400000 | nkf > ./short.csv

time ./target/release/namaco compile --dict ./short.csv --matrix ./naist-jdic/matrix.def --output dict
