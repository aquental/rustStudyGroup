use tokio::time::{sleep, Duration}; // Import Tokio's sleep and Duration for delays

#[tokio::main] // Initialize the Tokio runtime for the main function
async fn main() { // Define the main function as asynchronous
    println!("Task started..."); // Print a starting message
    sleep(Duration::from_secs(2)).await; // Non-blocking delay for 2 seconds
    println!("Task completed!"); // Print a completion message
}
