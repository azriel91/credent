#![deny(missing_debug_implementations, missing_docs)]

//! Manages `~/.config/<app>/credentials`.
//!
//! ![demo](https://raw.githubusercontent.com/azriel91/credent/main/demo.png)
//!
//! Add the following to Cargo.toml:
//!
//! ```toml
//! credent = { version = "0.4.0", features = ["backend-smol"] } # or "backend-tokio"
//! ```
//!
//! Example code:
//!
//! ```rust,ignore
//! use credent::{
//!     cli::CredentialsCliReader,
//!     fs::{model::AppName, CredentialsFile, CredentialsFileStorer},
//!     model::Credentials,
//! };
//!
//! /// Application name
//! const CREDENT: AppName<'_> = AppName("credent");
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     smol::run(async {
//!         let credentials = CredentialsCliReader::<Credentials>::read_from_tty().await?;
//!         println!("credentials: {}", credentials);
//!
//!         CredentialsFileStorer::<Credentials>::store(CREDENT, &credentials).await?;
//!
//!         println!(
//!             "credentials written to: {}",
//!             CredentialsFile::<Credentials>::path(CREDENT)?.display()
//!         );
//!
//!         Result::<(), Box<dyn std::error::Error>>::Ok(())
//!     })
//! }
//! ```
//!
//! More examples can be seen in the [examples].
//!
//! [examples]: https://github.com/azriel91/credent/tree/main/examples

pub use credent_cli as cli;
pub use credent_fs as fs;
pub use credent_model as model;
