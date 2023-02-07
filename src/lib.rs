pub mod entry;
pub use entry::Entry;

pub mod client;

pub mod io;

pub struct Error {
    pub code: i32,
    pub message: String,
}

pub async fn run() -> Result<(), Error> {
    let args = io::Args::parse().or_else(|e| {
        Err(Error {
            code: 2,
            message: e.to_string(),
        })
    })?;

    let entries = client::fetch_word(&args.word).await.or_else(|e| {
        Err(Error {
            code: 1,
            message: e.to_string(),
        })
    })?;

    for (i, entry) in entries.iter().enumerate() {
        println!("{}", entry);
        if i < entries.len() - 1 {
            println!();
        }
    }

    Ok(())
}
