#!/bin/bash
# Version Sync
# Synchronizuje wersje w wszystkich plikach
# Scripts - Automatyzacja

set -e

echo "🔄 VantisOS - Version Sync"
echo "=========================="

# Kolory
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Aktualna wersja
CURRENT_VERSION="v1.4.0"
CURRENT_VERSION_NO_V="1.4.0"

# Zapytanie o nową wersję
read -p "Enter new version (e.g., v1.5.0) [default: $CURRENT_VERSION]: " NEW_VERSION
NEW_VERSION=${NEW_VERSION:-$CURRENT_VERSION}
NEW_VERSION_NO_V=${NEW_VERSION#v}

echo ""
echo -e "${BLUE}Updating version to: ${NEW_VERSION}${NC}"
echo ""

# Lista plików do aktualizacji
declare -a FILES=(
    "README.md"
    "CHANGELOG.md"
    "ROADMAP.md"
    "TODO.md"
    "MASTER_TODO.md"
    "docs/releases/RELEASE_NOTES.md"
    "docs/api/API_REFERENCE.md"
    "docs/guides/USER_GUIDE.md"
    "docs/security/SECURITY.md"
    "CITATION.cff"
    "Cargo.toml"
)

# Aktualizacja wersji w plikach
for file in "${FILES[@]}"; do
    if [[ -f "$file" ]]; then
        # Zamiana wersji z v
        sed -i "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/${NEW_VERSION}/g" "$file" 2>/dev/null || true
        # Zamiana wersji bez v
        sed -i "s/[0-9]\+\.[0-9]\+\.[0-9]\+/${NEW_VERSION_NO_V}/g" "$file" 2>/dev/null || true
        echo -e "${GREEN}✅${NC} Updated: $file"
    else
        echo -e "${YELLOW}⚠️${NC} Not found: $file"
    fi
done

# Aktualizacja daty
TODAY=$(date +"%B %d, %Y")
TODAY_PL=$(date +"%d %B %Y" | sed 's/January/stycznia/;s/February/lutego/;s/March/marca/;s/April/kwietnia/;s/May/maja/;s/June/czerwca/;s/July/lipca/;s/August/sierpnia/;s/September/września/;s/October/października/;s/November/listopada/;s/December/grudnia/')

echo ""
echo -e "${GREEN}✅ Version sync complete!${NC}"
echo ""
echo "Updated files: ${#FILES[@]}"
echo ""
echo "Run 'git diff' to review changes."