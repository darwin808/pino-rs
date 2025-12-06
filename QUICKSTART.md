# Quick Start Guide

## For Rust Developers

### 1. Add dependency to your Cargo.toml

```toml
[dependencies]
pino-core = { path = "path/to/pino-rs/pino-core" }
serde_json = "1.0"
```

### 2. Use in your code

```rust
use pino_core::{Logger, LoggerBuilder, Level};

fn main() {
    // Simple usage
    let logger = Logger::new();
    logger.info("Hello from pino-rs!");

    // With custom level
    let logger = LoggerBuilder::new()
        .level(Level::Debug)
        .build();

    logger.debug("Debug message");
}
```

### 3. Run examples

```bash
cargo run --example basic
cargo run --example child_logger
cargo run --example structured_logging
```

## For Node.js Developers

### 1. Build the Node.js bindings

```bash
cd pino-node
npm install
npm run build
```

### 2. Use in your JavaScript code

```javascript
const pino = require('./pino-node');

const logger = pino();
logger.info('hello world');

// With options
const logger = pino({
  level: 'debug',
  base: { app: 'my-app' }
});

logger.debug('debug message');

// Child logger
const child = logger.child({ requestId: '123' });
child.info('request handled');
```

### 3. Run the test

```bash
cd pino-node
node test.js
```

## Differences from pino.js

While pino-rs aims to be API-compatible with pino.js, there are some current limitations:

- **Not yet implemented**: Transports, Pretty printing, Redaction
- **Performance**: Much faster due to Rust implementation
- **Installation**: Currently requires building from source

## Output Format

Both Rust and Node.js versions produce the same JSON format:

```json
{
  "level": 30,
  "time": 1531171074631,
  "pid": 657,
  "hostname": "server-01",
  "msg": "hello world"
}
```

## Log Levels

| Level | Value | Method |
|-------|-------|--------|
| trace | 10 | `logger.trace()` |
| debug | 20 | `logger.debug()` |
| info | 30 | `logger.info()` |
| warn | 40 | `logger.warn()` |
| error | 50 | `logger.error()` |
| fatal | 60 | `logger.fatal()` |

## Next Steps

- Read the full [README.md](README.md)
- Check out [CONTRIBUTING.md](CONTRIBUTING.md) to contribute
- Run the test suite: `cargo test`
- Explore the examples in the `examples/` directory
