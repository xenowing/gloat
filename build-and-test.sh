#!/bin/sh

cargo build --release && (cp ./target/release/opengl32.dll ./test/Mosaik/; pushd test/Mosaik/; ./Mosaik; popd)
