use std::{
    cmp::{Ordering, PartialOrd},
    fmt::{self, Debug, Display},
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
