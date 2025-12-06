use crate::{Level, LogRecord, Fields};
use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::Arc;
use parking_lot::RwLock;

pub struct Logger {
    min_level: Level,
    base_fields: Fields,
    writer: Arc<RwLock<Box<dyn Write + Send + Sync>>>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            min_level: Level::Info,
            base_fields: HashMap::new(),
            writer: Arc::new(RwLock::new(Box::new(io::stdout()))),
        }
    }

    pub fn builder() -> LoggerBuilder {
        LoggerBuilder::default()
    }

    pub fn child(&self, fields: Fields) -> Self {
        let mut child_fields = self.base_fields.clone();
        child_fields.extend(fields);

        Self {
            min_level: self.min_level,
            base_fields: child_fields,
            writer: Arc::clone(&self.writer),
        }
    }

    pub fn log(&self, level: Level, msg: Option<&str>, fields: Option<Fields>) {
        if level < self.min_level {
            return;
        }

        let mut record = LogRecord::new(level, msg.map(|s| s.to_string()));

        // Add base fields first
        for (k, v) in &self.base_fields {
            record.add_field(k.clone(), v.clone());
        }

        // Add custom fields
        if let Some(custom_fields) = fields {
            for (k, v) in custom_fields {
                record.add_field(k, v);
            }
        }

        if let Ok(json) = record.to_json() {
            let mut writer = self.writer.write();
            let _ = writeln!(writer, "{}", json);
            let _ = writer.flush();
        }
    }

    pub fn trace(&self, msg: &str) {
        self.log(Level::Trace, Some(msg), None);
    }

    pub fn trace_with_fields(&self, msg: &str, fields: Fields) {
        self.log(Level::Trace, Some(msg), Some(fields));
    }

    pub fn debug(&self, msg: &str) {
        self.log(Level::Debug, Some(msg), None);
    }

    pub fn debug_with_fields(&self, msg: &str, fields: Fields) {
        self.log(Level::Debug, Some(msg), Some(fields));
    }

    pub fn info(&self, msg: &str) {
        self.log(Level::Info, Some(msg), None);
    }

    pub fn info_with_fields(&self, msg: &str, fields: Fields) {
        self.log(Level::Info, Some(msg), Some(fields));
    }

    pub fn warn(&self, msg: &str) {
        self.log(Level::Warn, Some(msg), None);
    }

    pub fn warn_with_fields(&self, msg: &str, fields: Fields) {
        self.log(Level::Warn, Some(msg), Some(fields));
    }

    pub fn error(&self, msg: &str) {
        self.log(Level::Error, Some(msg), None);
    }

    pub fn error_with_fields(&self, msg: &str, fields: Fields) {
        self.log(Level::Error, Some(msg), Some(fields));
    }

    pub fn fatal(&self, msg: &str) {
        self.log(Level::Fatal, Some(msg), None);
    }

    pub fn fatal_with_fields(&self, msg: &str, fields: Fields) {
        self.log(Level::Fatal, Some(msg), Some(fields));
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Self {
            min_level: self.min_level,
            base_fields: self.base_fields.clone(),
            writer: Arc::clone(&self.writer),
        }
    }
}

pub struct LoggerBuilder {
    min_level: Level,
    base_fields: Fields,
    writer: Option<Box<dyn Write + Send + Sync>>,
}

impl LoggerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn level(mut self, level: Level) -> Self {
        self.min_level = level;
        self
    }

    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.base_fields.insert(key.into(), value.into());
        self
    }

    pub fn with_fields(mut self, fields: Fields) -> Self {
        self.base_fields.extend(fields);
        self
    }

    pub fn writer(mut self, writer: Box<dyn Write + Send + Sync>) -> Self {
        self.writer = Some(writer);
        self
    }

    pub fn build(self) -> Logger {
        Logger {
            min_level: self.min_level,
            base_fields: self.base_fields,
            writer: Arc::new(RwLock::new(
                self.writer.unwrap_or_else(|| Box::new(io::stdout()))
            )),
        }
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self {
            min_level: Level::Info,
            base_fields: HashMap::new(),
            writer: None,
        }
    }
}
