// Simple test to verify pino-rs works like pino.js
const pino = require('./index');

console.log('=== Pino-RS Node.js Bindings Test ===\n');

// Test 1: Basic logger
console.log('Test 1: Basic logger');
const logger = pino();
logger.info('hello world');

// Test 2: Logger with options
console.log('\nTest 2: Logger with custom level');
const debugLogger = pino({ level: 'debug' });
debugLogger.debug('debug message');
debugLogger.trace('trace message - should not appear');

// Test 3: Logger with base fields
console.log('\nTest 3: Logger with base fields');
const appLogger = pino({
  base: {
    app: 'my-app',
    version: '1.0.0'
  }
});
appLogger.info('application started');

// Test 4: Child logger
console.log('\nTest 4: Child logger');
const childLogger = appLogger.child({ module: 'auth', requestId: 'abc-123' });
childLogger.info('user logged in');

// Test 5: Logging with objects
console.log('\nTest 5: Logging with objects');
logger.info({ userId: 42, email: 'user@example.com' }, 'user data');

// Test 6: All log levels
console.log('\nTest 6: All log levels');
const allLevels = pino({ level: 'trace' });
allLevels.trace('trace level');
allLevels.debug('debug level');
allLevels.info('info level');
allLevels.warn('warn level');
allLevels.error('error level');
allLevels.fatal('fatal level');

console.log('\n=== All tests completed ===');
