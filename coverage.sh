#!/bin/bash

HERE=$(cd $(dirname $0); pwd)

cd $HERE
cargo clean
RUSTFLAGS="-C link-dead-code" cargo test --no-run
REPORT=$(find target/debug -maxdepth 1 -name 'namaco-*' -a ! -name '*.d')

for file in $REPORT; do
	echo $file
    mkdir -p "target/cov/$(basename $file)"
    kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"
done

wget -O - -q "https://codecov.io/bash" > .codecov
chmod +x .codecov
./.codecov -t $CODECOV_TOKEN
echo "Uploaded code coverage"
