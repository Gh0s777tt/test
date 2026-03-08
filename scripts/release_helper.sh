#!/bin/bash
# Release Helper
# Automatyzuje proces release
# Scripts - Automatyzacja

set -e

echo "🚀 VantisOS - Release Helper"
echo "============================"

# Kolory
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Sprawdzenie czy jest na branchu main/0.4.1
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [[ "$CURRENT_BRANCH" != "0.4.1" ]]; then
    echo -e "${RED}Error: Not on branch 0.4.1${NC}"
    echo "Current branch: $CURRENT_BRANCH"
    exit 1
fi

# Sprawdzenie czy working tree jest czyste
if [[ -n $(git status --porcelain) ]]; then
    echo -e "${RED}Error: Working tree is not clean${NC}"
    git status
    exit 1
fi

# Zapytanie o wersję
echo ""
read -p "Enter new version (e.g., v1.5.0): " NEW_VERSION
echo ""

# Walidacja wersji
if [[ ! "$NEW_VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format. Use vX.Y.Z format.${NC}"
    exit 1
fi

echo -e "${BLUE}Release: ${NEW_VERSION}${NC}"
echo ""

# Sprawdzenie dokumentacji
echo "📋 Checking documentation..."
./scripts/docs_update_checker.sh
echo ""

# Sprawdzenie testów
echo "🧪 Running tests..."
cargo test --workspace
echo -e "${GREEN}✅ Tests passed${NC}"
echo ""

# Budowanie
echo "🔨 Building..."
make build
echo -e "${GREEN}✅ Build successful${NC}"
echo ""

# Tagowanie
echo "🏷️  Creating tag..."
git tag -a "$NEW_VERSION" -m "Release $NEW_VERSION"
echo -e "${GREEN}✅ Tag created${NC}"
echo ""

# Push tag
read -p "Push tag to remote? (y/n): " PUSH_TAG
if [[ "$PUSH_TAG" == "y" ]]; then
    git push origin 0.4.1
    git push origin "$NEW_VERSION"
    echo -e "${GREEN}✅ Tag pushed${NC}"
fi

echo ""
echo -e "${GREEN}🎉 Release $NEW_VERSION complete!${NC}"