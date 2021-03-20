#![deny(missing_debug_implementations, missing_docs)]

//! Data types used when reading credentials from the file system.

pub use crate::{app_name::AppName, error::Error};

mod app_name;
mod error;
