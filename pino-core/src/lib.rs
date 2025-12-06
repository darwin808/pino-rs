mod level;
mod logger;
mod record;
mod bindings;
mod transport;

#[cfg(test)]
mod tests;

pub use level::Level;
pub use logger::{Logger, LoggerBuilder};
pub use record::LogRecord;
pub use transport::{AsyncTransport, BufferedTransport};

use std::collections::HashMap;
use serde_json::Value;

pub type Fields = HashMap<String, Value>;
