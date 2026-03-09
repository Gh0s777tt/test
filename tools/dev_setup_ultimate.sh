#!/bin/bash
set -e

echo "🔮 VANTIS OS: ULTIMATE SETUP"

# 1. System Dependencies (Linux/Mac)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    sudo apt update && sudo apt install -y qemu-system-x86 nasm lld
elif [[ "$OSTYPE" == "darwin"* ]]; then
    brew install qemu nasm llvm
fi

# 2. Rust Nightly (Required for OS Dev)
rustup override set nightly
rustup component add rust-src llvm-tools-preview

# 3. Git Hooks Install
cp scripts/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

# 4. Generate Local Dev Keys (For Signing)
if [ ! -f "certs/dev.key" ]; then
    mkdir -p certs
    openssl genrsa -out certs/dev.key 2048
    echo "✅ Dev Keys Generated"
fi

echo "🚀 ENVIRONMENT READY. TYPE 'just run' TO BOOT."
