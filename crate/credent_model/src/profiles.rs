use std::{
    collections::BTreeSet,
    fmt::{self, Display},
    ops::{Deref, DerefMut},
};

use crate::Profile;

/// Set of `Profile`s. `BTreeSet` newtype.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Profiles(pub BTreeSet<Profile>);

impl Deref for Profiles {
    type Target = BTreeSet<Profile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Profiles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Profiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut profile_iter = self.0.iter();

        if let Some(profile_first) = profile_iter.next() {
            write!(f, "{}", profile_first)?;
        }
        profile_iter.try_for_each(|profile| write!(f, ", {}", profile))?;

        write!(f, "]")
    }
}
