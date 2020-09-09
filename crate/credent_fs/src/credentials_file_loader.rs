use std::path::Path;

use credent_model::Credentials;

use crate::{AppName, CredentialsFile, Error};

/// Reads credentials from the user's configuration directory.
#[derive(Debug)]
pub struct CredentialsFileLoader;

impl CredentialsFileLoader {
    /// Returns the credentials stored in the user's configuration directory.
    ///
    /// The path differs depending on the user's operating system:
    ///
    /// * `Windows`: `C:\Users\%USER%\AppData\Roaming\<app>\credentials`
    /// * `Linux`: `$XDG_CONFIG_HOME` or `$HOME/.config/<app>/credentials`
    /// * `OS X`: `$HOME/Library/Application Support/<app>/credentials`
    pub async fn load(app_name: AppName<'_>) -> Option<Result<Credentials, Error>> {
        let credentials_path = CredentialsFile::path(app_name).ok()?;
        if credentials_path.exists() {
            let credentials_result = Self::load_file(credentials_path.as_ref()).await;
            Some(credentials_result)
        } else {
            None
        }
    }

    /// Loads `Credentials` from the given file.
    ///
    /// # Parameters
    ///
    /// * `credentials_path`: File to load credentials from.
    pub async fn load_file(credentials_path: &Path) -> Result<Credentials, Error> {
        if !credentials_path.exists() {
            let credentials_path = credentials_path.to_owned();
            Err(Error::CredentialsFileNonExistent { credentials_path })
        } else if credentials_path.is_dir() {
            let credentials_path = credentials_path.to_owned();
            Err(Error::CredentialsFileIsDir { credentials_path })
        } else {
            let credentials_contents = Self::credentials_file_read(credentials_path).await?;
            Self::credentials_deserialize(credentials_contents, credentials_path)
        }
    }

    async fn credentials_file_read(credentials_path: &Path) -> Result<Vec<u8>, Error> {
        async_fs::read(credentials_path).await.map_err(|io_error| {
            let credentials_path = credentials_path.to_owned();
            Error::CredentialsFileFailedToRead {
                credentials_path,
                io_error,
            }
        })
    }

    fn credentials_deserialize(
        credentials_contents: Vec<u8>,
        credentials_path: &Path,
    ) -> Result<Credentials, Error> {
        toml::from_slice(&credentials_contents).map_err(|toml_de_error| {
            let credentials_path = credentials_path.to_owned();
            Error::CredentialsFileFailedToDeserialize {
                credentials_path,
                toml_de_error,
            }
        })
    }
}
