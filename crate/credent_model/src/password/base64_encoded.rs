use std::{
    convert::Infallible,
    fmt::{self, Debug, Display},
    str::FromStr,
};

/// Password to login, encoded in base64. `String` newtype.
///
/// The `Debug` and `Display` implementations for this type mask the password.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, PartialEq, Eq)]
pub struct Base64Encoded(String);

impl Base64Encoded {
    /// Returns a new base64 encoded password.
    pub fn new<S>(plain_text: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(base64::encode(AsRef::<str>::as_ref(&plain_text)))
    }

    /// Returns the in-memory representation of the password.
    ///
    /// This is the base64 encoded password.
    pub fn encoded(&self) -> &str {
        &self.0
    }

    /// Returns the plain text password.
    pub fn plain_text(&self) -> String {
        let decoded_bytes = base64::decode(&self.0).unwrap_or_else(Self::unreachable_decode);
        String::from_utf8(decoded_bytes).unwrap_or_else(Self::unreachable_utf8)
    }

    #[cfg(not(tarpaulin_include))]
    fn unreachable_decode(_: impl std::error::Error) -> Vec<u8> {
        unreachable!(
            "Password Base64 decode failed.\n\
                 This should be impossible as we only decode what we have encoded."
        )
    }

    #[cfg(not(tarpaulin_include))]
    fn unreachable_utf8(_: impl std::error::Error) -> String {
        unreachable!(
            "Failed to construct UTF8 string from decoded base64 password.\n\
             This should be impossible as we only handle valid UTF8 strings."
        )
    }
}

// Never reveal the password, even in `Debug`
impl Debug for Base64Encoded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Base64Encoded(\"******\")")
    }
}

impl Display for Base64Encoded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "******")
    }
}

impl FromStr for Base64Encoded {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Base64Encoded, Infallible> {
        Ok(Base64Encoded::new(s))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Base64Encoded;

    #[test]
    fn stores_base64_encoded_password() {
        assert_eq!("aGk=", Base64Encoded::new("hi").encoded());
    }

    #[test]
    fn returns_plain_text() {
        assert_eq!("hi", Base64Encoded::new("hi").plain_text());
    }

    #[test]
    fn debug_masks_password() {
        assert_eq!(
            "Base64Encoded(\"******\")",
            format!("{:?}", Base64Encoded::new("hi"))
        )
    }

    #[test]
    fn display_masks_password() {
        assert_eq!("******", format!("{}", Base64Encoded::new("hi")))
    }

    #[test]
    fn from_str_encodes_string() {
        assert_eq!(
            "aGk=",
            Base64Encoded::from_str("hi")
                .unwrap_or_else(unreachable)
                .encoded()
        );
    }

    #[cfg(not(tarpaulin_include))]
    fn unreachable(_: impl std::error::Error) -> Base64Encoded {
        unreachable!()
    }
}
