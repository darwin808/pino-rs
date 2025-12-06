const { pino, Logger } = require('./index.node');

// Export pino function as default and named export for compatibility
module.exports = pino;
module.exports.pino = pino;
module.exports.Logger = Logger;
module.exports.default = pino;

// Pino-compatible log level constants
module.exports.levels = {
  values: {
    trace: 10,
    debug: 20,
    info: 30,
    warn: 40,
    error: 50,
    fatal: 60
  },
  labels: {
    10: 'trace',
    20: 'debug',
    30: 'info',
    40: 'warn',
    50: 'error',
    60: 'fatal'
  }
};
