use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));

    let c1_task1 = Arc::clone(&counter1);
    let c2_task1 = Arc::clone(&counter2);
    let c1_task2 = Arc::clone(&counter1);
    let c2_task2 = Arc::clone(&counter2);

    let handle1 = tokio::spawn(async move {
        let mut num1 = c1_task1.lock().await;  // Line 15: Lock counter1 first
        let mut num2 = c2_task1.lock().await;  // Line 16: Lock counter2 second
        *num1 += 1;
        *num2 += 1;
        println!("Task 1: {} {}", *num1, *num2);
    });

    let handle2 = tokio::spawn(async move {
        let mut num1 = c1_task2.lock().await;  // Line 22: Lock counter1 first
        let mut num2 = c2_task2.lock().await;  // Line 23: Lock counter2 second
        *num1 += 1;
        *num2 += 1;
        println!("Task 2: {} {}", *num1, *num2);
    });

    handle1.await.unwrap();
    handle2.await.unwrap();

    let final1 = counter1.lock().await;
    let final2 = counter2.lock().await;
    println!("Final Values: {} {}", *final1, *final2);

     // Add a delay to keep the app running so tokio-console can connect
     println!("Keeping the app running for tokio-console... Press Ctrl+C to exit.");
     tokio::time::sleep(std::time::Duration::from_secs(60)).await; // Wait for 60 seconds
}

