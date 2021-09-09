use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = rustconv::run().await {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}
