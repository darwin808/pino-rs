mod level;
mod logger;
mod record;
mod bindings;
mod transport;
mod fast_json;
mod async_logger;

#[cfg(test)]
mod tests;

pub use level::Level;
pub use logger::{Logger, LoggerBuilder};
pub use async_logger::AsyncLogger;
pub use record::LogRecord;
pub use transport::{AsyncTransport, BufferedTransport};

use std::collections::HashMap;
use serde_json::Value;

pub type Fields = HashMap<String, Value>;
