#!/bin/bash

# Install for Windows target
# rustc --print target-list
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
sudo apt update -y
sudo apt install -y mingw-w64

# Install for Linux target
# rustc --print target-list
rustup target add x86_64-unknown-linux-gnu
rustup toolchain install stable-x86_64-unknown-linux-gnu
