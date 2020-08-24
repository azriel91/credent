use std::{
    convert::Infallible,
    fmt::{self, Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Password to login. `String` newtype.
///
/// The `Debug` and `Display` implementations for this type mask the password.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, PartialEq, Eq)]
pub struct Password(pub String);

impl Deref for Password {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Password {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Never reveal the password, even in `Debug`
impl Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Password(\"******\")")
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "******")
    }
}

impl FromStr for Password {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Password, Infallible> {
        Ok(Password(s.to_string()))
    }
}
