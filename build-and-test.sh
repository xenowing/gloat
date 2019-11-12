#!/bin/sh

cargo build --release && (cp ./target/release/opengl32.dll ./test/hjb_ch5s/; pushd test/hjb_ch5s/; ./ch5seq; popd)
