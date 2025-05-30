async fn async_task() {
    println!("Task started");
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("Task finished");
}

fn main() {
    async_task();
    println!("Main function ending");
}
