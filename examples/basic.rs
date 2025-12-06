use pino_core::{Logger, LoggerBuilder, Level};

fn main() {
    // Create a basic logger with default settings
    let logger = Logger::new();

    // Log messages at different levels
    logger.info("Application starting");
    logger.warn("This is a warning");
    logger.error("An error occurred");

    // Create a logger with trace level enabled
    let trace_logger = LoggerBuilder::new()
        .level(Level::Trace)
        .build();

    trace_logger.trace("Detailed trace information");
    trace_logger.debug("Debug information");
    trace_logger.info("Informational message");
}
