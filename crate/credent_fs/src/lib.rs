#![deny(missing_debug_implementations, missing_docs)]

//! Reads / Writes credentials from / to disk.

pub use crate::{
    credentials_file::{CredentialsFile, CREDENTIALS_FILE_NAME},
    credentials_file_loader::CredentialsFileLoader,
    credentials_file_storer::CredentialsFileStorer,
};

pub use credent_fs_model as model;

mod credentials_file;
mod credentials_file_loader;
mod credentials_file_storer;
