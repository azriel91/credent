#![deny(missing_debug_implementations, missing_docs)]

//! Reads in credentials from the CLI.

pub use crate::credentials_cli_reader::CredentialsCliReader;

pub use credent_cli_model as model;

mod credentials_cli_reader;
