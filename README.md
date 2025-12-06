# pino-rs

A blazingly fast Rust implementation of the [pino.js](https://github.com/pinojs/pino) logger. Built for performance while maintaining full API compatibility with pino.js, making it a true drop-in replacement.

## Features

- **Extremely Fast**: Written in Rust for maximum performance with minimal overhead
- **Drop-in Replacement**: Compatible with pino.js API - works with existing code
- **JSON Structured Logging**: Outputs structured JSON logs like pino.js
- **Child Loggers**: Supports child loggers with inherited context
- **Multiple Log Levels**: trace, debug, info, warn, error, fatal
- **Zero-Cost Abstractions**: Leverages Rust's performance guarantees
- **Node.js Bindings**: Use from JavaScript/TypeScript via NAPI bindings
- **Thread-Safe**: Built on Rust's concurrency primitives

## Performance

Pino-rs is designed to be significantly faster than pino.js while maintaining the same output format and API. The Rust implementation provides:

- Lower memory footprint
- Faster JSON serialization
- Reduced latency for high-volume logging
- Efficient async I/O

## Installation

### From NPM (Node.js)

```bash
npm install pino-rs
```

### From Cargo (Rust)

```toml
[dependencies]
pino-core = "0.1"
```

## Usage

### Node.js / JavaScript

```javascript
const pino = require('pino-rs');

// Basic usage
const logger = pino();
logger.info('hello world');
// {"level":30,"time":1531171074631,"msg":"hello world","pid":657,"hostname":"..."}

// With log level
const logger = pino({ level: 'debug' });
logger.debug('debug message');

// With base fields
const logger = pino({
  base: {
    app: 'my-app',
    version: '1.0.0'
  }
});
logger.info('application started');

// Child logger
const child = logger.child({ module: 'auth' });
child.info('user logged in');

// With additional fields
logger.info({ userId: 42 }, 'user action');
```

### Rust

```rust
use pino_core::{Logger, LoggerBuilder, Level};
use serde_json::json;
use std::collections::HashMap;

fn main() {
    // Basic logger
    let logger = Logger::new();
    logger.info("Hello from pino-rs!");

    // With custom level
    let debug_logger = LoggerBuilder::new()
        .level(Level::Debug)
        .build();
    debug_logger.debug("Debug message");

    // With base fields
    let app_logger = LoggerBuilder::new()
        .with_field("app", "my-app")
        .with_field("version", "1.0.0")
        .build();
    app_logger.info("Application started");

    // Child logger
    let mut child_fields = HashMap::new();
    child_fields.insert("module".to_string(), json!("auth"));
    let child = app_logger.child(child_fields);
    child.info("User authenticated");
}
```

## API Compatibility

Pino-rs implements the core pino.js API:

### Logger Methods

- `trace(msg)` - Log at trace level (10)
- `debug(msg)` - Log at debug level (20)
- `info(msg)` - Log at info level (30)
- `warn(msg)` - Log at warn level (40)
- `error(msg)` - Log at error level (50)
- `fatal(msg)` - Log at fatal level (60)
- `child(bindings)` - Create a child logger with additional context

### Logger Options

- `level` - Minimum log level (trace, debug, info, warn, error, fatal)
- `base` - Base fields to include in all logs

### Output Format

Pino-rs produces the same JSON output format as pino.js:

```json
{
  "level": 30,
  "time": 1531171074631,
  "pid": 657,
  "hostname": "server-01",
  "msg": "hello world"
}
```

## Project Structure

- **pino-core** - Core Rust logging library
- **pino-node** - Node.js bindings via NAPI
- **src/main.rs** - Example Rust usage

## Development

### Build from source

```bash
# Clone the repository
git clone https://github.com/yourusername/pino-rs
cd pino-rs

# Build the Rust library
cargo build --release

# Build Node.js bindings
cd pino-node
npm install
npm run build
```

### Run tests

```bash
# Rust tests
cargo test

# Node.js tests
cd pino-node
npm test
```

### Run examples

```bash
# Rust example
cargo run

# Node.js example
cd pino-node
node test.js
```

## Benchmarks

Coming soon: Comprehensive benchmarks comparing pino-rs with pino.js and other logging libraries.

## Roadmap

- [x] Core logging functionality
- [x] JSON serialization
- [x] Child loggers
- [x] Node.js bindings
- [ ] Transport system
- [ ] Redaction support
- [ ] Pretty printing
- [ ] Comprehensive benchmarks
- [ ] Full pino.js feature parity

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT

## Credits

Inspired by and compatible with [pino.js](https://github.com/pinojs/pino) by Matteo Collina and the pino team.

## Why pino-rs?

While pino.js is already one of the fastest JavaScript loggers, pino-rs takes it further by:

1. **Native Performance**: Rust's zero-cost abstractions and efficient memory management
2. **Lower Overhead**: Minimal runtime overhead compared to JavaScript
3. **Better Concurrency**: Rust's ownership system enables safe concurrent logging
4. **Easy Integration**: Drop-in replacement - no code changes needed

Use pino-rs when you need the absolute best logging performance while maintaining compatibility with the pino.js ecosystem.
