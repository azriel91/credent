#![deny(missing_debug_implementations, missing_docs)]

//! Reads credentials from the command line and stores them.

use credent::{
    cli::CredentialsCliReader,
    fs::{model::AppName, CredentialsFileStorer},
    model::Profile,
};
use credent_model::Credentials;

/// Application name
const CREDENT: AppName<'_> = AppName("credent");

type CredentialsFile = credent::fs::CredentialsFile<Credentials>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let credentials = CredentialsCliReader::read_from_tty().await?;
        println!("credentials: {}", credentials);

        let profile = Profile {
            name: String::from("default"),
            credentials,
        };
        CredentialsFileStorer::store(CREDENT, &profile).await?;

        println!(
            "credentials written to: {}",
            CredentialsFile::path(CREDENT)?.display()
        );

        Result::<(), Box<dyn std::error::Error>>::Ok(())
    })
}
