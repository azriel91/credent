use std::{
    convert::Infallible,
    fmt::{self, Debug, Display},
    str::FromStr,
};

/// Password to login. `String` newtype.
///
/// The `Debug` and `Display` implementations for this type mask the password.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, PartialEq, Eq)]
pub struct PlainText(String);

impl PlainText {
    /// Returns a new plain text password.
    pub fn new<S>(plain_text: S) -> Self
    where
        S: Into<String>,
    {
        Self(Into::<String>::into(plain_text))
    }

    /// Returns the in-memory representation of the password.
    ///
    /// This is simply the plain text password.
    pub fn encoded(&self) -> &str {
        &self.0
    }

    /// Returns the plain text password.
    pub fn plain_text(&self) -> &str {
        &self.0
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
        Ok(PlainText::new(s))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::PlainText;

    #[test]
    fn stores_plain_text_password() {
        assert_eq!("hi", PlainText::new("hi").encoded());
    }

    #[test]
    fn returns_plain_text() {
        assert_eq!("hi", PlainText::new("hi").plain_text());
    }

    #[test]
    fn debug_masks_password() {
        assert_eq!(
            "PlainText(\"******\")",
            format!("{:?}", PlainText::new("hi"))
        )
    }

    #[test]
    fn display_masks_password() {
        assert_eq!("******", format!("{}", PlainText::new("hi")))
    }

    #[test]
    fn from_str_returns_plain_text() {
        assert_eq!(
            "hi",
            PlainText::from_str("hi")
                .unwrap_or_else(unreachable)
                .encoded()
        );
    }

    #[cfg(not(tarpaulin_include))]
    fn unreachable(_: impl std::error::Error) -> PlainText {
        unreachable!()
    }
}
