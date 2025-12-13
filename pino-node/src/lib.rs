use napi::bindgen_prelude::*;
use napi::Either;
use napi_derive::napi;
use pino_core::{AsyncLogger as CoreLogger, Level, Fields};
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
        let mut logger = CoreLogger::new();

        if let Some(opts) = options {
            // Parse level
            if let Ok(Some(level_str)) = opts.get::<_, String>("level") {
                let level = match level_str.as_str() {
                    "trace" => Level::Trace,
                    "debug" => Level::Debug,
                    "info" => Level::Info,
                    "warn" => Level::Warn,
                    "error" => Level::Error,
                    "fatal" => Level::Fatal,
                    _ => Level::Info,
                };
                logger.set_level(level);
            }

            // Parse base fields
            if let Ok(Some(base)) = opts.get::<_, Object>("base") {
                let keys = Object::keys(&base)?;
                let mut base_fields = HashMap::new();
                for key in keys {
                    if let Ok(Some(val)) = base.get::<_, Unknown>(&key) {
                        if let Ok(json_val) = napi_value_to_json(val) {
                            base_fields.insert(key, json_val);
                        }
                    }
                }
                if !base_fields.is_empty() {
                    logger = logger.child(base_fields);
                }
            }
        }

        Ok(Self {
            inner: logger,
        })
    }

    #[napi]
    pub fn trace(&self, msg_or_obj: Either<String, Object>, msg: Option<String>) -> Result<()> {
        log_with_either(&self.inner, pino_core::Level::Trace, msg_or_obj, msg)
    }

    #[napi]
    pub fn debug(&self, msg_or_obj: Either<String, Object>, msg: Option<String>) -> Result<()> {
        log_with_either(&self.inner, pino_core::Level::Debug, msg_or_obj, msg)
    }

    #[napi]
    pub fn info(&self, msg_or_obj: Either<String, Object>, msg: Option<String>) -> Result<()> {
        log_with_either(&self.inner, pino_core::Level::Info, msg_or_obj, msg)
    }

    #[napi]
    pub fn warn(&self, msg_or_obj: Either<String, Object>, msg: Option<String>) -> Result<()> {
        log_with_either(&self.inner, pino_core::Level::Warn, msg_or_obj, msg)
    }

    #[napi]
    pub fn error(&self, msg_or_obj: Either<String, Object>, msg: Option<String>) -> Result<()> {
        log_with_either(&self.inner, pino_core::Level::Error, msg_or_obj, msg)
    }

    #[napi]
    pub fn fatal(&self, msg_or_obj: Either<String, Object>, msg: Option<String>) -> Result<()> {
        log_with_either(&self.inner, pino_core::Level::Fatal, msg_or_obj, msg)
    }

    #[napi]
    pub fn child(&self, bindings: Object) -> Result<Self> {
        let fields = parse_fields(bindings)?;
        Ok(Self {
            inner: self.inner.child(fields),
        })
    }

    #[napi]
    pub fn bindings(&self, env: Env) -> Result<Object> {
        let fields = self.inner.get_bindings();
        let mut obj = env.create_object()?;

        for (key, value) in fields {
            let js_val = json_to_napi_value(&env, value)?;
            obj.set(key.as_str(), js_val)?;
        }

        Ok(obj)
    }
}

