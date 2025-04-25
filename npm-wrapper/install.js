#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');
const os = require('os');

const platformMap = {
  darwin: 'macos',
  linux: 'linux',
  win32: 'windows',
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

const target = `${archMap[arch]}-${platformMap[platform]}`;
const version = 'v1.0.0'; // replace with your actual version
const binaryName = platform === 'win32' ? 'your-tool.exe' : 'your-tool';
const url = `https://github.com/YOUR_USERNAME/your-tool/releases/download/${version}/${binaryName}-${target}`;

const destDir = path.resolve(__dirname, 'bin');
const destPath = path.join(destDir, binaryName);

// Ensure bin/ exists
fs.mkdirSync(destDir, { recursive: true });

// Download the binary
https.get(url, res => {
  if (res.statusCode !== 200) {
    console.error(`Failed to download binary: ${res.statusCode} ${res.statusMessage}`);
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
  process.exit(1);
});
