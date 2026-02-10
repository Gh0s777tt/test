#!/usr/bin/env bash
# Generate or verify SHA-256 checksums.
# Usage:
#   ./scripts/checksum.sh <file> [output.sha256]
#   ./scripts/checksum.sh --verify <checksum.sha256>

set -euo pipefail

if ! command -v sha256sum >/dev/null 2>&1; then
  echo "sha256sum is required but not installed." >&2
  exit 1
fi

if [[ "${1:-}" == "--verify" ]]; then
  CHECK_FILE="${2:-}"
  if [[ -z "$CHECK_FILE" ]]; then
    echo "Usage: $0 --verify <checksum.sha256>" >&2
    exit 1
  fi
  sha256sum --check "$CHECK_FILE"
  exit 0
fi

TARGET_FILE="${1:-}"
OUTPUT_FILE="${2:-}"
if [[ -z "$TARGET_FILE" ]]; then
  echo "Usage: $0 <file> [output.sha256]" >&2
  exit 1
fi
if [[ ! -f "$TARGET_FILE" ]]; then
  echo "File not found: $TARGET_FILE" >&2
  exit 1
fi

if [[ -z "$OUTPUT_FILE" ]]; then
  OUTPUT_FILE="${TARGET_FILE}.sha256"
fi

sha256sum "$TARGET_FILE" > "$OUTPUT_FILE"
echo "Checksum written to: $OUTPUT_FILE"
