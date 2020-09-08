#![deny(missing_debug_implementations, missing_docs)]

//! Control Credent from the command line.
//!
//! ```text,ignore
//!                _         _
//!  ___ ___ ___ _| |___ ___| |_
//! |  _|  _| -_| . | -_|   |  _|
//! |___|_| |___|___|___|_|_|_|
//! ```

pub use credent_auth_cli::CredentialsCliReader;
pub use credent_auth_fs::{AppName, CredentialsFile, CredentialsFileLoader, CredentialsFileStorer};
pub use credent_auth_model::{Credentials, Password};
