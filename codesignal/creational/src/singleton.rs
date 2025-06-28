use std::sync::LazyLock;

struct Logger;

impl Logger {
    fn log(&self, message: &str) {
        println!("{}", message);
    }
}

static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger);

fn main() {
    LOGGER.log("Hello, from LOGGER!");
}
