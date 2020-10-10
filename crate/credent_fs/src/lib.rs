#![deny(missing_debug_implementations, missing_docs)]

//! Reads / Writes credentials from / to disk.

mod app_name;
mod credentials_file;
mod credentials_file_loader;
mod credentials_file_storer;
mod error;

pub use crate::{
    app_name::AppName,
    credentials_file::{CredentialsFile, CREDENTIALS_FILE_NAME},
    credentials_file_loader::CredentialsFileLoader,
    credentials_file_storer::CredentialsFileStorer,
    error::Error,
};
