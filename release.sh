#!/bin/sh

NAME="Windtunnel"

# Very slow build process, only use for actual release
cargo build --target x86_64-pc-windows-gnu --release &&
cp target/x86_64-pc-windows-gnu/release/$NAME.exe . &&
exec ./$NAME.exe "$@"