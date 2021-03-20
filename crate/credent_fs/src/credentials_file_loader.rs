use std::{marker::PhantomData, path::Path};

use credent_fs_model::{AppName, Error};
use credent_model::{Credentials, Profile, Profiles};
use serde::Deserialize;

use crate::CredentialsFile;

/// Reads credentials from the user's configuration directory.
#[derive(Debug)]
pub struct CredentialsFileLoader<C = Credentials>(PhantomData<C>);

impl<C> CredentialsFileLoader<C>
where
    C: Clone + Eq + for<'de> Deserialize<'de>,
{
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
    pub async fn load(app_name: AppName<'_>) -> Result<Option<Profile<C>>, Error<C>> {
        let credentials_path = CredentialsFile::<C>::path(app_name)?;
        if credentials_path.exists() {
            Self::load_profile(app_name, Profile::<C>::DEFAULT_NAME).await
        } else {
            Ok(None)
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
    ) -> Result<Option<Profile<C>>, Error<C>> {
        Self::load_all(app_name)
            .await
            .map(|profiles_result| {
                profiles_result.map(|profiles| {
                    profiles
                        .0
                        .into_iter()
                        .find(|profile| profile.name == profile_name)
                })
            })
            .map(Option::flatten)
    }

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
    pub async fn load_all(app_name: AppName<'_>) -> Result<Option<Profiles<C>>, Error<C>> {
        let credentials_path = CredentialsFile::<C>::path(app_name)?;
        if credentials_path.exists() {
            Self::load_file(credentials_path.as_ref()).await.map(Some)
        } else {
            Ok(None)
        }
    }

    /// Loads all credential profiles from the given file.
    ///
    /// # Parameters
    ///
    /// * `credentials_path`: File to load credentials from.
    pub async fn load_file(credentials_path: &Path) -> Result<Profiles<C>, Error<C>> {
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

    async fn credentials_file_read(credentials_path: &Path) -> Result<Vec<u8>, Error<C>> {
        async_fs::read(credentials_path).await.map_err(|error| {
            let credentials_path = credentials_path.to_owned();
            Error::CredentialsFileRead {
                credentials_path,
                error,
            }
        })
    }

    fn credentials_deserialize(
        profiles_contents: Vec<u8>,
        credentials_path: &Path,
    ) -> Result<Profiles<C>, Error<C>> {
        toml::from_slice(&profiles_contents).map_err(|error| {
            let credentials_path = credentials_path.to_owned();
            Error::CredentialsFileDeserialize {
                credentials_path,
                error,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use credent_fs_model::Error;
    use credent_model::{Credentials, Password, Profile, Profiles, Username};
    use tempfile::NamedTempFile;

    use super::CredentialsFileLoader;

    #[cfg(feature = "base64")]
    const PROFILES_CONTENT: &str = r#"
        [default]
        username = "me"
        password = "c2VjcmV0" # secret

        [profile_other]
        username = "you"
        password = "Y29kZQ==" # code
    "#;

    #[cfg(not(feature = "base64"))]
    const PROFILES_CONTENT: &str = r#"
        [default]
        username = "me"
        password = "secret"

        [profile_other]
        username = "you"
        password = "code"
    "#;

    #[test]
    fn loads_credentials() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        write!(file, "{}", PROFILES_CONTENT)?;

        let profiles: Profiles = smol::block_on(CredentialsFileLoader::load_file(file.path()))?;
        let profile_default = profiles.get(Profile::<Credentials>::DEFAULT_NAME);
        let profile_other = profiles.get("profile_other");

        let profile_default_expected = Profile::new_default(Credentials {
            username: Username(String::from("me")),
            password: Password::new("secret"),
        });
        assert_eq!(Some(&profile_default_expected), profile_default);

        let profile_other_expected = Profile::new(
            String::from("profile_other"),
            Credentials {
                username: Username(String::from("you")),
                password: Password::new("code"),
            },
        );
        assert_eq!(Some(&profile_other_expected), profile_other);

        Ok(())
    }

    #[test]
    fn returns_err_file_non_existent_when_file_not_exist() -> Result<(), Box<dyn std::error::Error>>
    {
        let file = NamedTempFile::new()?;
        let path = file.path().to_path_buf();
        file.close()?;

        let load_result = smol::block_on(CredentialsFileLoader::<Credentials>::load_file(&path));

        if let Err(Error::CredentialsFileNonExistent { credentials_path }) = &load_result {
            assert_eq!(&path, credentials_path);
        } else {
            panic!("Expected `load_result` to return `CredentialsFileNonExistent` error, but got `{:?}`.", load_result);
        }

        Ok(())
    }

    #[test]
    fn returns_err_file_is_dir_when_file_is_directory() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempfile::tempdir()?;
        let path = temp_dir.path();

        let load_result = smol::block_on(CredentialsFileLoader::<Credentials>::load_file(path));

        if let Err(Error::CredentialsFileIsDir { credentials_path }) = &load_result {
            assert_eq!(path, credentials_path);
        } else {
            panic!(
                "Expected `load_result` to return `CredentialsFileIsDir` error, but got `{:?}`.",
                load_result
            );
        }

        Ok(())
    }
    #[test]
    fn returns_err_deserialize_when_contents_are_broken() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut file = NamedTempFile::new()?;
        write!(file, "garbage")?;
        let path = file.path();

        let load_result = smol::block_on(CredentialsFileLoader::<Credentials>::load_file(path));

        if let Err(Error::CredentialsFileDeserialize {
            credentials_path,
            error: _,
        }) = &load_result
        {
            assert_eq!(path, credentials_path);
        } else {
            panic!(
                "Expected `load_result` to return `CredentialsFileDeserialize` error, but got `{:?}`.",
                load_result
            );
        }

        Ok(())
    }
}
