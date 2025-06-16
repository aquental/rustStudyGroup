use tokio::time;
use tokio::io::AsyncReadExt;
use log::Level;

async fn run() {
    tokio::join!(
        sleeper(), 
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
    );
}

fn fib(n: u32) -> u32 {
    match n{
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

async fn sleeper() {
    log::info!("Sleeping...");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("Awake!");
}

async fn reader() {
    log::info!("Reading a lot of data...");    
    let mut f = tokio::fs::File::open("./target/debug/tokio-nb").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read {} bytes", contents.len());
    tokio::task::spawn_blocking(move || {
        log::info!("Spawning blocking fib(41)");
        fib(41);
        log::info!("Done computing");
    }).await.unwrap();
    
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();
    log::info!("Finished in {:?} seconds", end - start);
}
