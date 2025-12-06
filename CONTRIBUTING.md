# Contributing to pino-rs

Thank you for your interest in contributing to pino-rs!

## Development Setup

1. Install Rust (1.70 or later):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Node.js (v10 or later) for Node.js bindings development

3. Clone the repository:
```bash
git clone https://github.com/yourusername/pino-rs
cd pino-rs
```

4. Build the project:
```bash
cargo build
```

## Project Structure

- `pino-core/` - Core Rust logging library
  - `src/level.rs` - Log level definitions
  - `src/logger.rs` - Main logger implementation
  - `src/record.rs` - Log record structure
  - `src/transport.rs` - Async transport implementations
  - `src/tests.rs` - Unit tests

- `pino-node/` - Node.js bindings
  - `src/lib.rs` - NAPI bindings implementation
  - `index.js` - JavaScript wrapper
  - `index.d.ts` - TypeScript definitions
  - `test.js` - Node.js tests

- `src/main.rs` - Rust example/demo

## Running Tests

### Rust tests
```bash
cargo test
```

### Node.js tests
```bash
cd pino-node
npm test
```

## Building Node.js Bindings

```bash
cd pino-node
npm install
npm run build
```

## Code Style

- Follow Rust standard formatting: `cargo fmt`
- Check for common issues: `cargo clippy`
- Write tests for new features
- Document public APIs

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Run `cargo fmt` and `cargo clippy`
7. Submit a pull request

## Compatibility Guidelines

Since pino-rs aims to be a drop-in replacement for pino.js:

- Maintain API compatibility with pino.js
- Match pino.js JSON output format
- Support the same log levels and semantics
- Document any intentional differences

## Questions?

Feel free to open an issue for questions or discussion!
