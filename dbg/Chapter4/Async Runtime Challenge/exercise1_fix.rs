async fn async_task() {
    println!("Task started");
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("Task finished");
}

#[tokio::main]
async fn main() {
    println!("Starting program");
    async_task().await;
    println!("Main function ending");
}
