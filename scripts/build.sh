#!/usr/bin/env bash

set -euo pipefail

# Binary name defined in cli/Cargo.toml
BINARY_NAME="ts-validator"
PACKAGE_NAME="cli"

# Output directory for npm wrapper
BIN_DIR="../npm-wrapper/bin"
mkdir -p "$BIN_DIR"

TARGETS=(
  "x86_64-unknown-linux-gnu"
  "aarch64-unknown-linux-gnu"
  "x86_64-apple-darwin"
  "aarch64-apple-darwin"
  "x86_64-pc-windows-gnu"
)

for TARGET in "${TARGETS[@]}"; do
  echo "🔨 Building for $TARGET..."
  cross build --release --target "$TARGET" --package "$PACKAGE_NAME"

  case "$TARGET" in
    *windows*) OUT_NAME="${BINARY_NAME}-${TARGET}.exe"; SRC_NAME="${BINARY_NAME}.exe" ;;
    *)         OUT_NAME="${BINARY_NAME}-${TARGET}"; SRC_NAME="${BINARY_NAME}" ;;
  esac

  SRC_PATH="target/$TARGET/release/$SRC_NAME"
  DEST_PATH="$BIN_DIR/$OUT_NAME"

  if [[ ! -f "$SRC_PATH" ]]; then
    echo "❌ Build failed or binary missing: $SRC_PATH"
    exit 1
  fi

  cp "$SRC_PATH" "$DEST_PATH"
  chmod +x "$DEST_PATH"
  echo "✅ Copied to $DEST_PATH"
done

echo "🎉 All binaries built and copied to $BIN_DIR"
