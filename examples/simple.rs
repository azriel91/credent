#![deny(missing_debug_implementations, missing_docs)]

//! Reads credentials from the command line and stores them.

use credent::{
    cli::CredentialsCliReader,
    fs::{AppName, CredentialsFile, CredentialsFileStorer},
};

/// Application name
const CREDENT: AppName<'_> = AppName("credent");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::run(async {
        let credentials = CredentialsCliReader::read_from_tty().await?;
        println!("credentials: {}", credentials);

        CredentialsFileStorer::store(CREDENT, &credentials).await?;

        println!(
            "credentials written to: {}",
            CredentialsFile::path(CREDENT)?.display()
        );

        Result::<(), Box<dyn std::error::Error>>::Ok(())
    })
}
