#! /bin/bash
cargo clean
cargo build --release
cp ./target/release/snip /bin

