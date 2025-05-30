use async_std::task; // Import async-std's task module for async operations
//If you’re seeing an error, it could be due to:
//Missing Dependency: Ensure async-std is added to your Cargo.toml .
use std::time::Duration; // Import Duration for delays

#[async_std::main] // Initialize the async-std runtime for the main function
async fn main() { // Define the main function as asynchronous
    println!("Task started..."); // Print a starting message
    task::sleep(Duration::from_secs(2)).await; // Non-blocking delay for 2 seconds
    println!("Task completed!"); // Print a completion message
}
