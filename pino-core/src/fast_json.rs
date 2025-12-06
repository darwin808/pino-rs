/// Ultra-fast JSON serializer optimized for log output
/// Avoids serde_json overhead by writing directly to a buffer

use std::io::{self, Write};
use serde_json::Value;

pub struct FastJsonWriter<W: Write> {
    writer: W,
}

impl<W: Write> FastJsonWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    #[inline]
    pub fn write_object_start(&mut self) -> io::Result<()> {
        self.writer.write_all(b"{")
    }

    #[inline]
    pub fn write_object_end(&mut self) -> io::Result<()> {
        self.writer.write_all(b"}")
    }

    #[inline]
    pub fn write_comma(&mut self) -> io::Result<()> {
        self.writer.write_all(b",")
    }

    #[inline]
    pub fn write_field_name(&mut self, name: &str) -> io::Result<()> {
        self.writer.write_all(b"\"")?;
        self.writer.write_all(name.as_bytes())?;
        self.writer.write_all(b"\":")
    }

    #[inline]
    pub fn write_string(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_all(b"\"")?;
        // Fast path: no escaping needed for most log messages
        if s.bytes().all(|b| b >= 32 && b != b'"' && b != b'\\') {
            self.writer.write_all(s.as_bytes())?;
        } else {
            // Slow path: escape special characters
            for ch in s.chars() {
                match ch {
                    '"' => self.writer.write_all(b"\\\"")?,
                    '\\' => self.writer.write_all(b"\\\\")?,
                    '\n' => self.writer.write_all(b"\\n")?,
                    '\r' => self.writer.write_all(b"\\r")?,
                    '\t' => self.writer.write_all(b"\\t")?,
                    c if c.is_control() => {
                        write!(self.writer, "\\u{:04x}", c as u32)?;
                    }
                    c => {
                        let mut buf = [0u8; 4];
                        self.writer.write_all(c.encode_utf8(&mut buf).as_bytes())?;
                    }
                }
            }
        }
        self.writer.write_all(b"\"")
    }

    #[inline]
    pub fn write_u64(&mut self, n: u64) -> io::Result<()> {
        let mut buf = itoa::Buffer::new();
        self.writer.write_all(buf.format(n).as_bytes())
    }

    #[inline]
    pub fn write_i64(&mut self, n: i64) -> io::Result<()> {
        let mut buf = itoa::Buffer::new();
        self.writer.write_all(buf.format(n).as_bytes())
    }

    #[inline]
    pub fn write_f64(&mut self, n: f64) -> io::Result<()> {
        let mut buf = ryu::Buffer::new();
        self.writer.write_all(buf.format(n).as_bytes())
    }

    #[inline]
    pub fn write_bool(&mut self, b: bool) -> io::Result<()> {
        if b {
            self.writer.write_all(b"true")
        } else {
            self.writer.write_all(b"false")
        }
    }

    #[inline]
    pub fn write_null(&mut self) -> io::Result<()> {
        self.writer.write_all(b"null")
    }

    pub fn write_value(&mut self, value: &Value) -> io::Result<()> {
        match value {
            Value::Null => self.write_null(),
            Value::Bool(b) => self.write_bool(*b),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    self.write_i64(i)
                } else if let Some(u) = n.as_u64() {
                    self.write_u64(u)
                } else if let Some(f) = n.as_f64() {
                    self.write_f64(f)
                } else {
                    self.write_null()
                }
            }
            Value::String(s) => self.write_string(s),
            Value::Array(_) | Value::Object(_) => {
                // For complex nested structures, fall back to serde_json
                let s = serde_json::to_string(value).map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, e)
                })?;
                self.writer.write_all(s.as_bytes())
            }
        }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}
