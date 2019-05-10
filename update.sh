#!/usr/bin/env bash
set -x
set -e

svd=LPC55S69_cm33_core0.xml

# NOTE: Last executed using Rust 1.34.0

# cargo install --force --version 0.14.0 svd2rust
# cargo install --force --version 0.6.0  form
# rustup component add rustfmt

rm -rf src
mkdir src
svd2rust -i ./${svd}
form -i lib.rs -o src
rm lib.rs
cargo fmt
rustfmt build.rs