fn log_with_either(
    logger: &CoreLogger,
    level: Level,
    msg_or_obj: Either<String, Object>,
    msg: Option<String>,
) -> Result<()> {
    match msg_or_obj {
        Either::A(message) => {
            // First argument is a string (the message)
            match level {
                Level::Trace => logger.trace(&message),
                Level::Debug => logger.debug(&message),
                Level::Info => logger.info(&message),
                Level::Warn => logger.warn(&message),
                Level::Error => logger.error(&message),
                Level::Fatal => logger.fatal(&message),
            }
        }
        Either::B(obj) => {
            // First argument is an object (fields), second argument is message
            let fields = parse_fields(obj)?;
            let message = msg.as_deref();
            match level {
                Level::Trace => {
                    if let Some(m) = message {
                        logger.trace_with_fields(m, fields);
                    } else {
                        logger.log(level, None, Some(fields));
                    }
                }
                Level::Debug => {
                    if let Some(m) = message {
                        logger.debug_with_fields(m, fields);
                    } else {
                        logger.log(level, None, Some(fields));
                    }
                }
                Level::Info => {
                    if let Some(m) = message {
                        logger.info_with_fields(m, fields);
                    } else {
                        logger.log(level, None, Some(fields));
                    }
                }
                Level::Warn => {
                    if let Some(m) = message {
                        logger.warn_with_fields(m, fields);
                    } else {
                        logger.log(level, None, Some(fields));
                    }
                }
                Level::Error => {
                    if let Some(m) = message {
                        logger.error_with_fields(m, fields);
                    } else {
                        logger.log(level, None, Some(fields));
                    }
                }
                Level::Fatal => {
                    if let Some(m) = message {
                        logger.fatal_with_fields(m, fields);
                    } else {
                        logger.log(level, None, Some(fields));
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_fields(obj: Object) -> Result<Fields> {
    let keys = Object::keys(&obj)?;
    let mut fields = HashMap::with_capacity(keys.len());

    for key in keys {
        if let Ok(Some(val)) = obj.get::<_, Unknown>(&key) {
            if let Ok(json_val) = napi_value_to_json(val) {
                fields.insert(key, json_val);
            }
        }
    }

    Ok(fields)
}

fn napi_value_to_json(val: Unknown) -> Result<Value> {
    use napi::{ValueType, JsString, JsNumber, JsBoolean};

    let value_type = val.get_type()?;

    match value_type {
        ValueType::String => {
            let s = unsafe { val.cast::<JsString>() };
            let utf8 = s.into_utf8()?;
            Ok(Value::String(utf8.as_str()?.to_string()))
        }
        ValueType::Number => {
            let n = unsafe { val.cast::<JsNumber>() };
            let num = n.get_double()?;
            if num.fract() == 0.0 && num.is_finite() {
                Ok(Value::Number(serde_json::Number::from(num as i64)))
            } else if let Some(num_val) = serde_json::Number::from_f64(num) {
                Ok(Value::Number(num_val))
            } else {
                Ok(Value::Null)
            }
        }
        ValueType::Boolean => {
            let b = unsafe { val.cast::<JsBoolean>() };
            Ok(Value::Bool(b.get_value()?))
        }
        ValueType::Null | ValueType::Undefined => {
            Ok(Value::Null)
        }
        _ => Ok(Value::Null),
    }
}

fn json_to_napi_value(env: &Env, value: &Value) -> Result<napi::JsUnknown> {
    match value {
        Value::String(s) => Ok(env.create_string(s)?.into_unknown()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(env.create_int64(i)?.into_unknown())
            } else if let Some(f) = n.as_f64() {
                Ok(env.create_double(f)?.into_unknown())
            } else {
                Ok(env.get_null()?.into_unknown())
            }
        }
        Value::Bool(b) => Ok(env.get_boolean(*b)?.into_unknown()),
        Value::Null => Ok(env.get_null()?.into_unknown()),
        Value::Array(arr) => {
            let mut js_arr = env.create_array(arr.len() as u32)?;
            for (i, v) in arr.iter().enumerate() {
                let js_val = json_to_napi_value(env, v)?;
                js_arr.set(i as u32, js_val)?;
            }
            Ok(js_arr.coerce_to_object()?.into_unknown())
        }
        Value::Object(obj) => {
            let mut js_obj = env.create_object()?;
            for (k, v) in obj {
                let js_val = json_to_napi_value(env, v)?;
                js_obj.set(k.as_str(), js_val)?;
            }
            Ok(js_obj.into_unknown())
        }
    }
}

#[napi]
pub fn pino(options: Option<Object>) -> Result<Logger> {
    Logger::new(options)
}
