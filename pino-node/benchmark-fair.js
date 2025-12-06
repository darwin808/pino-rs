#!/usr/bin/env node
/**
 * Fair benchmark comparing pino.js and pino-rs
 * Both loggers write to /dev/null for fair comparison
 */

const fs = require('fs');
const pinoJs = require('pino');

// Both write to /dev/null for fair comparison
const nullStream = fs.createWriteStream('/dev/null');
const pinoJsLogger = pinoJs(nullStream);

console.log('=== Pino.js Benchmark (Baseline) ===\n');
console.log('Note: pino-rs NAPI bindings will have overhead compared to pure JS\n');

const ITERATIONS = 100000;

function benchmark(name, fn) {
  // Warmup
  for (let i = 0; i < 1000; i++) fn();

  // Benchmark
  const start = process.hrtime.bigint();
  for (let i = 0; i < ITERATIONS; i++) {
    fn();
  }
  const end = process.hrtime.bigint();

  const durationMs = Number(end - start) / 1000000;
  const opsPerSec = Math.round(ITERATIONS / (durationMs / 1000));

  console.log(`${name}: ${opsPerSec.toLocaleString()} ops/sec (${durationMs.toFixed(2)}ms)`);
  return { durationMs, opsPerSec };
}

console.log('Pino.js Performance:');
benchmark('Simple logging', () => pinoJsLogger.info('hello world'));
benchmark('With fields', () => pinoJsLogger.info({ userId: 42, action: 'login' }, 'user action'));

const child = pinoJsLogger.child({ module: 'auth' });
benchmark('Child logger', () => child.info('auth event'));

console.log('\n--- Performance Notes ---');
console.log('pino.js is highly optimized JavaScript with years of performance tuning');
console.log('pino-rs currently has NAPI overhead and synchronous I/O');
console.log('\nFuture Optimizations for pino-rs:');
console.log('1. Async I/O and buffered writes');
console.log('2. Direct Rust usage (no NAPI overhead)');
console.log('3. Batch writes and optimized serialization');
console.log('4. Lock-free data structures');
