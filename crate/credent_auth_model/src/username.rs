use std::{
    convert::Infallible,
    fmt::{self, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Username to login. `String` newtype.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Username(pub String);

impl Deref for Username {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Username {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Username {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Username, Infallible> {
        Ok(Username(s.to_string()))
    }
}
