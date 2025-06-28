use std::sync::LazyLock;

// Define the ConfigManager struct
struct ConfigManager;

impl ConfigManager {
    // Method to load configurations
    fn load_config(&self, config: &str) {
        println!("Loading config: {}", config);
    }
}

// Create a static instance using LazyLock for lazy initialization
static CONFIG_MANAGER: LazyLock<ConfigManager> = LazyLock::new(|| ConfigManager);

fn main() {
    // Access the singleton instance and call the load_config method
    CONFIG_MANAGER.load_config("app_settings.json");
}
