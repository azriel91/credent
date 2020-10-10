use std::{
    cmp::{Ordering, PartialOrd},
    fmt::{self, Debug, Display},
};

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

impl Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.username, self.password)
    }
}
