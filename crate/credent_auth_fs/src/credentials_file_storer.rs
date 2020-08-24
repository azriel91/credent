use std::path::Path;

use credent_auth_model::Credentials;

use crate::{AppName, CredentialsFile, Error};

/// Writes credentials to the user's configuration directory.
#[derive(Debug)]
pub struct CredentialsFileStorer;

impl CredentialsFileStorer {
    /// Returns the credentials stored in the user's configuration directory.
    ///
    /// The path differs depending on the user's operating system:
    ///
    /// * `Windows`: `C:\Users\%USER%\AppData\Roaming\<app>\credentials`
    /// * `Linux`: `$XDG_CONFIG_HOME` or `$HOME/.config/<app>/credentials`
    /// * `OS X`: `$HOME/Library/Application Support/<app>/credentials`
    pub async fn store(app_name: AppName<'_>, credentials: &Credentials) -> Result<(), Error> {
        let credentials_path = CredentialsFile::path(app_name)?;
        Self::store_file(credentials, credentials_path.as_ref()).await
    }

    /// Stores `Credentials` in the given file.
    ///
    /// Currently this overwrites the credentials in the file. In the future,
    /// this may be changed to handle multiple credentials in the same file.
    ///
    /// # Parameters
    ///
    /// * `credentials_path`: File to write credentials to.
    pub async fn store_file(
        credentials: &Credentials,
        credentials_path: &Path,
    ) -> Result<(), Error> {
        Self::credentials_parent_create(credentials_path).await?;
        let credentials_contents = Self::credentials_serialize(credentials)?;
        Self::credentials_file_write(credentials_contents.as_bytes(), credentials_path).await?;

        Ok(())
    }

    async fn credentials_parent_create(credentials_path: &Path) -> Result<(), Error> {
        if let Some(parent_path) = credentials_path.parent() {
            async_fs::create_dir_all(parent_path)
                .await
                .map_err(|io_error| {
                    let parent_path = parent_path.to_owned();
                    Error::CredentialsParentDirFailedToCreate {
                        parent_path,
                        io_error,
                    }
                })?;
        }
        Ok(())
    }

    async fn credentials_file_write(
        credentials_contents: &[u8],
        credentials_path: &Path,
    ) -> Result<(), Error> {
        async_fs::write(credentials_path, credentials_contents)
            .await
            .map_err(|io_error| {
                let credentials_path = credentials_path.to_owned();
                Error::CredentialsFileFailedToWrite {
                    credentials_path,
                    io_error,
                }
            })
    }

    fn credentials_serialize(credentials: &Credentials) -> Result<String, Error> {
        toml::ser::to_string_pretty(credentials).map_err(|toml_ser_error| {
            let credentials = credentials.clone();
            Error::CredentialsFileFailedToSerialize {
                credentials,
                toml_ser_error,
            }
        })
    }
}
