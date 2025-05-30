use tokio::sync::Mutex; // Import Tokio’s async-aware Mutex for safe concurrency
use std::sync::Arc; // Import Arc for shared ownership across tasks

#[tokio::main] // Initialize the Tokio runtime for the main function
async fn main() { // Define the main function as asynchronous
    let counter1 = Arc::new(Mutex::new(0)); // Create a shared Mutex-protected counter1
    let counter2 = Arc::new(Mutex::new(0)); // Create a shared Mutex-protected counter2

    let c1_task1 = Arc::clone(&counter1); // Clone counter1 for Task 1
    let c2_task1 = Arc::clone(&counter2); // Clone counter2 for Task 1
    let c1_task2 = Arc::clone(&counter1); // Clone counter1 for Task 2
    let c2_task2 = Arc::clone(&counter2); // Clone counter2 for Task 2

    let handle1 = tokio::spawn(async move { // Spawn Task 1
        let mut num1 = c1_task1.lock().await; // Lock counter1 in Task 1
        let mut num2 = c2_task1.lock().await; // Lock counter2 in Task 1 (consistent order)
        *num1 += 1; // Increment counter1
        *num2 += 1; // Increment counter2
        println!("Task 1: {} {}", *num1, *num2); // Print the updated values
    });

    let handle2 = tokio::spawn(async move { // Spawn Task 2
        let mut num1 = c1_task2.lock().await; // Lock counter1 in Task 2 (same order)
        let mut num2 = c2_task2.lock().await; // Lock counter2 in Task 2 (same order)
        *num1 += 1; // Increment counter1
        *num2 += 1; // Increment counter2
        println!("Task 2: {} {}", *num1, *num2); // Print the updated values
    });

    handle1.await.unwrap(); // Wait for Task 1 to finish
    handle2.await.unwrap(); // Wait for Task 2 to finish

    let final1 = counter1.lock().await; // Lock counter1 to read final value
    let final2 = counter2.lock().await; // Lock counter2 to read final value
    println!("Final Values: {} {}", *final1, *final2); // Print the final values of both counters
}
