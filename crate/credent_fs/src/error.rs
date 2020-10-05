use std::{
    collections::BTreeSet,
    fmt::{self, Display},
    path::PathBuf,
};

use credent_model::Profile;

/// Errors when reading the user credentials file.
#[derive(Debug)]
pub enum Error {
    /// Unable to determine user configuration directory.
    UserConfigDirNotFound,
    /// Failed to create the parent directory of the credentials file.
    CredentialsParentDirFailedToCreate {
        /// Path to the user credentials file.
        parent_path: PathBuf,
        /// The underlying IO error.
        io_error: std::io::Error,
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
    CredentialsFileFailedToRead {
        /// Path to the user credentials file.
        credentials_path: PathBuf,
        /// The underlying IO error.
        io_error: std::io::Error,
    },
    /// Failed to write to the user credentials file.
    CredentialsFileFailedToWrite {
        /// Path to the user credentials file.
        credentials_path: PathBuf,
        /// The underlying IO error.
        io_error: std::io::Error,
    },
    /// Failed to deserialize user credentials file contents.
    CredentialsFileFailedToDeserialize {
        /// Path to the user credentials file.
        credentials_path: PathBuf,
        /// The underlying TOML error.
        toml_de_error: toml::de::Error,
    },
    /// Failed to serialize user credentials.
    CredentialsFileFailedToSerialize {
        /// Profiles which failed to be serialized.
        profiles: BTreeSet<Profile>,
        /// The underlying TOML error.
        toml_ser_error: toml::ser::Error,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UserConfigDirNotFound => {
                write!(f, "Unable to determine user configuration directory.")
            }
            Self::CredentialsParentDirFailedToCreate {
                parent_path,
                io_error,
            } => write!(
                f,
                "Failed to create credentials file parent directory.\n\
                Path: `{}`\n\
                Error: `{}`",
                parent_path.display(),
                io_error
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
            Self::CredentialsFileFailedToRead {
                credentials_path,
                io_error,
            } => write!(
                f,
                "User credentials file failed to be read.\n\
                Path: `{}`\n\
                Error: `{}`",
                credentials_path.display(),
                io_error
            ),
            Self::CredentialsFileFailedToWrite {
                credentials_path,
                io_error,
            } => write!(
                f,
                "User credentials file failed to be read.\n\
                Path: `{}`\n\
                Error: `{}`",
                credentials_path.display(),
                io_error
            ),
            Self::CredentialsFileFailedToDeserialize {
                credentials_path,
                toml_de_error,
            } => write!(
                f,
                "User credentials file failed to be deserialized.\n\
                Path: `{}`\n\
                Error: `{}`",
                credentials_path.display(),
                toml_de_error
            ),
            Self::CredentialsFileFailedToSerialize {
                profiles,
                toml_ser_error,
            } => write!(
                f,
                "User credentials failed to be serialized.\n\
                Profiles: `{:?}`\n\
                Error: `{}`",
                profiles, toml_ser_error
            ),
        }
    }
}

impl std::error::Error for Error {}
