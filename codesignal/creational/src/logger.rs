use std::sync::LazyLock;
use std::sync::atomic::{AtomicU32, Ordering};

struct Logger {
    // TODO: Add a counter 'log_count' using AtomicU32 to keep track of log messages and initialize with value 0
    log_count: AtomicU32,
}

impl Logger {
    fn log(&self, message: &str) {
        // Increment counter and get the previous value
        let count = self.log_count.fetch_add(1, Ordering::SeqCst) + 1;
        // Print message with log count
        println!("{} (Log count: {})", message, count);
    }
}

static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger {
    // TODO: Initialize the 'log_count' with AtomicU32::new(0)
    log_count: AtomicU32::new(0),
});

fn main() {
    LOGGER.log("First log message.");
    LOGGER.log("Second log message.");
}
