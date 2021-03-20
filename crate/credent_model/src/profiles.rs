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
    serde(from = "BTreeMap<String, C>", into = "BTreeMap<String, C>")
)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Profiles<C = Credentials>(pub BTreeSet<Profile<C>>)
where
    C: Clone + Eq;

impl<C> Profiles<C>
where
    C: Clone + Eq,
{
    /// Returns an empty set of [`Profile`]s.
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }
}

impl<C> Deref for Profiles<C>
where
    C: Clone + Eq,
{
    type Target = BTreeSet<Profile<C>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C> DerefMut for Profiles<C>
where
    C: Clone + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<C> Display for Profiles<C>
where
    C: Clone + Display + Eq,
{
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

impl<C> From<BTreeMap<String, C>> for Profiles<C>
where
    C: Clone + Eq,
{
    fn from(profile_map: BTreeMap<String, C>) -> Self {
        let profiles_set = profile_map
            .into_iter()
            .map(|(name, credentials)| Profile::<C>::new(name, credentials))
            .collect();

        Self(profiles_set)
    }
}

impl<C> From<Profiles<C>> for BTreeMap<String, C>
where
    C: Clone + Eq,
{
    fn from(profiles: Profiles<C>) -> Self {
        profiles
            .0
            .into_iter()
            .map(|profile| (profile.name, profile.credentials))
            .collect()
    }
}

impl<'p, C> From<&'p Profiles<C>> for BTreeMap<&'p str, &'p C>
where
    C: Clone + Eq,
{
    fn from(profiles: &'p Profiles<C>) -> Self {
        profiles
            .iter()
            .map(|profile| (profile.name.as_str(), &profile.credentials))
            .collect::<BTreeMap<&str, &C>>()
    }
}
