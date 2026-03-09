#!/bin/bash

set -e

echo "🚀 Setting up VantisOS Development Environment..."

# Update packages
sudo apt-get update -y

# Install system dependencies
sudo apt-get install -y \
    build-essential \
    qemu-system-x86 \
    nasm \
    grub-pc-bin \
    xorriso \
    mtools \
    git \
    curl \
    wget \
    python3 \
    python3-pip \
    pkg-config \
    libssl-dev \
    fuse \
    git-lfs

# Install Rust if not already installed
if ! command -v rustc &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.93.0
    source $HOME/.cargo/env
fi

# Install Verus (formal verification tool)
echo "🔬 Installing Verus..."
cargo install verus --version 0.1.0 || echo "Verus installation failed (optional)"

# Install Z3 theorem prover
echo "🧮 Installing Z3..."
wget -q https://github.com/Z3Prover/z3/releases/download/Z3-4.12.4/z3-4.12.4-x64-ubuntu-22.04.zip -O /tmp/z3.zip
unzip -q /tmp/z3.zip -d /tmp
sudo mv /tmp/z3-4.12.4-x64-ubuntu-22.04 /usr/local/z3
sudo ln -s /usr/local/z3/bin/z3 /usr/local/bin/z3
rm /tmp/z3.zip

# Initialize Git LFS
echo "📋 Initializing Git LFS..."
git lfs install

# Clone submodules
echo "🔗 Updating submodules..."
git submodule update --init --recursive

# Build VantisOS (initial build)
echo "🏗️ Building VantisOS..."
make build || echo "Initial build failed (may be expected)"

# Setup pre-commit hooks
echo "🪝 Setting up pre-commit hooks..."
pip3 install pre-commit
pre-commit install || echo "Pre-commit setup failed (optional)"

echo "✅ Setup complete!"
echo ""
echo "🎉 VantisOS development environment is ready!"
echo ""
echo "Next steps:"
echo "  1. Open the project in VS Code"
echo "  2. Run 'make build' to build the system"
echo "  3. Run 'make test' to run tests"
echo "  4. Run 'make qemu' to start VantisOS in QEMU"
echo ""
echo "📚 Documentation: https://github.com/vantisCorp/VantisOS/wiki"