#!/bin/bash

build=$(date +%FT%T%z)

linux="target/x86_64-unknown-linux-gnu/release/metalmetrics-rs"
windows="target/x86_64-pc-windows-gnu/release/metalmetrics-rs.exe"

if [ "$1" = "all" ]; then
  build=$build cargo build --release --all-features --all-targets --target x86_64-pc-windows-gnu
  build=$build cargo build --release --all-features --all-targets --target x86_64-unknown-linux-gnu
elif [ "$1" = "check" ]; then
  build=$build cargo check --release --all-features --all-targets
else
  build=$build cargo build --release --all-features --all-targets
fi

if [ -f $linux ]; then upx $linux; fi
if [ -f $windows ]; then upx $windows; fi
