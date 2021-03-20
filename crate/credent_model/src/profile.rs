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
pub struct Profile<C = Credentials> {
    /// Profile name.
    pub name: String,
    /// Credentials for this profile.
    pub credentials: C,
}

impl<C> Profile<C> {
    /// Name given to the *default* profile.
    pub const DEFAULT_NAME: &'static str = "default";

    /// Returns a new `Profile`.
    pub fn new(name: String, credentials: C) -> Self {
        Self { name, credentials }
    }

    /// Returns a new `Profile` with the `"default"` name.
    pub fn new_default(credentials: C) -> Self {
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

impl<C> PartialOrd for Profile<C>
where
    C: Eq,
{
    fn partial_cmp(&self, other: &Profile<C>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> Ord for Profile<C>
where
    C: Eq,
{
    fn cmp(&self, other: &Profile<C>) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl<C> Display for Profile<C>
where
    C: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{profile_name}/{credentials}",
            profile_name = self.name,
            credentials = self.credentials
        )
    }
}

impl<C> Borrow<str> for Profile<C> {
    fn borrow(&self) -> &str {
        &self.name
    }
}
