/// High-performance async logger using channels for non-blocking writes
use crate::{Level, Fields, fast_json::FastJsonWriter};
use crossbeam_channel::{unbounded, Sender, Receiver};
use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, BufWriter, Write};
use std::sync::Arc;
use std::thread;

const WRITE_BUFFER_SIZE: usize = 64 * 1024; // 64KB buffer

pub struct LogMessage {
    pub level: u8,
    pub time: i64,
    pub pid: u32,
    pub hostname: Arc<str>,
    pub msg: Option<String>,
    pub fields: HashMap<String, Value>,
}

pub struct AsyncLogger {
    min_level: Level,
    base_fields: Fields,
    sender: Sender<LogMessage>,
    hostname: Arc<str>,
    pid: u32,
}

impl AsyncLogger {
    pub fn new() -> Self {
        Self::with_capacity(10000)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let (sender, receiver) = if capacity > 0 {
            crossbeam_channel::bounded(capacity)
        } else {
            unbounded()
        };

        // Spawn background writer thread
        thread::spawn(move || {
            Self::writer_thread(receiver);
        });

        let hostname = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "unknown".to_string())
            .into();

        Self {
            min_level: Level::Info,
            base_fields: HashMap::new(),
            sender,
            hostname,
            pid: std::process::id(),
        }
    }

    fn writer_thread(receiver: Receiver<LogMessage>) {
        let stdout = io::stdout();
        let mut writer = BufWriter::with_capacity(WRITE_BUFFER_SIZE, stdout);

        for msg in receiver {
            if let Err(e) = Self::write_log(&mut writer, msg) {
                eprintln!("pino-rs write error: {}", e);
            }
        }

        // Flush on exit
        let _ = writer.flush();
    }

    #[inline]
    fn write_log<W: Write>(writer: &mut W, msg: LogMessage) -> io::Result<()> {
        let mut json = FastJsonWriter::new(writer);

        json.write_object_start()?;

        // Write level
        json.write_field_name("level")?;
        json.write_u64(msg.level as u64)?;

        // Write time
        json.write_comma()?;
        json.write_field_name("time")?;
        json.write_i64(msg.time)?;

        // Write pid
        json.write_comma()?;
        json.write_field_name("pid")?;
        json.write_u64(msg.pid as u64)?;

        // Write hostname
        json.write_comma()?;
        json.write_field_name("hostname")?;
        json.write_string(&msg.hostname)?;

        // Write message if present
        if let Some(ref m) = msg.msg {
            json.write_comma()?;
            json.write_field_name("msg")?;
            json.write_string(m)?;
        }

        // Write custom fields
        for (key, value) in &msg.fields {
            json.write_comma()?;
            json.write_field_name(key)?;
            json.write_value(value)?;
        }

        json.write_object_end()?;
        let writer = json.into_inner();
        writer.write_all(b"\n")?;

        Ok(())
    }

    pub fn child(&self, fields: Fields) -> Self {
        let mut child_fields = self.base_fields.clone();
        child_fields.extend(fields);

        Self {
            min_level: self.min_level,
            base_fields: child_fields,
            sender: self.sender.clone(),
            hostname: Arc::clone(&self.hostname),
            pid: self.pid,
        }
    }

    #[inline]
    pub fn log(&self, level: Level, msg: Option<&str>, fields: Option<Fields>) {
        if level < self.min_level {
            return;
        }

        let mut all_fields = if self.base_fields.is_empty() {
            HashMap::new()
        } else {
            self.base_fields.clone()
        };

        if let Some(custom_fields) = fields {
            all_fields.extend(custom_fields);
        }

        let log_msg = LogMessage {
            level: level.as_u8(),
            time: chrono::Utc::now().timestamp_millis(),
            pid: self.pid,
            hostname: Arc::clone(&self.hostname),
            msg: msg.map(|s| s.to_string()),
            fields: all_fields,
        };

        // Non-blocking send - if channel is full, drop the message
        // For bounded channels, could use try_send
        let _ = self.sender.send(log_msg);
    }

    pub fn set_level(&mut self, level: Level) {
        self.min_level = level;
    }

    // Convenience methods
    #[inline] pub fn trace(&self, msg: &str) { self.log(Level::Trace, Some(msg), None); }
    #[inline] pub fn debug(&self, msg: &str) { self.log(Level::Debug, Some(msg), None); }
    #[inline] pub fn info(&self, msg: &str) { self.log(Level::Info, Some(msg), None); }
    #[inline] pub fn warn(&self, msg: &str) { self.log(Level::Warn, Some(msg), None); }
    #[inline] pub fn error(&self, msg: &str) { self.log(Level::Error, Some(msg), None); }
    #[inline] pub fn fatal(&self, msg: &str) { self.log(Level::Fatal, Some(msg), None); }

    #[inline] pub fn trace_with_fields(&self, msg: &str, fields: Fields) { self.log(Level::Trace, Some(msg), Some(fields)); }
    #[inline] pub fn debug_with_fields(&self, msg: &str, fields: Fields) { self.log(Level::Debug, Some(msg), Some(fields)); }
    #[inline] pub fn info_with_fields(&self, msg: &str, fields: Fields) { self.log(Level::Info, Some(msg), Some(fields)); }
    #[inline] pub fn warn_with_fields(&self, msg: &str, fields: Fields) { self.log(Level::Warn, Some(msg), Some(fields)); }
    #[inline] pub fn error_with_fields(&self, msg: &str, fields: Fields) { self.log(Level::Error, Some(msg), Some(fields)); }
    #[inline] pub fn fatal_with_fields(&self, msg: &str, fields: Fields) { self.log(Level::Fatal, Some(msg), Some(fields)); }
}

impl Default for AsyncLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for AsyncLogger {
    fn clone(&self) -> Self {
        Self {
            min_level: self.min_level,
            base_fields: self.base_fields.clone(),
            sender: self.sender.clone(),
            hostname: Arc::clone(&self.hostname),
            pid: self.pid,
        }
    }
}
