#!/bin/sh

cd `dirname $0`
cargo +nightly build -Zbuild-std=core,alloc --target ./x86_64-unknown-none-mikankernel.json --release --verbose
