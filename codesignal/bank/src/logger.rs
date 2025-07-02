use std::sync::LazyLock;

// Static LOGGER instance using LazyLock
static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger);

pub struct Logger;

impl Logger {
    // Returns a reference to the singleton Logger instance
    pub fn instance() -> &'static Logger {
        &LOGGER
    }

    // Logs a message with a timestamp
    pub fn log(msg: String) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        println!("[{}] {}", timestamp, msg);
    }
}
