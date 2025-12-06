use pino_core::LoggerBuilder;
use serde_json::json;
use std::collections::HashMap;

fn main() {
    // Create a parent logger with base fields
    let app_logger = LoggerBuilder::new()
        .with_field("app", "my-application")
        .with_field("version", "1.0.0")
        .build();

    app_logger.info("Application initialized");

    // Create child logger for authentication module
    let mut auth_fields = HashMap::new();
    auth_fields.insert("module".to_string(), json!("authentication"));
    let auth_logger = app_logger.child(auth_fields);

    auth_logger.info("User login attempt");
    auth_logger.info("User authenticated successfully");

    // Create child logger for database module
    let mut db_fields = HashMap::new();
    db_fields.insert("module".to_string(), json!("database"));
    let db_logger = app_logger.child(db_fields);

    db_logger.info("Database connection established");
    db_logger.warn("Slow query detected");

    // Child loggers can have their own children
    let mut query_fields = HashMap::new();
    query_fields.insert("query_id".to_string(), json!("q-12345"));
    let query_logger = db_logger.child(query_fields);

    query_logger.debug("Executing query");
}
