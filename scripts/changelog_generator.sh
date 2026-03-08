#!/bin/bash
# Changelog Generator
# Automatycznie generuje changelog z commit messages
# Scripts - Automatyzacja

set -e

echo "📝 VantisOS - Changelog Generator"
echo "=================================="

# Kolory
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Zmienne
OUTPUT_FILE="CHANGELOG.md"
TEMP_FILE=$(mktemp)

# Sprawdzenie ostatniego tagu
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
if [[ -z "$LAST_TAG" ]]; then
    echo -e "${YELLOW}No previous tag found${NC}"
    COMMITS=$(git log --pretty=format:"%h - %s" --reverse)
else
    echo "Last tag: $LAST_TAG"
    COMMITS=$(git log ${LAST_TAG}..HEAD --pretty=format:"%h - %s" --reverse)
fi

# Parsowanie commitów
echo ""
echo "Parsing commits..."
echo ""

# Nagłówek
cat > "$TEMP_FILE" << EOF
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

EOF

# Dodanie nowych zmian
if [[ -n "$COMMITS" ]]; then
    echo "## [Unreleased]" >> "$TEMP_FILE"
    echo "" >> "$TEMP_FILE"
    
    # Kategoryzowanie commitów
    ADDED=0
    CHANGED=0
    FIXED=0
    REMOVED=0
    
    while IFS= read -r commit; do
        if [[ "$commit" == *"feat:"* ]] || [[ "$commit" == *"add"* ]]; then
            if [[ $ADDED -eq 0 ]]; then
                echo "### Added" >> "$TEMP_FILE"
                ADDED=1
            fi
            echo "- ${commit#*: }" >> "$TEMP_FILE"
        elif [[ "$commit" == *"fix:"* ]]; then
            if [[ $FIXED -eq 0 ]]; then
                echo "" >> "$TEMP_FILE"
                echo "### Fixed" >> "$TEMP_FILE"
                FIXED=1
            fi
            echo "- ${commit#*: }" >> "$TEMP_FILE"
        elif [[ "$commit" == *"chore:"* ]] || [[ "$commit" == *"update"* ]]; then
            if [[ $CHANGED -eq 0 ]]; then
                echo "" >> "$TEMP_FILE"
                echo "### Changed" >> "$TEMP_FILE"
                CHANGED=1
            fi
            echo "- ${commit#*: }" >> "$TEMP_FILE"
        elif [[ "$commit" == *"remove"* ]] || [[ "$commit" == *"delete"* ]]; then
            if [[ $REMOVED -eq 0 ]]; then
                echo "" >> "$TEMP_FILE"
                echo "### Removed" >> "$TEMP_FILE"
                REMOVED=1
            fi
            echo "- ${commit#*: }" >> "$TEMP_FILE"
        fi
    done <<< "$COMMITS"
    
    echo "" >> "$TEMP_FILE"
fi

# Dodanie starego changelog
if [[ -f "$OUTPUT_FILE" ]]; then
    # Pomiń nagłówek starego changelog
    tail -n +4 "$OUTPUT_FILE" >> "$TEMP_FILE"
fi

# Przeniesienie do pliku wyjściowego
mv "$TEMP_FILE" "$OUTPUT_FILE"

echo -e "${GREEN}✅ Changelog generated!${NC}"
echo ""
echo "Review: CHANGELOG.md"
echo ""