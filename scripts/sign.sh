#!/usr/bin/env bash
# Sign a file using cosign.
# Usage:
#   ./scripts/sign.sh <file>

set -euo pipefail

TARGET_FILE="${1:-}"
if [[ -z "$TARGET_FILE" ]]; then
  echo "Usage: $0 <file>" >&2
  exit 1
fi
if [[ ! -f "$TARGET_FILE" ]]; then
  echo "File not found: $TARGET_FILE" >&2
  exit 1
fi

if ! command -v cosign >/dev/null 2>&1; then
  echo "cosign is required but not installed." >&2
  exit 1
fi

cosign sign-blob "$TARGET_FILE"
echo "Signed: $TARGET_FILE"
