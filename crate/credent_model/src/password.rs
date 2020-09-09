#[cfg(feature = "base64")]
pub use self::base64_encoded::Base64Encoded as Password;
#[cfg(not(feature = "base64"))]
pub use self::plain_text::PlainText as Password;

#[cfg(feature = "base64")]
mod base64_encoded;
#[cfg(not(feature = "base64"))]
mod plain_text;
