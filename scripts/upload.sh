#!/usr/bin/env bash

set -euo pipefail

BIN_DIR="../npm-wrapper/bin"
TAG="${1:-}"
REPO="Stephen-Collins-tech/ts-validator"  # CHANGE this to your GitHub repo slug

if [[ -z "$TAG" ]]; then
  echo "❌ Usage: ./upload.sh v1.0.0"
  exit 1
fi

# Ensure tag has 'v' prefix
if [[ "$TAG" != v* ]]; then
  echo "❌ Tag should start with 'v' (e.g., v0.0.1)"
  exit 1
fi

# Ensure the GitHub CLI is authenticated
if ! gh auth status &>/dev/null; then
  echo "❌ GitHub CLI not authenticated. Run: gh auth login"
  exit 1
fi

# Check if release already exists, or create it
if gh release view "$TAG" --repo "$REPO" &>/dev/null; then
  echo "📦 Found existing release $TAG"
else
  echo "🚀 Creating release $TAG"
  gh release create "$TAG" --repo "$REPO" --title "$TAG" --notes "Automated release for $TAG"
fi

# Upload all binaries
echo "⬆️ Uploading binaries to GitHub release $TAG"
for FILE in "$BIN_DIR"/*; do
  FILENAME=$(basename "$FILE")

  echo "  - Uploading $FILENAME..."
  gh release upload "$TAG" "$FILE" --repo "$REPO" --clobber
done

echo "✅ Upload complete."
