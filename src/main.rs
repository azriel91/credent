//! Control Credent from the command line.
//!
//! ```text,ignore
//!                _         _
//!  ___ ___ ___ _| |___ ___| |_
//! |  _|  _| -_| . | -_|   |  _|
//! |___|_| |___|___|___|_|_|_|
//! ```

use credent_auth_cli::CredentialsCliReader;
use credent_auth_fs::{AppName, CredentialsFileLoader, CredentialsFileStorer};

use credent::Logo;

/// Application name
const CREDENT: AppName<'_> = AppName("credent");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", Logo::ascii_coloured());

    smol::run(async {
        let credentials_result = CredentialsFileLoader::load(CREDENT).await;
        let credentials = if let Some(credentials_result) = credentials_result {
            credentials_result?
        } else {
            let credentials = CredentialsCliReader::read_from_tty().await?;
            CredentialsFileStorer::store(CREDENT, &credentials).await?;

            credentials
        };

        println!("{:?}", credentials);

        Result::<(), Box<dyn std::error::Error>>::Ok(())
    })
}
