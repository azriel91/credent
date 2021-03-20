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
            Self::CredentialsParentDirCreate { parent_path, .. } => write!(
                f,
                "Failed to create credentials file parent directory. Path: `{}`",
                parent_path.display()
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
                credentials_path, ..
            } => write!(
                f,
                "User credentials file failed to be read. Path: `{}`",
                credentials_path.display()
            ),
            Self::CredentialsFileWrite {
                credentials_path, ..
            } => write!(
                f,
                "User credentials file failed to be read. Path: `{}`",
                credentials_path.display()
            ),
            Self::CredentialsFileDeserialize {
                credentials_path, ..
            } => write!(
                f,
                "User credentials file failed to be deserialized. Path: `{}`",
                credentials_path.display()
            ),
            Self::CredentialsFileSerialize { profiles, .. } => write!(
                f,
                "User credentials failed to be serialized. Profiles: `{:?}`",
                profiles
            ),
        }
    }
}

impl<C> std::error::Error for Error<C>
where
    C: Clone + Eq + fmt::Debug,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::UserConfigDirNotFound => None,
            Self::CredentialsParentDirCreate { error, .. } => Some(error),
            Self::CredentialsFileNonExistent { .. } => None,
            Self::CredentialsFileIsDir { .. } => None,
            Self::CredentialsFileRead { error, .. } => Some(error),
            Self::CredentialsFileWrite { error, .. } => Some(error),
            Self::CredentialsFileDeserialize { error, .. } => Some(error),
            Self::CredentialsFileSerialize { error, .. } => Some(error),
        }
    }
}
