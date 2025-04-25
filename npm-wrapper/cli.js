#!/usr/bin/env node

const { spawnSync } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

const platformMap = {
  darwin: 'apple-darwin',
  linux: 'unknown-linux-gnu',
  win32: 'pc-windows-gnu'
};

const archMap = {
  x64: 'x86_64',
  arm64: 'aarch64'
};

const platform = os.platform();
const arch = os.arch();

const binaryName = `ts-validator-${archMap[arch]}-${platformMap[platform]}${platform === 'win32' ? '.exe' : ''}`;
const binaryPath = path.resolve(__dirname, 'bin', binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error(`Binary not found: ${binaryPath}`);
  console.error('Did install.js fail to download the correct binary?');
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), { stdio: 'inherit' });

process.exit(result.status ?? 1);
