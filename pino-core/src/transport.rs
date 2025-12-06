use std::sync::Arc;
use parking_lot::RwLock;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::io::{self, Write};

pub struct AsyncTransport {
    sender: Sender<String>,
}

impl AsyncTransport {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<String>();

        thread::spawn(move || {
            Self::write_loop(receiver);
        });

        Self { sender }
    }

    pub fn send(&self, log: String) {
        let _ = self.sender.send(log);
    }

    fn write_loop(receiver: Receiver<String>) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        for log in receiver {
            let _ = writeln!(handle, "{}", log);
            let _ = handle.flush();
        }
    }
}

impl Default for AsyncTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for AsyncTransport {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

pub struct BufferedTransport {
    buffer: Arc<RwLock<Vec<String>>>,
    capacity: usize,
    writer: Arc<RwLock<Box<dyn Write + Send + Sync>>>,
}

impl BufferedTransport {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Arc::new(RwLock::new(Vec::with_capacity(capacity))),
            capacity,
            writer: Arc::new(RwLock::new(Box::new(io::stdout()))),
        }
    }

    pub fn write(&self, log: String) {
        let mut buffer = self.buffer.write();
        buffer.push(log);

        if buffer.len() >= self.capacity {
            self.flush_buffer(&mut buffer);
        }
    }

    pub fn flush(&self) {
        let mut buffer = self.buffer.write();
        self.flush_buffer(&mut buffer);
    }

    fn flush_buffer(&self, buffer: &mut Vec<String>) {
        if buffer.is_empty() {
            return;
        }

        let mut writer = self.writer.write();
        for log in buffer.drain(..) {
            let _ = writeln!(writer, "{}", log);
        }
        let _ = writer.flush();
    }
}

impl Drop for BufferedTransport {
    fn drop(&mut self) {
        self.flush();
    }
}
