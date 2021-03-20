use std::{fmt, path::PathBuf};

use credent_model::{Credentials, Profiles};

/// Errors when reading the user credentials file.
#[derive(Debug)]
pub enum Error<C = Credentials>
where
    C: Clone + Eq,
{
    /// Unable to determine user configuration directory.
    UserConfigDirNotFound,
    /// Failed to create the parent directory of the credentials file.
    CredentialsParentDirCreate {
        /// Path to the user credentials file.
        parent_path: PathBuf,
        /// The underlying IO error.
        error: std::io::Error,
    },
    /// User credentials file does not exist.
    CredentialsFileNonExistent {
        /// Path to the user credentials file.
        credentials_path: PathBuf,
    },
    /// User credentials file is a directory.
    CredentialsFileIsDir {
        /// Path to the user credentials file.
        credentials_path: PathBuf,
    },
    /// Failed to read from the user credentials file.
    CredentialsFileRead {
        /// Path to the user credentials file.
        credentials_path: PathBuf,
        /// The underlying IO error.
        error: std::io::Error,
    },
    /// Failed to write to the user credentials file.
    CredentialsFileWrite {
        /// Path to the user credentials file.
        credentials_path: PathBuf,
        /// The underlying IO error.
        error: std::io::Error,
    },
    /// Failed to deserialize user credentials file contents.
    CredentialsFileDeserialize {
        /// Path to the user credentials file.
        credentials_path: PathBuf,
        /// The underlying TOML error.
        error: toml::de::Error,
    },
    /// Failed to serialize user credentials.
    CredentialsFileSerialize {
        /// Profiles which failed to be serialized.
        profiles: Profiles<C>,
        /// The underlying TOML error.
        error: toml::ser::Error,
    },
}

impl<C> fmt::Display for Error<C>
where
    C: Clone + Eq + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UserConfigDirNotFound => {
                write!(f, "Unable to determine user configuration directory.")
            }
            Self::CredentialsParentDirCreate { parent_path, error } => write!(
                f,
                "Failed to create credentials file parent directory.\n\
                Path: `{}`\n\
                Error: `{}`",
                parent_path.display(),
                error
            ),
            Self::CredentialsFileNonExistent { credentials_path } => write!(
                f,
                "User credentials does not exist or cannot be accessed. Path: `{}`",
                credentials_path.display()
            ),
            Self::CredentialsFileIsDir { credentials_path } => write!(
                f,
                "User credentials file should be a file, but it is a directory. Path: `{}`",
                credentials_path.display()
            ),
            Self::CredentialsFileRead {
                credentials_path,
                error,
            } => write!(
                f,
                "User credentials file failed to be read.\n\
                Path: `{}`\n\
                Error: `{}`",
                credentials_path.display(),
                error
            ),
            Self::CredentialsFileWrite {
                credentials_path,
                error,
            } => write!(
                f,
                "User credentials file failed to be read.\n\
                Path: `{}`\n\
                Error: `{}`",
                credentials_path.display(),
                error
            ),
            Self::CredentialsFileDeserialize {
                credentials_path,
                error,
            } => write!(
                f,
                "User credentials file failed to be deserialized.\n\
                Path: `{}`\n\
                Error: `{}`",
                credentials_path.display(),
                error
            ),
            Self::CredentialsFileSerialize { profiles, error } => write!(
                f,
                "User credentials failed to be serialized.\n\
                Profiles: `{:?}`\n\
                Error: `{}`",
                profiles, error
            ),
        }
    }
}

impl<C> std::error::Error for Error<C> where C: Clone + Eq + fmt::Debug {}
