use crate::{Level, Fields};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRecord {
    #[serde(rename = "level")]
    pub level: u8,

    #[serde(rename = "time")]
    pub time: i64,

    #[serde(rename = "pid")]
    pub pid: u32,

    #[serde(rename = "hostname")]
    pub hostname: String,

    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,

    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

impl LogRecord {
    pub fn new(level: Level, msg: Option<String>) -> Self {
        Self {
            level: level.as_u8(),
            time: chrono::Utc::now().timestamp_millis(),
            pid: std::process::id(),
            hostname: hostname::get()
                .ok()
                .and_then(|h| h.into_string().ok())
                .unwrap_or_else(|| "unknown".to_string()),
            msg,
            fields: HashMap::new(),
        }
    }

    pub fn with_fields(mut self, fields: Fields) -> Self {
        self.fields.extend(fields);
        self
    }

    pub fn add_field(&mut self, key: String, value: Value) {
        self.fields.insert(key, value);
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
