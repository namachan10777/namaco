#!/usr/bin/bash
cd $(dirname $0)

cargo build --release
valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes ./target/release/bot
for f in callgrind.out.*;do
	mv $f ./callgrind/callgrind.out
done
