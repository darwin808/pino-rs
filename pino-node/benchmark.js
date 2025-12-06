const fs = require('fs');

// Suppress output by writing to /dev/null
const nullStream = fs.createWriteStream('/dev/null');

// Load both loggers
const pinoJs = require('pino');
const pinoRs = require('./index');

// Create loggers that write to /dev/null for fair comparison
const pinoJsLogger = pinoJs(nullStream);

// For pino-rs, we'll redirect stdout at the file descriptor level
// Create logger (output will be redirected during benchmarks)
const pinoRsLogger = pinoRs();

const ITERATIONS = 100000;

function benchmark(name, fn, isPinoRs = false) {
  // Warmup
  for (let i = 0; i < 1000; i++) fn();

  // For pino-rs, we need to redirect stdout at the FD level
  let originalStdoutFd;
  let devNullFd;
  if (isPinoRs) {
    const os = require('os');
    if (os.platform() !== 'win32') {
      originalStdoutFd = fs.openSync('/dev/stdout', 'w');
      devNullFd = fs.openSync('/dev/null', 'w');
      // Duplicate stdout and redirect to /dev/null
      const savedFd = fs.dup(1);
      fs.dup2(devNullFd, 1);
    }
  }

  // Actual benchmark
  const start = process.hrtime.bigint();
  for (let i = 0; i < ITERATIONS; i++) {
    fn();
  }
  const end = process.hrtime.bigint();

  // Restore stdout for pino-rs
  if (isPinoRs && devNullFd) {
    const os = require('os');
    if (os.platform() !== 'win32') {
      // Restore original stdout
      fs.closeSync(devNullFd);
    }
  }

  const durationMs = Number(end - start) / 1000000;
  const opsPerSec = Math.round(ITERATIONS / (durationMs / 1000));

  return { durationMs, opsPerSec };
}

console.log('=== Pino-RS vs Pino.js Benchmark ===\n');
console.log(`Iterations: ${ITERATIONS.toLocaleString()}\n`);

const benchmarks = [];

// Benchmark 1: Simple string logging
console.log('1. Simple string logging');
const pinoJsSimple = benchmark('pino.js simple', () => {
  pinoJsLogger.info('hello world');
});
const pinoRsSimple = benchmark('pino-rs simple', () => {
  pinoRsLogger.info('hello world');
});
benchmarks.push({
  name: 'Simple logging',
  pinoJs: pinoJsSimple,
  pinoRs: pinoRsSimple,
  speedup: pinoRsSimple.opsPerSec / pinoJsSimple.opsPerSec
});
console.log(`  pino.js: ${pinoJsSimple.opsPerSec.toLocaleString()} ops/sec (${pinoJsSimple.durationMs.toFixed(2)}ms)`);
console.log(`  pino-rs: ${pinoRsSimple.opsPerSec.toLocaleString()} ops/sec (${pinoRsSimple.durationMs.toFixed(2)}ms)`);
console.log(`  Speedup: ${(pinoRsSimple.opsPerSec / pinoJsSimple.opsPerSec).toFixed(2)}x\n`);

// Benchmark 2: Logging with object fields
console.log('2. Logging with object fields');
const pinoJsFields = benchmark('pino.js fields', () => {
  pinoJsLogger.info({ userId: 42, action: 'login', ip: '192.168.1.1' }, 'user action');
});
const pinoRsFields = benchmark('pino-rs fields', () => {
  pinoRsLogger.info({ userId: 42, action: 'login', ip: '192.168.1.1' }, 'user action');
});
benchmarks.push({
  name: 'With fields',
  pinoJs: pinoJsFields,
  pinoRs: pinoRsFields,
  speedup: pinoRsFields.opsPerSec / pinoJsFields.opsPerSec
});
console.log(`  pino.js: ${pinoJsFields.opsPerSec.toLocaleString()} ops/sec (${pinoJsFields.durationMs.toFixed(2)}ms)`);
console.log(`  pino-rs: ${pinoRsFields.opsPerSec.toLocaleString()} ops/sec (${pinoRsFields.durationMs.toFixed(2)}ms)`);
console.log(`  Speedup: ${(pinoRsFields.opsPerSec / pinoJsFields.opsPerSec).toFixed(2)}x\n`);

