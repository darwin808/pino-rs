#[cfg(test)]
mod tests {
    use crate::{Logger, LoggerBuilder, Level};
    use serde_json::json;
    use std::collections::HashMap;
    use std::io::Cursor;
    use std::sync::Arc;
    use parking_lot::RwLock;

    fn create_test_logger() -> (Logger, Arc<RwLock<Cursor<Vec<u8>>>>) {
        let buffer = Arc::new(RwLock::new(Cursor::new(Vec::new())));
        let buffer_clone = Arc::clone(&buffer);

        let logger = LoggerBuilder::new()
            .level(Level::Trace)
            .writer(Box::new(Cursor::new(Vec::new())))
            .build();

        (logger, buffer_clone)
    }

    #[test]
    fn test_log_levels() {
        let logger = Logger::new();

        // Should not panic
        logger.trace("trace");
        logger.debug("debug");
        logger.info("info");
        logger.warn("warn");
        logger.error("error");
        logger.fatal("fatal");
    }

    #[test]
    fn test_log_level_filtering() {
        let logger = LoggerBuilder::new()
            .level(Level::Warn)
            .build();

        // Below min level - should not log (but shouldn't panic)
        logger.trace("trace");
        logger.debug("debug");
        logger.info("info");

        // Above min level - should log
        logger.warn("warn");
        logger.error("error");
        logger.fatal("fatal");
    }

    #[test]
    fn test_child_logger() {
        let parent = LoggerBuilder::new()
            .with_field("app", "test-app")
            .build();

        let mut child_fields = HashMap::new();
        child_fields.insert("module".to_string(), json!("auth"));

        let child = parent.child(child_fields);

        // Child should inherit parent fields
        child.info("test message");
    }

    #[test]
    fn test_logger_builder() {
        let logger = LoggerBuilder::new()
            .level(Level::Debug)
            .with_field("env", "test")
            .with_field("version", "1.0.0")
            .build();

        logger.debug("test message");
    }

    #[test]
    fn test_log_with_fields() {
        let logger = Logger::new();

        let mut fields = HashMap::new();
        fields.insert("user_id".to_string(), json!(42));
        fields.insert("action".to_string(), json!("login"));

        logger.info_with_fields("user action", fields);
    }

    #[test]
    fn test_multiple_child_loggers() {
        let parent = LoggerBuilder::new()
            .with_field("app", "test-app")
            .build();

        let mut fields1 = HashMap::new();
        fields1.insert("module".to_string(), json!("auth"));

        let mut fields2 = HashMap::new();
        fields2.insert("module".to_string(), json!("db"));

        let child1 = parent.child(fields1);
        let child2 = parent.child(fields2);

        child1.info("auth message");
        child2.info("db message");
    }

    #[test]
    fn test_logger_clone() {
        let logger1 = Logger::new();
        let logger2 = logger1.clone();

        logger1.info("from logger1");
        logger2.info("from logger2");
    }

    #[test]
    fn test_level_values() {
        assert_eq!(Level::Trace.as_u8(), 10);
        assert_eq!(Level::Debug.as_u8(), 20);
        assert_eq!(Level::Info.as_u8(), 30);
        assert_eq!(Level::Warn.as_u8(), 40);
        assert_eq!(Level::Error.as_u8(), 50);
        assert_eq!(Level::Fatal.as_u8(), 60);
    }

    #[test]
    fn test_level_ordering() {
        assert!(Level::Trace < Level::Debug);
        assert!(Level::Debug < Level::Info);
        assert!(Level::Info < Level::Warn);
        assert!(Level::Warn < Level::Error);
        assert!(Level::Error < Level::Fatal);
    }

    #[test]
    fn test_level_display() {
        assert_eq!(Level::Trace.to_string(), "trace");
        assert_eq!(Level::Debug.to_string(), "debug");
        assert_eq!(Level::Info.to_string(), "info");
        assert_eq!(Level::Warn.to_string(), "warn");
        assert_eq!(Level::Error.to_string(), "error");
        assert_eq!(Level::Fatal.to_string(), "fatal");
    }
}
