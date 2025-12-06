const fs = require('fs');

// Suppress output
const nullStream = fs.createWriteStream('/dev/null');

const pinoJs = require('pino');
const pinoRs = require('./index');

const pinoJsLogger = pinoJs(nullStream);
const pinoRsLogger = pinoRs();

const ITERATIONS = 100000;

console.error('=== Pino-RS vs Pino.js Benchmark ===\n');
console.error(`Iterations: ${ITERATIONS.toLocaleString()}\n`);

// Test 1: Simple logging
console.error('1. Simple string logging');
let start = Date.now();
for (let i = 0; i < ITERATIONS; i++) {
  pinoJsLogger.info('hello world');
}
const pinoJsSimple = Date.now() - start;
console.error(`  pino.js: ${Math.round(ITERATIONS / (pinoJsSimple / 1000)).toLocaleString()} ops/sec (${pinoJsSimple}ms)`);

start = Date.now();
for (let i = 0; i < ITERATIONS; i++) {
  pinoRsLogger.info('hello world');
}
const pinoRsSimple = Date.now() - start;
console.error(`  pino-rs: ${Math.round(ITERATIONS / (pinoRsSimple / 1000)).toLocaleString()} ops/sec (${pinoRsSimple}ms)`);
console.error(`  Speedup: ${(pinoRsSimple / pinoJsSimple < 1 ? pinoJsSimple / pinoRsSimple : -(pinoRsSimple / pinoJsSimple)).toFixed(2)}x ${pinoRsSimple < pinoJsSimple ? 'faster' : 'slower'}\n`);

// Test 2: With fields
console.error('2. Logging with additional fields');
start = Date.now();
for (let i = 0; i < ITERATIONS; i++) {
  pinoJsLogger.info({ userId: 42, requestId: 'abc123' }, 'user action');
}
const pinoJsFields = Date.now() - start;
console.error(`  pino.js: ${Math.round(ITERATIONS / (pinoJsFields / 1000)).toLocaleString()} ops/sec (${pinoJsFields}ms)`);

start = Date.now();
for (let i = 0; i < ITERATIONS; i++) {
  pinoRsLogger.info({ userId: 42, requestId: 'abc123' }, 'user action');
}
const pinoRsFields = Date.now() - start;
console.error(`  pino-rs: ${Math.round(ITERATIONS / (pinoRsFields / 1000)).toLocaleString()} ops/sec (${pinoRsFields}ms)`);
console.error(`  Speedup: ${(pinoRsFields / pinoJsFields < 1 ? pinoJsFields / pinoRsFields : -(pinoRsFields / pinoJsFields)).toFixed(2)}x ${pinoRsFields < pinoJsFields ? 'faster' : 'slower'}\n`);

// Test 3: Child logger
console.error('3. Child logger');
const pinoJsChild = pinoJsLogger.child({ module: 'auth' });
const pinoRsChild = pinoRsLogger.child({ module: 'auth' });

start = Date.now();
for (let i = 0; i < ITERATIONS; i++) {
  pinoJsChild.info('authentication successful');
}
const pinoJsChildTime = Date.now() - start;
console.error(`  pino.js: ${Math.round(ITERATIONS / (pinoJsChildTime / 1000)).toLocaleString()} ops/sec (${pinoJsChildTime}ms)`);

start = Date.now();
for (let i = 0; i < ITERATIONS; i++) {
  pinoRsChild.info('authentication successful');
}
const pinoRsChildTime = Date.now() - start;
console.error(`  pino-rs: ${Math.round(ITERATIONS / (pinoRsChildTime / 1000)).toLocaleString()} ops/sec (${pinoRsChildTime}ms)`);
console.error(`  Speedup: ${(pinoRsChildTime / pinoJsChildTime < 1 ? pinoJsChildTime / pinoRsChildTime : -(pinoRsChildTime / pinoJsChildTime)).toFixed(2)}x ${pinoRsChildTime < pinoJsChildTime ? 'faster' : 'slower'}\n`);

// Test 4: Mixed levels
console.error('4. Mixed log levels');
start = Date.now();
for (let i = 0; i < ITERATIONS; i++) {
  if (i % 4 === 0) pinoJsLogger.trace('trace');
  else if (i % 4 === 1) pinoJsLogger.debug('debug');
  else if (i % 4 === 2) pinoJsLogger.info('info');
  else pinoJsLogger.warn('warn');
}
const pinoJsMixed = Date.now() - start;
console.error(`  pino.js: ${Math.round(ITERATIONS / (pinoJsMixed / 1000)).toLocaleString()} ops/sec (${pinoJsMixed}ms)`);

start = Date.now();
for (let i = 0; i < ITERATIONS; i++) {
  if (i % 4 === 0) pinoRsLogger.trace('trace');
  else if (i % 4 === 1) pinoRsLogger.debug('debug');
  else if (i % 4 === 2) pinoRsLogger.info('info');
  else pinoRsLogger.warn('warn');
}
const pinoRsMixed = Date.now() - start;
console.error(`  pino-rs: ${Math.round(ITERATIONS / (pinoRsMixed / 1000)).toLocaleString()} ops/sec (${pinoRsMixed}ms)`);
console.error(`  Speedup: ${(pinoRsMixed / pinoJsMixed < 1 ? pinoJsMixed / pinoRsMixed : -(pinoRsMixed / pinoJsMixed)).toFixed(2)}x ${pinoRsMixed < pinoJsMixed ? 'faster' : 'slower'}\n`);

// Summary
const avgPinoJs = (pinoJsSimple + pinoJsFields + pinoJsChildTime + pinoJsMixed) / 4;
const avgPinoRs = (pinoRsSimple + pinoRsFields + pinoRsChildTime + pinoRsMixed) / 4;
console.error('=== Summary ===');
console.error(`Average: ${(avgPinoRs < avgPinoJs ? avgPinoJs / avgPinoRs : -(avgPinoRs / avgPinoJs)).toFixed(2)}x ${avgPinoRs < avgPinoJs ? 'faster' : 'slower'} than pino.js`);
