#!/bin/bash

cargo build
rm -rf exe
mkdir exe
mv target/debug/brush exe/brush_DEBUG
cargo clean