#!/bin/sh

cargo build --release && (cp ./target/release/opengl32.dll ./test/hjb_liqu/; pushd test/hjb_liqu/; ./liquid; popd)
