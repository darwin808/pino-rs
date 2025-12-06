use pino_core::Logger;
use serde_json::json;
use std::collections::HashMap;

fn main() {
    let logger = Logger::new();

    // Log with additional structured fields
    let mut user_fields = HashMap::new();
    user_fields.insert("user_id".to_string(), json!(42));
    user_fields.insert("username".to_string(), json!("john_doe"));
    user_fields.insert("email".to_string(), json!("john@example.com"));

    logger.info_with_fields("User registered", user_fields);

    // Log HTTP request
    let mut request_fields = HashMap::new();
    request_fields.insert("method".to_string(), json!("GET"));
    request_fields.insert("path".to_string(), json!("/api/users/42"));
    request_fields.insert("status".to_string(), json!(200));
    request_fields.insert("duration_ms".to_string(), json!(45));
    request_fields.insert("ip".to_string(), json!("192.168.1.100"));

    logger.info_with_fields("HTTP request completed", request_fields);

    // Log error with context
    let mut error_fields = HashMap::new();
    error_fields.insert("error_code".to_string(), json!("DB_CONNECTION_FAILED"));
    error_fields.insert("retry_count".to_string(), json!(3));
    error_fields.insert("last_error".to_string(), json!("Connection timeout"));

    logger.error_with_fields("Database operation failed", error_fields);

    // Log performance metrics
    let mut perf_fields = HashMap::new();
    perf_fields.insert("cpu_usage".to_string(), json!(75.5));
    perf_fields.insert("memory_mb".to_string(), json!(512));
    perf_fields.insert("active_connections".to_string(), json!(150));

    logger.info_with_fields("Performance metrics", perf_fields);
}
