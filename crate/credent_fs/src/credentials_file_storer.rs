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

    /// Stores multiple `Profile`s in the default application credentials file.
    ///
    /// This replaces the specified profiles' credentials in the file, other
    /// profiles not included in the parameter are untouched in the file.
    ///
    /// The path differs depending on the user's operating system:
    ///
    /// * `Windows`: `C:\Users\%USER%\AppData\Roaming\<app>\credentials`
    /// * `Linux`: `$XDG_CONFIG_HOME` or `$HOME/.config/<app>/credentials`
    /// * `OS X`: `$HOME/Library/Application Support/<app>/credentials`
    pub async fn store_many(app_name: AppName<'_>, profiles: Profiles<C>) -> Result<(), Error<C>> {
        let credentials_path = CredentialsFile::path(app_name)?;
        Self::store_many_file(profiles, credentials_path.as_ref()).await
    }

    /// Stores a `Profile` in the given file.
    ///
    /// This replaces the profile's credentials in the file.
    ///
    /// # Parameters
    ///
    /// * `profile`: Profile to store.
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

    /// Stores multiple `Profile`s in the given file.
    ///
    /// This replaces the specified profiles' credentials in the file, other
    /// profiles not included in the parameter are untouched in the file.
    ///
    /// # Parameters
    ///
    /// * `profiles`: Profiles to store.
    /// * `credentials_path`: File to write credentials to.
    pub async fn store_many_file(
        mut profiles: Profiles<C>,
        credentials_path: &Path,
    ) -> Result<(), Error<C>> {
        let profiles_existing = Self::profiles_existing(credentials_path).await?;
        let profiles_from_file = profiles_existing.unwrap_or_else(Profiles::<C>::new);

        profiles_from_file
            .0
            .into_iter()
            .for_each(|profile_from_file| {
                if !profiles.contains(&profile_from_file) {
                    profiles.insert(profile_from_file);
                }
            });

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
    use credent_model::{Credentials, Password, Profile, Profiles, Username};
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

    #[test]
    fn store_many_file_adds_profiles_when_non_existent() -> Result<(), Box<dyn std::error::Error>> {
        smol::block_on(async {
            let file = NamedTempFile::new()?;
            let profiles_new = {
                let profile_new_a = Profile::new(
                    String::from("profile_new_a"),
                    Credentials {
                        username: Username(String::from("me_a")),
                        password: Password::new("secret"),
                    },
                );
                let profile_new_b = Profile::new(
                    String::from("profile_new_b"),
                    Credentials {
                        username: Username(String::from("me_b")),
                        password: Password::new("secret"),
                    },
                );

                let mut profiles_new = Profiles::new();
                profiles_new.insert(profile_new_a);
                profiles_new.insert(profile_new_b);
                profiles_new
            };

            CredentialsFileStorer::store_many_file(profiles_new, file.path()).await?;

            #[cfg(feature = "base64")]
            let content_expected = "\
                [profile_new_a]\n\
                username = 'me_a'\n\
                password = 'c2VjcmV0'\n\
                \n\
                [profile_new_b]\n\
                username = 'me_b'\n\
                password = 'c2VjcmV0'\n\
            ";
            #[cfg(not(feature = "base64"))]
            let content_expected = "\
                [profile_new_a]\n\
                username = 'me_a'\n\
                password = 'secret'\n\
                \n\
                [profile_new_b]\n\
                username = 'me_b'\n\
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
    fn store_many_file_replaces_profiles_when_pre_existent(
    ) -> Result<(), Box<dyn std::error::Error>> {
        smol::block_on(async {
            let mut file = NamedTempFile::new()?;
            write!(file, "{}", PROFILES_CONTENT)?;

            let profiles_replace = {
                let profile_default = Profile::new_default(Credentials {
                    username: Username(String::from("me")),
                    password: Password::new("boo"),
                });
                let profile_other = Profile::new(
                    String::from("profile_other"),
                    Credentials {
                        username: Username(String::from("you")),
                        password: Password::new("boo"),
                    },
                );
                let profile_other_b = Profile::new(
                    String::from("profile_other_b"),
                    Credentials {
                        username: Username(String::from("me_b")),
                        password: Password::new("boo"),
                    },
                );

                let mut profiles_replace = Profiles::new();
                profiles_replace.insert(profile_default);
                profiles_replace.insert(profile_other);
                profiles_replace.insert(profile_other_b);
                profiles_replace
            };

            CredentialsFileStorer::store_many_file(profiles_replace, file.path()).await?;

            #[cfg(feature = "base64")]
            let content_expected = "\
                [default]\n\
                username = 'me'\n\
                password = 'Ym9v'\n\
                \n\
                [profile_other]\n\
                username = 'you'\n\
                password = 'Ym9v'\n\
                \n\
                [profile_other_b]\n\
                username = 'me_b'\n\
                password = 'Ym9v'\n\
            ";
            #[cfg(not(feature = "base64"))]
            let content_expected = "\
                [default]\n\
                username = 'me'\n\
                password = 'boo'\n\
                \n\
                [profile_other]\n\
                username = 'you'\n\
                password = 'boo'\n\
                \n\
                [profile_other_b]\n\
                username = 'me_b'\n\
                password = 'boo'\n\
            ";

            let mut file = File::open(file.path()).await?;
            let mut contents = String::new();
            let _n = file.read_to_string(&mut contents).await?;
            assert_eq!(content_expected, contents);

            Ok(())
        })
    }

    #[test]
    fn store_many_file_retains_other_existing_profiles() -> Result<(), Box<dyn std::error::Error>> {
        smol::block_on(async {
            let mut file = NamedTempFile::new()?;
            write!(file, "{}", PROFILES_CONTENT)?;

            let profiles_replace = {
                let profile_other = Profile::new(
                    String::from("profile_other"),
                    Credentials {
                        username: Username(String::from("you")),
                        password: Password::new("boo"),
                    },
                );
                let profile_other_b = Profile::new(
                    String::from("profile_other_b"),
                    Credentials {
                        username: Username(String::from("me_b")),
                        password: Password::new("boo"),
                    },
                );

                let mut profiles_replace = Profiles::new();
                profiles_replace.insert(profile_other);
                profiles_replace.insert(profile_other_b);
                profiles_replace
            };

            CredentialsFileStorer::store_many_file(profiles_replace, file.path()).await?;

            #[cfg(feature = "base64")]
            let content_expected = "\
                [default]\n\
                username = 'me'\n\
                password = 'c2VjcmV0'\n\
                \n\
                [profile_other]\n\
                username = 'you'\n\
                password = 'Ym9v'\n\
                \n\
                [profile_other_b]\n\
                username = 'me_b'\n\
                password = 'Ym9v'\n\
            ";
            #[cfg(not(feature = "base64"))]
            let content_expected = "\
                [default]\n\
                username = 'me'\n\
                password = 'secret'\n\
                \n\
                [profile_other]\n\
                username = 'you'\n\
                password = 'boo'\n\
                \n\
                [profile_other_b]\n\
                username = 'me_b'\n\
                password = 'boo'\n\
            ";

            let mut file = File::open(file.path()).await?;
            let mut contents = String::new();
            let _n = file.read_to_string(&mut contents).await?;
            assert_eq!(content_expected, contents);

            Ok(())
        })
    }
}
