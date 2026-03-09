#!/bin/bash
# Release Script
# Kompleksowy skrypt release

set -e

echo "🚀 VantisOS - Release Script"
echo "============================"

VERSION="${1:-v1.4.0}"

echo "Version: $VERSION"
echo ""

# Sprawdzenie czystości repo
if [[ -n $(git status --porcelain) ]]; then
    echo "❌ Working tree not clean"
    git status
    exit 1
fi

# Testy
echo "🧪 Running tests..."
cargo test --workspace

# Build
echo "🔨 Building..."
make build

# Dokumentacja
echo "📚 Generating documentation..."
./scripts/generate_docs.sh

# Tag
echo "🏷️  Creating tag..."
git tag -a "$VERSION" -m "Release $VERSION"

# Changelog
echo "📝 Updating changelog..."
./scripts/changelog_generator.sh 2>/dev/null || echo "Changelog script not available"

echo ""
echo "✅ Release $VERSION prepared!"
echo ""
echo "Next steps:"
echo "  git push origin 0.4.1"
echo "  git push origin $VERSION"
echo "  Create GitHub release"
