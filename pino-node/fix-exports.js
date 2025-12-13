#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

const indexPath = path.join(__dirname, 'index.js');
let content = fs.readFileSync(indexPath, 'utf8');

// The pino.js compatibility additions
const pinoCompatCode = `
// Standard time functions (pino.js compatibility)
const stdTimeFunctions = {
  epochTime: () => \`,"time":\${Date.now()}\`,
  unixTime: () => \`,"time":\${Math.round(Date.now() / 1000)}\`,
  isoTime: () => \`,"time":"\${new Date().toISOString()}"\`,
  nullTime: () => ''
}

// Standard serializers (pino.js compatibility)
const stdSerializers = {
  req: (req) => ({
    method: req.method,
    url: req.url,
    headers: req.headers,
    remoteAddress: req.socket?.remoteAddress,
    remotePort: req.socket?.remotePort
  }),
  res: (res) => ({
    statusCode: res.statusCode,
    headers: res.getHeaders ? res.getHeaders() : {}
  }),
  err: (err) => ({
    type: err.constructor?.name || 'Error',
    message: err.message,
    stack: err.stack,
    ...(err.code && { code: err.code })
  })
}

// Log levels
const levels = {
  values: {
    fatal: 60,
    error: 50,
    warn: 40,
    info: 30,
    debug: 20,
    trace: 10
  },
  labels: {
    10: 'trace',
    20: 'debug',
    30: 'info',
    40: 'warn',
    50: 'error',
    60: 'fatal'
  }
}

module.exports = pino
module.exports.pino = pino
module.exports.Logger = Logger
module.exports.default = pino
module.exports.stdTimeFunctions = stdTimeFunctions
module.exports.stdSerializers = stdSerializers
module.exports.levels = levels
`;

// Check if already has stdTimeFunctions
if (content.includes('stdTimeFunctions')) {
  console.log('✓ Exports already fixed');
  process.exit(0);
}

// Replace the auto-generated exports with our custom ones
const oldExportsPattern1 = `const { Logger, pino } = nativeBinding

module.exports.Logger = Logger
module.exports.pino = pino`;

const oldExportsPattern2 = `const { Logger, pino } = nativeBinding

module.exports = pino
module.exports.pino = pino
module.exports.Logger = Logger
module.exports.default = pino`;

const newExports = `const { Logger, pino } = nativeBinding
${pinoCompatCode}`;

if (content.includes(oldExportsPattern1)) {
  content = content.replace(oldExportsPattern1, newExports);
  fs.writeFileSync(indexPath, content);
  console.log('✓ Fixed exports in index.js');
} else if (content.includes(oldExportsPattern2)) {
  content = content.replace(oldExportsPattern2, newExports);
  fs.writeFileSync(indexPath, content);
  console.log('✓ Fixed exports in index.js');
} else {
  // Fallback: append to end if pattern not found
  content = content.replace(
    /module\.exports\.default = pino\s*$/,
    pinoCompatCode
  );
  fs.writeFileSync(indexPath, content);
  console.log('✓ Fixed exports in index.js (fallback)');
}
