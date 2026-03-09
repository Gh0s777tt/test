#!/bin/bash
# Docs Update Checker
# Sprawdza czy dokumentacja jest aktualna

set -e

echo "🔍 VantisOS - Documentation Update Checker"
echo "=========================================="

# Kolory
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m'

# Zmienne
CURRENT_VERSION="v1.4.0"

echo ""
echo "Checking documentation versions..."
echo "Current version: ${GREEN}${CURRENT_VERSION}${NC}"
echo ""

# Sprawdzenie README
README_VERSION=$(grep -o "v[0-9.]*" README.md 2>/dev/null | head -1)
if [[ "$README_VERSION" == "$CURRENT_VERSION" ]]; then
    echo -e "README.md: ${GREEN}✅${NC} ($README_VERSION)"
else
    echo -e "README.md: ${RED}❌${NC} ($README_VERSION)"
fi

echo ""
echo "Documentation check complete!"
