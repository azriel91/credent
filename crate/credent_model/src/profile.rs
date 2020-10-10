use std::{
    borrow::Borrow,
    cmp::{Ordering, PartialOrd},
    fmt::{self, Display},
};

use crate::Credentials;

/// Profile to store credentials under.
///
/// This allows the `credentials` file to hold credentials for multiple
/// environments.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Profile {
    /// Profile name.
    pub name: String,
    /// Credentials for this profile.
    pub credentials: Credentials,
}

impl Profile {
    /// Name given to the *default* profile.
    pub const DEFAULT_NAME: &'static str = "default";

    /// Returns a new `Profile`.
    pub fn new(name: String, credentials: Credentials) -> Self {
        Self { name, credentials }
    }

    /// Returns a new `Profile` with the `"default"` name.
    pub fn new_default(credentials: Credentials) -> Self {
        Self {
            name: String::from(Self::DEFAULT_NAME),
            credentials,
        }
    }

    /// Returns whether this profile has the `"default"` profile name.
    pub fn is_default(&self) -> bool {
        self.name == Self::DEFAULT_NAME
    }
}

impl PartialOrd for Profile {
    fn partial_cmp(&self, other: &Profile) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Profile {
    fn cmp(&self, other: &Profile) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{profile_name}/{credentials}",
            profile_name = self.name,
            credentials = self.credentials
        )
    }
}

impl Borrow<str> for Profile {
    fn borrow(&self) -> &str {
        &self.name
    }
}
