#!/usr/bin/env bash

set -euo pipefail

# Move to repo root for consistent paths
cd "$(dirname "$0")/.."
REPO_ROOT=$(pwd)

# Get version from git tag
VERSION=$(git describe --tags --abbrev=0 2>/dev/null || echo "v1.0.0")
# Remove 'v' prefix for package.json and cargo version
NPM_VERSION="${VERSION#v}"
echo "📦 Using version: $VERSION (Cargo/NPM: $NPM_VERSION)"

# Update CLI crate version
CLI_CARGO_TOML="${REPO_ROOT}/crates/cli/Cargo.toml"
echo "🔄 Updating CLI crate version to ${NPM_VERSION}"
sed -i.bak "s/^version = \"[^\"]*\"/version = \"${NPM_VERSION}\"/" "${CLI_CARGO_TOML}"
rm "${CLI_CARGO_TOML}.bak"

# Build for current platform only
echo "🔨 Building binary for local testing..."

# Determine current platform target
case "$(uname -s)" in
  Darwin*)
    PLATFORM="apple-darwin"
    ;;
  Linux*)
    PLATFORM="unknown-linux-gnu"
    ;;
  MINGW*|MSYS*|CYGWIN*)
    PLATFORM="pc-windows-gnu"
    ;;
  *)
    echo "❌ Unsupported platform: $(uname -s)"
    exit 1
    ;;
esac

case "$(uname -m)" in
  x86_64|amd64)
    ARCH="x86_64"
    ;;
  arm64|aarch64)
    ARCH="aarch64"
    ;;
  *)
    echo "❌ Unsupported architecture: $(uname -m)"
    exit 1
    ;;
esac

TARGET="${ARCH}-${PLATFORM}"
echo "🎯 Detected target: ${TARGET}"

# Build binary
echo "🔨 Building ts-validator for ${TARGET}..."
cargo build --release --package cli

# Determine binary extension based on platform
if [[ $PLATFORM == *"windows"* ]]; then
  SRC_NAME="ts-validator.exe"
  DEST_NAME="ts-validator-${TARGET}.exe"
else
  SRC_NAME="ts-validator"
  DEST_NAME="ts-validator-${TARGET}"
fi

# Create npm-wrapper/bin directory if it doesn't exist
BIN_DIR="${REPO_ROOT}/npm-wrapper/bin"
mkdir -p "${BIN_DIR}"

# Copy binary to npm-wrapper/bin
SRC_PATH="${REPO_ROOT}/target/release/${SRC_NAME}"
DEST_PATH="${BIN_DIR}/${DEST_NAME}"

if [[ ! -f "${SRC_PATH}" ]]; then
  echo "❌ Build failed or binary missing: ${SRC_PATH}"
  exit 1
fi

cp "${SRC_PATH}" "${DEST_PATH}"
chmod +x "${DEST_PATH}"
echo "✅ Copied binary to ${DEST_PATH}"

# Set up npm link with SKIP_BINARY_DOWNLOAD
cd "${REPO_ROOT}/npm-wrapper"
echo "🔄 Setting up npm link..."
export SKIP_BINARY_DOWNLOAD=1
npm link

echo "✅ Local setup complete! You can now run 'ts-validator' to test."
echo ""
echo "🧪 Try running: ts-validator --version"
echo "🧪 Or: ts-validator --help" 