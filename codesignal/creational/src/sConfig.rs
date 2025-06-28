use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex};

struct Config {
    settings: Mutex<HashMap<String, String>>,
    access_count: AtomicU32,
}

impl Config {
    fn get(&self, key: &str) -> Option<String> {
        // Return owned String to avoid lifetime issues
        self.access_count.fetch_add(1, Ordering::SeqCst);
        let settings = self.settings.lock().unwrap();
        settings.get(key).cloned() // Clone to avoid holding the lock
    }
    fn get_access_count(&self) -> u32 {
        self.access_count.load(Ordering::SeqCst)
    }
    fn set(&self, key: String, value: String) {
        // Now takes &self instead of &mut self
        let mut settings = self.settings.lock().unwrap();
        settings.insert(key, value);
    }
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    settings: Mutex::new(HashMap::new()),
    access_count: AtomicU32::new(0),
});

pub fn main() {
    CONFIG.set("theme".to_string(), "dark".to_string()); // Now works!

    if let Some(theme) = CONFIG.get("theme") {
        println!("Theme: {}", theme);
    }
    println!("Config accessed: {} times", CONFIG.get_access_count());
}
