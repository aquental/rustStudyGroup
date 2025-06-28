use std::sync::LazyLock;

// Define the Logger struct
struct Logger;

impl Logger {
    // Modified log method to accept severity level
    fn log(&self, level: &str, message: &str) {
        // Prefix the message with the severity level
        println!("{}: {}", level, message);
    }
}

// Create a lazily-initialized static instance of Logger
static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger {});

fn main() {
    // Updated log calls with severity levels
    LOGGER.log("INFO", "Singleton pattern example with Logger.");
    LOGGER.log("WARNING", "This is a warning message.");
    LOGGER.log("ERROR", "This is an error message.");
}
