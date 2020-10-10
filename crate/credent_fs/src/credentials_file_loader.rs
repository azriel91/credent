use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use credent_model::{Credentials, Profile};

use crate::{AppName, CredentialsFile, Error};

/// Reads credentials from the user's configuration directory.
#[derive(Debug)]
pub struct CredentialsFileLoader;

impl CredentialsFileLoader {
    /// Returns all profile credentials stored in the user's configuration
    /// directory.
    ///
    /// The path differs depending on the user's operating system:
    ///
    /// * `Windows`: `C:\Users\%USER%\AppData\Roaming\<app>\credentials`
    /// * `Linux`: `$XDG_CONFIG_HOME` or `$HOME/.config/<app>/credentials`
    /// * `OS X`: `$HOME/Library/Application Support/<app>/credentials`
    ///
    /// # Parameters
    ///
    /// * `app_name`: Name of the application whose credentials to load.
    pub async fn load_all(app_name: AppName<'_>) -> Option<Result<BTreeSet<Profile>, Error>> {
        let credentials_path = CredentialsFile::path(app_name).ok()?;
        if credentials_path.exists() {
            let profiles_result = Self::load_file(credentials_path.as_ref()).await;
            Some(profiles_result)
        } else {
            None
        }
    }

    /// Returns the default profile credentials stored in the user's
    /// configuration directory.
    ///
    /// The path differs depending on the user's operating system:
    ///
    /// * `Windows`: `C:\Users\%USER%\AppData\Roaming\<app>\credentials`
    /// * `Linux`: `$XDG_CONFIG_HOME` or `$HOME/.config/<app>/credentials`
    /// * `OS X`: `$HOME/Library/Application Support/<app>/credentials`
    ///
    /// # Parameters
    ///
    /// * `app_name`: Name of the application whose credentials to load.
    pub async fn load(app_name: AppName<'_>) -> Option<Result<Profile, Error>> {
        let credentials_path = CredentialsFile::path(app_name).ok()?;
        if credentials_path.exists() {
            Self::load_profile(app_name, Profile::DEFAULT_NAME).await
        } else {
            None
        }
    }

    /// Returns the profile credentials stored in the user's configuration
    /// directory.
    ///
    /// The path differs depending on the user's operating system:
    ///
    /// * `Windows`: `C:\Users\%USER%\AppData\Roaming\<app>\credentials`
    /// * `Linux`: `$XDG_CONFIG_HOME` or `$HOME/.config/<app>/credentials`
    /// * `OS X`: `$HOME/Library/Application Support/<app>/credentials`
    ///
    /// # Parameters
    ///
    /// * `app_name`: Name of the application whose credentials to load.
    /// * `profile_name`: Which profile's credentials to load.
    pub async fn load_profile(
        app_name: AppName<'_>,
        profile_name: &str,
    ) -> Option<Result<Profile, Error>> {
        Self::load_all(app_name)
            .await
            .map(|profiles_result| {
                profiles_result
                    .map(|profiles| {
                        profiles
                            .into_iter()
                            .find(|profile| &profile.name == profile_name)
                    })
                    .transpose()
            })
            .flatten()
    }

    /// Loads all credential profiles from the given file.
    ///
    /// # Parameters
    ///
    /// * `credentials_path`: File to load credentials from.
    pub async fn load_file(credentials_path: &Path) -> Result<BTreeSet<Profile>, Error> {
        if !credentials_path.exists() {
            let credentials_path = credentials_path.to_owned();
            Err(Error::CredentialsFileNonExistent { credentials_path })
        } else if credentials_path.is_dir() {
            let credentials_path = credentials_path.to_owned();
            Err(Error::CredentialsFileIsDir { credentials_path })
        } else {
            let profiles_contents = Self::credentials_file_read(credentials_path).await?;
            Self::credentials_deserialize(profiles_contents, credentials_path)
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
        profiles_contents: Vec<u8>,
        credentials_path: &Path,
    ) -> Result<BTreeSet<Profile>, Error> {
        let profiles_map_result = toml::from_slice::<BTreeMap<String, Credentials>>(
            &profiles_contents,
        )
        .map_err(|toml_de_error| {
            let credentials_path = credentials_path.to_owned();
            Error::CredentialsFileFailedToDeserialize {
                credentials_path,
                toml_de_error,
            }
        });

        profiles_map_result.map(|profile_map| {
            profile_map
                .into_iter()
                .map(|(name, credentials)| Profile::new(name, credentials))
                .collect()
        })
    }
}
