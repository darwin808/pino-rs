#!/usr/bin/env node
/**
 * Memory usage benchmark comparing pino.js and pino-rs
 */

const fs = require('fs');
const pinoJs = require('pino');
const pinoRs = require('./index');

const nullStream = fs.createWriteStream('/dev/null');

function formatBytes(bytes) {
  return (bytes / 1024 / 1024).toFixed(2) + ' MB';
}

function getMemoryUsage() {
  const usage = process.memoryUsage();
  return {
    rss: usage.rss,          // Resident Set Size
    heapTotal: usage.heapTotal,
    heapUsed: usage.heapUsed,
    external: usage.external
  };
}

console.log('=== Memory Usage Benchmark ===\n');

// Force garbage collection if available
if (global.gc) {
  global.gc();
} else {
  console.log('Run with --expose-gc for accurate memory measurements\n');
}

const ITERATIONS = 100000;

// Baseline memory
const baseline = getMemoryUsage();
console.log('Baseline Memory:');
console.log(`  RSS: ${formatBytes(baseline.rss)}`);
console.log(`  Heap Used: ${formatBytes(baseline.heapUsed)}`);
console.log(`  Heap Total: ${formatBytes(baseline.heapTotal)}\n`);

// Test pino.js
console.log('Testing pino.js...');
const pinoJsLogger = pinoJs(nullStream);
const beforePinoJs = getMemoryUsage();

for (let i = 0; i < ITERATIONS; i++) {
  pinoJsLogger.info({ userId: i, action: 'test', data: 'sample' }, 'test message');
}

if (global.gc) global.gc();
const afterPinoJs = getMemoryUsage();

const pinoJsMemory = {
  rss: afterPinoJs.rss - beforePinoJs.rss,
  heapUsed: afterPinoJs.heapUsed - beforePinoJs.heapUsed
};

console.log(`  RSS Increase: ${formatBytes(pinoJsMemory.rss)}`);
console.log(`  Heap Used Increase: ${formatBytes(pinoJsMemory.heapUsed)}\n`);

// Clear and reset
if (global.gc) {
  global.gc();
  global.gc();
}

// Test pino-rs
console.log('Testing pino-rs...');
const pinoRsLogger = pinoRs();
const beforePinoRs = getMemoryUsage();

for (let i = 0; i < ITERATIONS; i++) {
  pinoRsLogger.info({ userId: i, action: 'test', data: 'sample' }, 'test message');
}

if (global.gc) global.gc();
const afterPinoRs = getMemoryUsage();

const pinoRsMemory = {
  rss: afterPinoRs.rss - beforePinoRs.rss,
  heapUsed: afterPinoRs.heapUsed - beforePinoRs.heapUsed
};

console.log(`  RSS Increase: ${formatBytes(pinoRsMemory.rss)}`);
console.log(`  Heap Used Increase: ${formatBytes(pinoRsMemory.heapUsed)}\n`);

// Comparison
console.log('=== Memory Comparison ===\n');
console.log('┌──────────────────┬─────────────┬─────────────┬─────────────┐');
console.log('│ Metric           │ pino.js     │ pino-rs     │ Difference  │');
console.log('├──────────────────┼─────────────┼─────────────┼─────────────┤');

const rssDiff = ((pinoRsMemory.rss / pinoJsMemory.rss - 1) * 100).toFixed(1);
const heapDiff = ((pinoRsMemory.heapUsed / pinoJsMemory.heapUsed - 1) * 100).toFixed(1);

console.log(`│ RSS Increase     │ ${formatBytes(pinoJsMemory.rss).padStart(11)} │ ${formatBytes(pinoRsMemory.rss).padStart(11)} │ ${rssDiff.padStart(10)}% │`);
console.log(`│ Heap Increase    │ ${formatBytes(pinoJsMemory.heapUsed).padStart(11)} │ ${formatBytes(pinoRsMemory.heapUsed).padStart(11)} │ ${heapDiff.padStart(10)}% │`);
console.log('└──────────────────┴─────────────┴─────────────┴─────────────┘\n');

if (pinoRsMemory.rss < pinoJsMemory.rss) {
  const savings = ((1 - pinoRsMemory.rss / pinoJsMemory.rss) * 100).toFixed(1);
  console.log(`✓ pino-rs uses ${savings}% less memory (RSS)`);
} else {
  const increase = ((pinoRsMemory.rss / pinoJsMemory.rss - 1) * 100).toFixed(1);
  console.log(`⚠ pino-rs uses ${increase}% more memory (RSS)`);
}

if (pinoRsMemory.heapUsed < pinoJsMemory.heapUsed) {
  const savings = ((1 - pinoRsMemory.heapUsed / pinoJsMemory.heapUsed) * 100).toFixed(1);
  console.log(`✓ pino-rs uses ${savings}% less heap memory`);
} else {
  const increase = ((pinoRsMemory.heapUsed / pinoJsMemory.heapUsed - 1) * 100).toFixed(1);
  console.log(`⚠ pino-rs uses ${increase}% more heap memory`);
}

console.log('\nNote: Run with `node --expose-gc benchmark-memory.js` for accurate measurements');
console.log(`Iterations: ${ITERATIONS.toLocaleString()}`);
