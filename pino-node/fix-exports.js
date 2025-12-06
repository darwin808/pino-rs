#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

const indexPath = path.join(__dirname, 'index.js');
let content = fs.readFileSync(indexPath, 'utf8');

// Check if exports are already correct
if (content.includes('module.exports = pino')) {
  console.log('Exports already fixed');
  process.exit(0);
}

// Replace the auto-generated exports with our custom ones
const oldExports = `const { Logger, pino } = nativeBinding

module.exports.Logger = Logger
module.exports.pino = pino`;

const newExports = `const { Logger, pino } = nativeBinding

module.exports = pino
module.exports.pino = pino
module.exports.Logger = Logger
module.exports.default = pino`;

if (content.includes(oldExports)) {
  content = content.replace(oldExports, newExports);
  fs.writeFileSync(indexPath, content);
  console.log('✓ Fixed exports in index.js');
} else {
  console.log('⚠ Could not find expected exports pattern');
}
