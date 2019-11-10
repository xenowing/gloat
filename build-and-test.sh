#!/bin/sh

cargo build --release && (cp ./target/release/opengl32.dll ./test/moments/; pushd test/moments/; ./Moments; popd)
