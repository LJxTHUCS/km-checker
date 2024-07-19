use std::ops::{Deref, DerefMut};

use super::AbstractState;
use serde::{Deserialize, Deserializer};

/// Not Checked Fileds
#[derive(Debug, Clone)]
pub struct Ignored<T>(pub T);

impl<'de, T> Deserialize<'de> for Ignored<T>
where
    T: Default,
{
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Ignored(T::default()))
    }
}

impl<T> AbstractState for Ignored<T> {
    fn matches(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Deref for Ignored<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Ignored<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
