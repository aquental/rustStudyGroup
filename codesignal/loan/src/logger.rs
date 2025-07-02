use std::sync::LazyLock;

// Define the Logger struct
pub struct Logger {
    // Private field to prevent direct instantiation
    _private: (),
}

// Create a static LOGGER instance using LazyLock
static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger { _private: () });

impl Logger {
    // Implement instance() method to return the singleton instance
    pub fn instance() -> &'static Logger {
        &LOGGER
    }

    // Implement the log method to output messages to the console
    pub fn log(&self, message: &str) {
        println!("{}", message);
    }
}

// Optional: Add a simple test to demonstrate usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let logger = Logger::instance();
        logger.log("Test message");
        // Verify same instance
        let logger2 = Logger::instance();
        assert_eq!(logger as *const _, logger2 as *const _);
    }
}
