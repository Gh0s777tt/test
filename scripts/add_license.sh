#!/usr/bin/env bash
# VANTIS HEADER INJECTOR
# Usage: ./scripts/add_license.sh

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

HEADER="/*
 * Copyright (c) 2026 VANTIS CORP.
 *
 * This source code is licensed under the Apache-2.0 license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * CODE IS LAW.
 */"

rg --files src -g '*.rs' | while IFS= read -r filename; do
  if ! rg -q "VANTIS CORP" "$filename"; then
    echo "Stamping $filename..."
    tmp="$(mktemp)"
    {
      echo "$HEADER"
      cat "$filename"
    } > "$tmp"
    mv "$tmp" "$filename"
  fi
done
