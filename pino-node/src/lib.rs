use napi::bindgen_prelude::*;
use napi_derive::napi;
use pino_core::{Logger as CoreLogger, LoggerBuilder, Level, Fields};
use serde_json::Value;
use std::collections::HashMap;

#[napi]
pub struct Logger {
    inner: CoreLogger,
}

#[napi]
impl Logger {
    #[napi(constructor)]
    pub fn new(options: Option<Object>) -> Result<Self> {
        let mut builder = LoggerBuilder::new();

        if let Some(opts) = options {
            // Parse level
            if let Ok(level_str) = opts.get::<_, String>("level") {
                let level = match level_str.as_str() {
                    "trace" => Level::Trace,
                    "debug" => Level::Debug,
                    "info" => Level::Info,
                    "warn" => Level::Warn,
                    "error" => Level::Error,
                    "fatal" => Level::Fatal,
                    _ => Level::Info,
                };
                builder = builder.level(level);
            }

            // Parse base fields
            if let Ok(base) = opts.get::<_, Object>("base") {
                let keys = Object::keys(&base)?;
                for key in keys {
                    if let Ok(val) = base.get::<_, Unknown>(&key) {
                        if let Ok(json_val) = napi_value_to_json(val) {
                            builder = builder.with_field(key, json_val);
                        }
                    }
                }
            }
        }

        Ok(Self {
            inner: builder.build(),
        })
    }

    #[napi]
    pub fn trace(&self, msg: String, fields: Option<Object>) -> Result<()> {
        if let Some(f) = fields {
            let parsed_fields = parse_fields(f)?;
            self.inner.trace_with_fields(&msg, parsed_fields);
        } else {
            self.inner.trace(&msg);
        }
        Ok(())
    }

    #[napi]
    pub fn debug(&self, msg: String, fields: Option<Object>) -> Result<()> {
        if let Some(f) = fields {
            let parsed_fields = parse_fields(f)?;
            self.inner.debug_with_fields(&msg, parsed_fields);
        } else {
            self.inner.debug(&msg);
        }
        Ok(())
    }

    #[napi]
    pub fn info(&self, msg: String, fields: Option<Object>) -> Result<()> {
        if let Some(f) = fields {
            let parsed_fields = parse_fields(f)?;
            self.inner.info_with_fields(&msg, parsed_fields);
        } else {
            self.inner.info(&msg);
        }
        Ok(())
    }

    #[napi]
    pub fn warn(&self, msg: String, fields: Option<Object>) -> Result<()> {
        if let Some(f) = fields {
            let parsed_fields = parse_fields(f)?;
            self.inner.warn_with_fields(&msg, parsed_fields);
        } else {
            self.inner.warn(&msg);
        }
        Ok(())
    }

    #[napi]
    pub fn error(&self, msg: String, fields: Option<Object>) -> Result<()> {
        if let Some(f) = fields {
            let parsed_fields = parse_fields(f)?;
            self.inner.error_with_fields(&msg, parsed_fields);
        } else {
            self.inner.error(&msg);
        }
        Ok(())
    }

    #[napi]
    pub fn fatal(&self, msg: String, fields: Option<Object>) -> Result<()> {
        if let Some(f) = fields {
            let parsed_fields = parse_fields(f)?;
            self.inner.fatal_with_fields(&msg, parsed_fields);
        } else {
            self.inner.fatal(&msg);
        }
        Ok(())
    }

    #[napi]
    pub fn child(&self, bindings: Object) -> Result<Self> {
        let fields = parse_fields(bindings)?;
        Ok(Self {
            inner: self.inner.child(fields),
        })
    }
}

fn parse_fields(obj: Object) -> Result<Fields> {
    let mut fields = HashMap::new();
    let keys = Object::keys(&obj)?;

    for key in keys {
        if let Ok(val) = obj.get::<_, Unknown>(&key) {
            if let Ok(json_val) = napi_value_to_json(val) {
                fields.insert(key, json_val);
            }
        }
    }

    Ok(fields)
}

fn napi_value_to_json(val: Unknown) -> Result<Value> {
    // Try to convert to different types
    if let Ok(s) = val.coerce_to_string() {
        let utf8 = s.into_utf8()?;
        return Ok(Value::String(utf8.as_str()?.to_string()));
    }

    if let Ok(n) = val.coerce_to_number() {
        let num = n.get_double()?;
        if num.fract() == 0.0 && num.is_finite() {
            return Ok(Value::Number(serde_json::Number::from(num as i64)));
        }
        if let Some(num_val) = serde_json::Number::from_f64(num) {
            return Ok(Value::Number(num_val));
        }
    }

    if let Ok(b) = val.coerce_to_bool() {
        return Ok(Value::Bool(b.get_value()?));
    }

    Ok(Value::Null)
}

#[napi]
pub fn pino(options: Option<Object>) -> Result<Logger> {
    Logger::new(options)
}
