#!/bin/bash -eu
# Copyright 2025 VantisOS Project
# SPDX-License-Identifier: MIT

# OSS-Fuzz build script for VantisOS

# This script builds VantisOS for fuzzing with OSS-Fuzz

# Set build directory
SRC=$SRC/vantisos
cd $SRC

# Install Rust toolchain
rustup default stable
rustup update stable

# Install fuzzing dependencies
cargo install cargo-fuzz

# Build fuzz targets
cd src/verified

# Build IPC fuzzer
cargo fuzz build --fuzz-target ipc_fuzzer

# Build scheduler fuzzer
cargo fuzz build --fuzz-target scheduler_fuzzer

# Build memory manager fuzzer
cargo fuzz build --fuzz-target memory_fuzzer

# Build VantisFS fuzzer
cargo fuzz build --fuzz-target filesystem_fuzzer

# Build Vantis Vault fuzzer
cargo fuzz build --fuzz-target vault_fuzzer

# Copy fuzz binaries to output directory
cp fuzz/target/x86_64-unknown-linux-gnu/release/ipc_fuzzer $OUT/
cp fuzz/target/x86_64-unknown-linux-gnu/release/scheduler_fuzzer $OUT/
cp fuzz/target/x86_64-unknown-linux-gnu/release/memory_fuzzer $OUT/
cp fuzz/target/x86_64-unknown-linux-gnu/release/filesystem_fuzzer $OUT/
cp fuzz/target/x86_64-unknown-linux-gnu/release/vault_fuzzer $OUT/

# Copy corpus dictionaries
mkdir -p $OUT/ipc_fuzzer.dict
mkdir -p $OUT/scheduler_fuzzer.dict
mkdir -p $OUT/memory_fuzzer.dict
mkdir -p $OUT/filesystem_fuzzer.dict
mkdir -p $OUT/vault_fuzzer.dict

# Copy seed corpus if available
if [ -d "fuzz/corpus/ipc" ]; then
    cp -r fuzz/corpus/ipc/* $OUT/ipc_fuzzer.dict/
fi

if [ -d "fuzz/corpus/scheduler" ]; then
    cp -r fuzz/corpus/scheduler/* $OUT/scheduler_fuzzer.dict/
fi

if [ -d "fuzz/corpus/memory" ]; then
    cp -r fuzz/corpus/memory/* $OUT/memory_fuzzer.dict/
fi

if [ -d "fuzz/corpus/filesystem" ]; then
    cp -r fuzz/corpus/filesystem/* $OUT/filesystem_fuzzer.dict/
fi

if [ -d "fuzz/corpus/vault" ]; then
    cp -r fuzz/corpus/vault/* $OUT/vault_fuzzer.dict/
fi

echo "Build complete!"