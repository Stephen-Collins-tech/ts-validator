#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');
const os = require('os');

// Import version from the generated version.js file
let version;
try {
  version = require('./version');
} catch (err) {
  // Fallback if version.js doesn't exist (e.g., during development)
  version = 'v1.0.0';
  console.warn('Warning: version.js not found, using fallback version');
}

const platformMap = {
  darwin: 'apple-darwin',
  linux: 'unknown-linux-gnu',
  win32: 'pc-windows-gnu'
};

const archMap = {
  x64: 'x86_64',
  arm64: 'aarch64',
};

const platform = os.platform();
const arch = os.arch();

if (!platformMap[platform] || !archMap[arch]) {
  console.error(`Unsupported platform or architecture: ${platform} ${arch}`);
  process.exit(1);
}

// Target format matching the one produced by build.sh
const target = `${archMap[arch]}-${platformMap[platform]}`;
const binaryName = 'ts-validator';
const executableName = platform === 'win32' ? `${binaryName}.exe` : binaryName;
const downloadName = `${binaryName}-${target}${platform === 'win32' ? '.exe' : ''}`;

const destDir = path.resolve(__dirname, 'bin');
const destPath = path.join(destDir, downloadName);

// Ensure bin/ exists
fs.mkdirSync(destDir, { recursive: true });

// For local development - check if the binary already exists
if (fs.existsSync(destPath)) {
  console.log(`✅ Binary already exists at ${destPath}. Skipping download.`);
  // Check if file is executable
  try {
    fs.accessSync(destPath, fs.constants.X_OK);
  } catch (err) {
    // Make executable if not already
    fs.chmodSync(destPath, 0o755);
    console.log('   Made binary executable.');
  }
  process.exit(0);
}

// Check for SKIP_BINARY_DOWNLOAD env var (useful for local testing)
if (process.env.SKIP_BINARY_DOWNLOAD) {
  console.log('⚠️ SKIP_BINARY_DOWNLOAD is set, skipping binary download.');
  console.log('⚠️ Make sure you have manually built and copied the binary to:', destPath);
  process.exit(0);
}

const url = `https://github.com/ts-validator/ts-validator/releases/download/${version}/${downloadName}`;
console.log(`📦 Downloading binary from: ${url}`);

// Download the binary
https.get(url, res => {
  if (res.statusCode !== 200) {
    console.error(`Failed to download binary: ${res.statusCode} ${res.statusMessage}`);
    console.error(`If you're developing locally, you can set SKIP_BINARY_DOWNLOAD=1 and build manually.`);
    process.exit(1);
  }

  const file = fs.createWriteStream(destPath, { mode: 0o755 });
  res.pipe(file);

  file.on('finish', () => {
    file.close(() => {
      console.log(`✅ Installed binary to ${destPath}`);
    });
  });
}).on('error', err => {
  console.error(`Download error: ${err.message}`);
  console.error(`If you're developing locally, you can set SKIP_BINARY_DOWNLOAD=1 and build manually.`);
  process.exit(1);
});
