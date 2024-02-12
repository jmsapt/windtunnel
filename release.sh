#!/bin/sh

# Very slow build process, only use for actual release
cargo build --target x86_64-pc-windows-gnu --release &&
cp target/x86_64-pc-windows-gnu/release/thesis.exe . &&
exec ./thesis.exe "$@"