use pino_core::{Logger, LoggerBuilder, Level};
use serde_json::json;
use std::collections::HashMap;

fn main() {
    println!("=== Pino-RS Demo ===\n");

    // Basic logger
    println!("1. Basic logging:");
    let logger = Logger::new();
    logger.info("Hello from pino-rs!");
    logger.warn("This is a warning");
    logger.error("This is an error");

    println!("\n2. Logger with custom level:");
    let debug_logger = LoggerBuilder::new()
        .level(Level::Debug)
        .build();
    debug_logger.debug("Debug message - visible");
    debug_logger.trace("Trace message - not visible (below min level)");

    println!("\n3. Logger with base fields:");
    let app_logger = LoggerBuilder::new()
        .with_field("app", "pino-rs-demo")
        .with_field("version", "0.1.0")
        .build();
    app_logger.info("Application started");

    println!("\n4. Child logger (inherits parent fields):");
    let mut child_fields = HashMap::new();
    child_fields.insert("module".to_string(), json!("auth"));
    child_fields.insert("user_id".to_string(), json!(12345));

    let child_logger = app_logger.child(child_fields);
    child_logger.info("User authenticated");

    println!("\n5. Logging with additional fields:");
    let mut extra_fields = HashMap::new();
    extra_fields.insert("request_id".to_string(), json!("req-abc-123"));
    extra_fields.insert("duration_ms".to_string(), json!(42));

    logger.info_with_fields("Request completed", extra_fields);

    println!("\n6. All log levels:");
    let trace_logger = LoggerBuilder::new()
        .level(Level::Trace)
        .build();

    trace_logger.trace("Trace level");
    trace_logger.debug("Debug level");
    trace_logger.info("Info level");
    trace_logger.warn("Warn level");
    trace_logger.error("Error level");
    trace_logger.fatal("Fatal level");

    println!("\n=== Demo Complete ===");
}