// Benchmark 3: Child logger
console.log('3. Child logger');
const pinoJsChild = pinoJsLogger.child({ module: 'auth' });
const pinoRsChild = pinoRsLogger.child({ module: 'auth' });
const pinoJsChildBench = benchmark('pino.js child', () => {
  pinoJsChild.info('authentication event');
});
const pinoRsChildBench = benchmark('pino-rs child', () => {
  pinoRsChild.info('authentication event');
});
benchmarks.push({
  name: 'Child logger',
  pinoJs: pinoJsChildBench,
  pinoRs: pinoRsChildBench,
  speedup: pinoRsChildBench.opsPerSec / pinoJsChildBench.opsPerSec
});
console.log(`  pino.js: ${pinoJsChildBench.opsPerSec.toLocaleString()} ops/sec (${pinoJsChildBench.durationMs.toFixed(2)}ms)`);
console.log(`  pino-rs: ${pinoRsChildBench.opsPerSec.toLocaleString()} ops/sec (${pinoRsChildBench.durationMs.toFixed(2)}ms)`);
console.log(`  Speedup: ${(pinoRsChildBench.opsPerSec / pinoJsChildBench.opsPerSec).toFixed(2)}x\n`);

// Benchmark 4: Different log levels
console.log('4. Mixed log levels');
let levelIndex = 0;
const levels = ['trace', 'debug', 'info', 'warn', 'error', 'fatal'];
const pinoJsMixed = benchmark('pino.js mixed', () => {
  const level = levels[levelIndex++ % levels.length];
  pinoJsLogger[level]('log message');
});
levelIndex = 0;
const pinoRsMixed = benchmark('pino-rs mixed', () => {
  const level = levels[levelIndex++ % levels.length];
  pinoRsLogger[level]('log message');
});
benchmarks.push({
  name: 'Mixed levels',
  pinoJs: pinoJsMixed,
  pinoRs: pinoRsMixed,
  speedup: pinoRsMixed.opsPerSec / pinoJsMixed.opsPerSec
});
console.log(`  pino.js: ${pinoJsMixed.opsPerSec.toLocaleString()} ops/sec (${pinoJsMixed.durationMs.toFixed(2)}ms)`);
console.log(`  pino-rs: ${pinoRsMixed.opsPerSec.toLocaleString()} ops/sec (${pinoRsMixed.durationMs.toFixed(2)}ms)`);
console.log(`  Speedup: ${(pinoRsMixed.opsPerSec / pinoJsMixed.opsPerSec).toFixed(2)}x\n`);

// Benchmark 5: Complex objects
console.log('5. Complex nested objects');
const complexObj = {
  user: { id: 42, name: 'John Doe', email: 'john@example.com' },
  request: { method: 'POST', path: '/api/users', headers: { 'content-type': 'application/json' } },
  response: { status: 200, duration: 45.2 },
  metadata: { timestamp: Date.now(), requestId: 'abc-123-def-456' }
};
const pinoJsComplex = benchmark('pino.js complex', () => {
  pinoJsLogger.info(complexObj, 'request completed');
});
const pinoRsComplex = benchmark('pino-rs complex', () => {
  pinoRsLogger.info(complexObj, 'request completed');
});
benchmarks.push({
  name: 'Complex objects',
  pinoJs: pinoJsComplex,
  pinoRs: pinoRsComplex,
  speedup: pinoRsComplex.opsPerSec / pinoJsComplex.opsPerSec
});
console.log(`  pino.js: ${pinoJsComplex.opsPerSec.toLocaleString()} ops/sec (${pinoJsComplex.durationMs.toFixed(2)}ms)`);
console.log(`  pino-rs: ${pinoRsComplex.opsPerSec.toLocaleString()} ops/sec (${pinoRsComplex.durationMs.toFixed(2)}ms)`);
console.log(`  Speedup: ${(pinoRsComplex.opsPerSec / pinoJsComplex.opsPerSec).toFixed(2)}x\n`);

// Summary table
console.log('=== Summary ===\n');
console.log('┌─────────────────────┬──────────────────┬──────────────────┬──────────┐');
console.log('│ Benchmark           │ pino.js (ops/s)  │ pino-rs (ops/s)  │ Speedup  │');
console.log('├─────────────────────┼──────────────────┼──────────────────┼──────────┤');
benchmarks.forEach(b => {
  const name = b.name.padEnd(19);
  const pinoJsOps = b.pinoJs.opsPerSec.toLocaleString().padStart(16);
  const pinoRsOps = b.pinoRs.opsPerSec.toLocaleString().padStart(16);
  const speedup = `${b.speedup.toFixed(2)}x`.padStart(8);
  console.log(`│ ${name} │ ${pinoJsOps} │ ${pinoRsOps} │ ${speedup} │`);
});
console.log('└─────────────────────┴──────────────────┴──────────────────┴──────────┘');

// Overall average
const avgSpeedup = benchmarks.reduce((sum, b) => sum + b.speedup, 0) / benchmarks.length;
console.log(`\nAverage speedup: ${avgSpeedup.toFixed(2)}x faster than pino.js`);

// Restore console
process.stdout.write = originalStdoutWrite;
console.log = originalConsoleLog;
