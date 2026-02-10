#!/usr/bin/env bash
# ==========================================
# 🔮 VANTIS OS: INITIALIZATION PROTOCOL
# ==========================================

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo -e "\033[0;32m"
echo "   _    __            _   _      "
echo "  | |  / /___ _____  | |_(_)____ "
echo "  | | / / __ \`/ __ \ | __/ / ___/"
echo "  | |/ / /_/ / / / / | |_| (__  ) "
echo "  |___/\__,_/_/ /_/   \__/_/____/  "
echo -e "\033[0m"
echo ">> INITIALIZING DEVELOPMENT ENVIRONMENT..."

# 1. Check Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found! Please install rustup."
    exit 1
fi
echo "✅ Rust toolchain detected."

# 2. Install Pre-commit hooks
if [ -f ".pre-commit-config.yaml" ]; then
    echo ">> Installing Security Hooks..."
    if command -v python3 &> /dev/null; then
        python3 -m pip install --user pre-commit
    else
        echo "⚠️ Python3 not found. Cannot install pre-commit automatically."
    fi
    if command -v pre-commit &> /dev/null; then
        pre-commit install
        echo "✅ Hooks installed. You are now protected."
    else
        echo "⚠️ pre-commit command not found in PATH."
    fi
else
    echo "⚠️ No pre-commit config found. Skipping."
fi

# 3. Check Docker
if ! command -v docker &> /dev/null; then
    echo "⚠️ Docker not found. Orbital Station will not work."
else
    echo "✅ Docker detected."
fi

echo ""
echo ">> SYSTEM READY. WELCOME TO THE CITADEL."
echo ">> Run 'cargo build' to compile the kernel."
