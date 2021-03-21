use std::{marker::PhantomData, path::Path};

use credent_fs_model::{AppName, Error};
use credent_model::{Credentials, Profile, Profiles};
use serde::{Deserialize, Serialize};

use crate::{CredentialsFile, CredentialsFileLoader};

/// Writes credentials to the user's configuration directory.
#[derive(Debug)]
pub struct CredentialsFileStorer<C = Credentials>(PhantomData<C>);

impl<C> CredentialsFileStorer<C>
where
    C: Clone + Eq + for<'de> Deserialize<'de> + Serialize,
{
    /// Stores a `Profile` in the default application credentials file.
    ///
    /// This replaces the profile's credentials in the file.
    ///
    /// The path differs depending on the user's operating system:
    ///
    /// * `Windows`: `C:\Users\%USER%\AppData\Roaming\<app>\credentials`
    /// * `Linux`: `$XDG_CONFIG_HOME` or `$HOME/.config/<app>/credentials`
    /// * `OS X`: `$HOME/Library/Application Support/<app>/credentials`
    pub async fn store(app_name: AppName<'_>, profile: &Profile<C>) -> Result<(), Error<C>> {
        let credentials_path = CredentialsFile::path(app_name)?;
        Self::store_file(profile, credentials_path.as_ref()).await
    }

    /// Stores a `Profile` in the given file.
    ///
    /// This replaces the profile's credentials in the file.
    ///
    /// # Parameters
    ///
    /// * `credentials_path`: File to write credentials to.
    pub async fn store_file(profile: &Profile<C>, credentials_path: &Path) -> Result<(), Error<C>> {
        let profiles_existing = Self::profiles_existing(credentials_path).await?;
        let mut profiles = profiles_existing.unwrap_or_else(Profiles::<C>::new);

        // [`BTreeSet::insert`] does not replace the value if the `Ordering` is the
        // same, which it is for `Profile`s with the same name, even if the
        // username or password differ.
        profiles.replace(profile.clone());

        let profiles_contents = Self::profiles_serialize(&profiles)?;

        Self::credentials_parent_create(credentials_path).await?;
        Self::credentials_file_write(profiles_contents.as_bytes(), credentials_path).await?;

        Ok(())
    }

    async fn profiles_existing(credentials_path: &Path) -> Result<Option<Profiles<C>>, Error<C>> {
        if credentials_path.exists() {
            CredentialsFileLoader::<C>::load_file(credentials_path)
                .await
                .map(Some)
        } else {
            Ok(None)
        }
    }

    async fn credentials_parent_create(credentials_path: &Path) -> Result<(), Error<C>> {
        if let Some(parent_path) = credentials_path.parent() {
            async_fs::create_dir_all(parent_path)
                .await
                .map_err(|error| {
                    let parent_path = parent_path.to_owned();
                    Error::CredentialsParentDirCreate { parent_path, error }
                })?;
        }
        Ok(())
    }

    async fn credentials_file_write(
        credentials_contents: &[u8],
        credentials_path: &Path,
    ) -> Result<(), Error<C>> {
        async_fs::write(credentials_path, credentials_contents)
            .await
            .map_err(|error| {
                let credentials_path = credentials_path.to_owned();
                Error::CredentialsFileWrite {
                    credentials_path,
                    error,
                }
            })
    }

    fn profiles_serialize(profiles: &Profiles<C>) -> Result<String, Error<C>> {
        toml::ser::to_string_pretty(&profiles).map_err(|error| {
            let profiles = profiles.clone();
            Error::CredentialsFileSerialize { profiles, error }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use async_fs::File;
    use credent_model::{Credentials, Password, Profile, Username};
    use futures_lite::io::AsyncReadExt;
    use tempfile::NamedTempFile;

    use super::CredentialsFileStorer;

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
    fn store_file_creates_file_when_non_existent() -> Result<(), Box<dyn std::error::Error>> {
        smol::block_on(async {
            let tempdir = tempfile::tempdir()?;
            let file_path = tempdir.path().join("credentials");
            let profile_default = Profile::new_default(Credentials {
                username: Username(String::from("me")),
                password: Password::new("secret"),
            });

            CredentialsFileStorer::store_file(&profile_default, &file_path).await?;

            #[cfg(feature = "base64")]
            let content_expected = "\
                [default]\n\
                username = 'me'\n\
                password = 'c2VjcmV0'\n\
            ";
            #[cfg(not(feature = "base64"))]
            let content_expected = "\
                [default]\n\
                username = 'me'\n\
                password = 'secret'\n\
            ";

            let mut file = File::open(&file_path).await?;
            let mut contents = String::new();
            let _n = file.read_to_string(&mut contents).await?;
            assert_eq!(content_expected, contents);

            Ok(())
        })
    }

    #[test]
    fn store_file_adds_profile_when_non_existent() -> Result<(), Box<dyn std::error::Error>> {
        smol::block_on(async {
            let file = NamedTempFile::new()?;
            let profile_other = Profile::new(
                String::from("profile_other"),
                Credentials {
                    username: Username(String::from("me")),
                    password: Password::new("secret"),
                },
            );

            CredentialsFileStorer::store_file(&profile_other, file.path()).await?;

            #[cfg(feature = "base64")]
            let content_expected = "\
                [profile_other]\n\
                username = 'me'\n\
                password = 'c2VjcmV0'\n\
            ";
            #[cfg(not(feature = "base64"))]
            let content_expected = "\
                [profile_other]\n\
                username = 'me'\n\
                password = 'secret'\n\
            ";

            let mut file = File::open(file.path()).await?;
            let mut contents = String::new();
            let _n = file.read_to_string(&mut contents).await?;
            assert_eq!(content_expected, contents);

            Ok(())
        })
    }

    #[test]
    fn store_file_replaces_profile_when_pre_existent() -> Result<(), Box<dyn std::error::Error>> {
        smol::block_on(async {
            let mut file = NamedTempFile::new()?;
            write!(file, "{}", PROFILES_CONTENT)?;

            let profile_default = Profile::new_default(Credentials {
                username: Username(String::from("me")),
                password: Password::new("boo"),
            });

            CredentialsFileStorer::store_file(&profile_default, file.path()).await?;

            #[cfg(feature = "base64")]
            let content_expected = "\
                [default]\n\
                username = 'me'\n\
                password = 'Ym9v'\n\
                \n\
                [profile_other]\n\
                username = 'you'\n\
                password = 'Y29kZQ=='\n\
            ";
            #[cfg(not(feature = "base64"))]
            let content_expected = "\
                [default]\n\
                username = 'me'\n\
                password = 'boo'\n\
                \n\
                [profile_other]\n\
                username = 'you'\n\
                password = 'code'\n\
            ";

            let mut file = File::open(file.path()).await?;
            let mut contents = String::new();
            let _n = file.read_to_string(&mut contents).await?;
            assert_eq!(content_expected, contents);

            Ok(())
        })
    }
}
