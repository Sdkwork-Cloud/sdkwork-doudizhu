#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { joinPath } from '@sdkwork/utils';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const manifests = [
  joinPath('sdks', '_route-manifests', 'app-api', 'sdkwork-router-health-app-api.route-manifest.json'),
  joinPath('sdks', '_route-manifests', 'app-api', 'sdkwork-router-match-app-api.route-manifest.json'),
  joinPath('sdks', '_route-manifests', 'backend-api', 'sdkwork-router-match-backend-api.route-manifest.json'),
];

for (const relativePath of manifests) {
  const fullPath = path.join(root, relativePath);
  if (!fs.existsSync(fullPath)) {
    console.error(`[doudizhu-route-manifest] missing ${relativePath}`);
    process.exit(1);
  }
}

process.stdout.write('[doudizhu-route-manifest] route manifests present\n');
