use std::sync::LazyLock;

// Define the Logger struct
struct Logger;

// Implement the Logger struct
impl Logger {
    // Define a log method that takes a message and log level as arguments and prints the message in the format "LEVEL: MESSAGE"
    fn log(&self, level: &str, message: &str) {
        println!("{}: {}", level, message);
    }
}

// Create a static LazyLock instance of Logger to implement the singleton pattern
static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger);

fn main() {
    // Call the log method of the Logger instance with a message "Server started" and log level "INFO"
    LOGGER.log("INFO", "Server started");
    // Call the log method of the Logger instance with a message "Server failed" and log level "ERROR"
    LOGGER.log("ERROR", "Server failed");
}
