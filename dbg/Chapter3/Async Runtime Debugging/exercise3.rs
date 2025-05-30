use tokio; // Import Tokio for async runtime support

#[tokio::main] // Initialize the Tokio runtime for the main function
async fn main() { // Define the main function as asynchronous
    tokio::spawn(async { // Spawn the first async task
        println!("Task 1: Start"); // Print Task 1 starting message
        tokio::time::sleep(std::time::Duration::from_secs(2)).await; // Non-blocking delay for 2 seconds
        println!("Task 1: Done"); // Print Task 1 completion message
    });
    tokio::spawn(async { // Spawn the second async task
        println!("Task 2: Start"); // Print Task 2 starting message
        println!("Task 2: Done"); // Print Task 2 completion message immediately
    });

    tokio::time::sleep(std::time::Duration::from_secs(3)).await; // Non-blocking delay for 3 seconds to let tasks finish
}
