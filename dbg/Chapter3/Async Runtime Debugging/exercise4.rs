use tokio; // Import Tokio for async runtime support

#[tokio::main] // Initialize the Tokio runtime for the main function
async fn main() { // Define the main function as asynchronous
    for i in 0..3 { // Loop three times to create three tasks
        tokio::spawn(async move { // Spawn a new async task for each iteration
            println!("Task {}: Start", i); // Print the task's starting message with its number
            tokio::time::sleep(std::time::Duration::from_secs(1)).await; // Non-blocking delay for 1 second
            println!("Task {}: Done", i); // Print the task's completion message with its number
        });
    }
    tokio::time::sleep(std::time::Duration::from_secs(2)).await; // Non-blocking delay for 2 seconds to let tasks finish
}

