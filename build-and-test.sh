#!/bin/sh

cargo build && (cp ./target/debug/opengl32.dll ./test/Mosaik/; pushd test/Mosaik/; ./Mosaik; popd)
