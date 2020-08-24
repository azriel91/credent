use std::cmp::{Ordering, PartialOrd};

use crate::{Password, Username};

/// Credentials to log into the application.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Credentials {
    /// Username to login.
    pub username: Username,
    /// Password to login.
    pub password: Password,
}

impl PartialOrd for Credentials {
    fn partial_cmp(&self, other: &Credentials) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Credentials {
    fn cmp(&self, other: &Credentials) -> Ordering {
        self.username.cmp(&other.username)
    }
}
