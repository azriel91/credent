use std::{
    convert::{Infallible, TryFrom},
    fmt::{self, Display},
    ops::{Deref, DerefMut},
};

/// Name of an application. `&str` newtype.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AppName<'s>(pub &'s str);

impl<'s> Deref for AppName<'s> {
    type Target = &'s str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s> DerefMut for AppName<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'s> Display for AppName<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'s> TryFrom<&'s str> for AppName<'s> {
    type Error = Infallible;

    fn try_from(s: &str) -> Result<AppName, Infallible> {
        Ok(AppName(s))
    }
}
