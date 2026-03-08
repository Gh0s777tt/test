#!/bin/bash
# Docs Update Checker
# Sprawdza czy dokumentacja jest aktualna
# Scripts - Automatyzacja

set -e

echo "🔍 VantisOS - Documentation Update Checker"
echo "=========================================="

# Kolory
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Zmienne
CURRENT_VERSION="v1.4.0"
README_VERSION=$(grep -o "v[0-9.]*" README.md | head -1)
CHANGELOG_VERSION=$(grep -o "^## v[0-9.]*" CHANGELOG.md | head -1 | cut -d' ' -f2)
ROADMAP_VERSION=$(grep -o "v[0-9.]*" ROADMAP.md | head -1)

echo ""
echo "Checking documentation versions..."
echo "Current version: ${GREEN}${CURRENT_VERSION}${NC}"
echo ""

# Sprawdzenie README
if [[ "$README_VERSION" == "$CURRENT_VERSION" ]]; then
    echo -e "README.md: ${GREEN}✅${NC} ($README_VERSION)"
else
    echo -e "README.md: ${RED}❌${NC} ($README_VERSION != $CURRENT_VERSION)"
fi

# Sprawdzenie CHANGELOG
if [[ "$CHANGELOG_VERSION" == "$CURRENT_VERSION" ]]; then
    echo -e "CHANGELOG.md: ${GREEN}✅${NC} ($CHANGELOG_VERSION)"
else
    echo -e "CHANGELOG.md: ${YELLOW}⚠️${NC} ($CHANGELOG_VERSION != $CURRENT_VERSION)"
fi

# Sprawdzenie ROADMAP
if [[ "$ROADMAP_VERSION" == "$CURRENT_VERSION" ]]; then
    echo -e "ROADMAP.md: ${GREEN}✅${NC} ($ROADMAP_VERSION)"
else
    echo -e "ROADMAP.md: ${YELLOW}⚠️${NC} ($ROADMAP_VERSION != $CURRENT_VERSION)"
fi

echo ""
echo "Documentation check complete!"
echo "Run 'make fmt' to format all documentation files."