#!/bin/bash

cargo build --release
rm -rf exe
mkdir exe
mv target/release/brush exe/brush
cargo clean