use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{self, Display},
    ops::{Deref, DerefMut},
};

use crate::{Credentials, Profile};

/// Set of `Profile`s. `BTreeSet` newtype.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        from = "BTreeMap<String, Credentials>",
        into = "BTreeMap<String, Credentials>"
    )
)]
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

impl From<BTreeMap<String, Credentials>> for Profiles {
    fn from(profile_map: BTreeMap<String, Credentials>) -> Self {
        let profiles_set = profile_map
            .into_iter()
            .map(|(name, credentials)| Profile::new(name, credentials))
            .collect();

        Self(profiles_set)
    }
}

impl From<Profiles> for BTreeMap<String, Credentials> {
    fn from(profiles: Profiles) -> Self {
        profiles
            .0
            .into_iter()
            .map(|profile| (profile.name, profile.credentials))
            .collect()
    }
}

impl<'p> From<&'p Profiles> for BTreeMap<&'p str, &'p Credentials> {
    fn from(profiles: &'p Profiles) -> Self {
        profiles
            .iter()
            .map(|profile| (profile.name.as_str(), &profile.credentials))
            .collect::<BTreeMap<&str, &Credentials>>()
    }
}
