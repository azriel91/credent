use std::{marker::PhantomData, path::PathBuf};

use credent_fs_model::{AppName, Error};
use credent_model::Credentials;

/// Name of the file used to store credentials.
pub const CREDENTIALS_FILE_NAME: &str = "credentials";

/// Returns the path to the credentials file.
#[derive(Debug)]
pub struct CredentialsFile<C = Credentials>(PhantomData<C>);

impl<C> CredentialsFile<C>
where
    C: Clone + Eq,
{
    /// Returns the path to the credentials in the user's configuration
    /// directory.
    ///
    /// The file's existence is not checked -- that is the responsibility of the
    /// caller.
    ///
    /// The path differs depending on the user's operating system:
    ///
    /// * `Windows`: `C:\Users\%USER%\AppData\Roaming\<app>\credentials`
    /// * `Linux`: `$XDG_CONFIG_HOME` or `$HOME/.config/<app>/credentials`
    /// * `OS X`: `$HOME/Library/Application Support/<app>/credentials`
    pub fn path(app_name: AppName<'_>) -> Result<PathBuf, Error<C>> {
        dirs::config_dir()
            .map(|config_dir| config_dir.join(*app_name).join(CREDENTIALS_FILE_NAME))
            .ok_or(Error::UserConfigDirNotFound)
    }
}
