use std::process;

#[tokio::main]
async fn main() {
    dictate::run().await.unwrap_or_else(|e| {
        eprintln!("dictate: {}", e.message);
        process::exit(e.code);
    });
}
