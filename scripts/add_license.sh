#!/bin/bash
# VANTIS HEADER INJECTOR
# Usage: ./scripts/add_license.sh

HEADER="/*
 * Copyright (c) 2026 VANTIS CORP.
 *
 * This source code is licensed under the Apache-2.0 license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * CODE IS LAW.
 */"

find src -name "*.rs" | while read filename; do
  if ! grep -q "VANTIS CORP" "$filename"; then
    echo "Stamping $filename..."
    echo "$HEADER" | cat - "$filename" > temp && mv temp "$filename"
  fi
done
