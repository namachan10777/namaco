#!/bin/bash

HERE=$(cd $(dirname $0); pwd)

cd $HERE
cargo test
