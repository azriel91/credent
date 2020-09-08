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
pub struct PlainText(pub String);

impl PlainText {
    /// Returns the plain text password.
    pub fn plain_text(&self) -> &str {
        &self.0
    }
}

impl Deref for PlainText {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlainText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Never reveal the password, even in `Debug`
impl Debug for PlainText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PlainText(\"******\")")
    }
}

impl Display for PlainText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "******")
    }
}

impl FromStr for PlainText {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<PlainText, Infallible> {
        Ok(PlainText(s.to_string()))
    }
}
