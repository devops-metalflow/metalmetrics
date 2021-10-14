#!/bin/bash

# Install for Windows target
# rustc --print target-list
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
sudo apt update
sudo apt install mingw-w64

# Install for Linux target
# rustc --print target-list
rustup target add x86_64-unknown-linux-gnu
rustup toolchain install stable-x86_64-unknown-linux-gnu
