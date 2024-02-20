#!/bin/sh

NAME="Windtunnel"

cargo build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/$NAME.exe . &&
exec ./$NAME.exe "$@"