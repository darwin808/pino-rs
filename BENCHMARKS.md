# Benchmark Results

## Summary

Current benchmarks comparing pino.js vs pino-rs (via NAPI bindings):

| Benchmark | pino.js (ops/sec) | pino-rs (ops/sec) | Ratio |
|-----------|-------------------|-------------------|-------|
| Simple logging | 1,723,394 | 792,623 | 0.46x |
| With fields | 1,095,263 | 361,484 | 0.33x |
| Child logger | 1,849,591 | 692,328 | 0.37x |
| Mixed levels | 3,110,307 | 1,084,956 | 0.35x |
| Complex objects | 585,077 | 324,339 | 0.55x |
| **Average** | **1,673,126** | **651,146** | **0.41x** |

**pino-rs is currently ~2.4x slower than pino.js** when used from Node.js.

## Why is pino-rs Slower?

### 1. NAPI Overhead
Every log call crosses the JavaScript ↔ Rust boundary via NAPI, which adds significant overhead:
- Argument marshalling (converting JS objects to Rust types)
- Type checking and validation
- Memory allocation for cross-boundary data
- Function call overhead

### 2. Synchronous I/O
- **pino.js**: Uses buffered streams with async I/O
- **pino-rs**: Currently uses synchronous stdout writes with mutex locking

### 3. Maturity
- **pino.js**: Years of performance optimizations
- **pino-rs**: Initial implementation focusing on correctness

### 4. Benchmark Setup
The comparison isn't entirely fair:
- pino.js writes to `/dev/null` (instant, buffered)
- pino-rs writes to actual stdout (slower, synchronized)

## Performance Roadmap

### Short Term (v0.2)
- [ ] Implement buffered writes
- [ ] Reduce NAPI marshalling overhead
- [ ] Pre-allocate common strings

### Medium Term (v0.3-v0.5)
- [ ] Async I/O using tokio
- [ ] Lock-free data structures
- [ ] Batch write optimization
- [ ] SIMD JSON serialization

### Long Term (v1.0+)
- [ ] Zero-copy serialization
- [ ] Thread-local buffers
- [ ] Direct memory mapped I/O
- [ ] Custom allocator optimizations

**Target**: 3-5x faster than pino.js for native Rust usage (no NAPI overhead)

## Current Recommendation

### Use pino.js if:
- You're writing a Node.js application
- You need maximum logging performance
- You're already using the pino ecosystem

### Use pino-rs if:
- You're writing a Rust application
- You want pino.js-compatible JSON output
- You value type safety and memory safety
- You're okay with current performance for MVP

## Running Benchmarks

```bash
cd pino-node

# Install dependencies
npm install
npm install pino

# Run benchmarks
node benchmark.js 2>/dev/null | tail -20

# Run pino.js baseline only
node benchmark-fair.js
```

## Hardware

Benchmarks run on:
- macOS (Darwin 25.1.0)
- Apple Silicon (ARM64)
- Node.js v23.7.0
- Rust 1.x (stable)

## Conclusion

pino-rs is a **correctness-first** implementation that prioritizes:
1. API compatibility with pino.js
2. Type safety and memory safety
3. Foundation for future optimizations

Performance will improve significantly in future releases, especially for native Rust usage where NAPI overhead is eliminated.
