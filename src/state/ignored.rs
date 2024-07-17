use super::AbstractState;
use serde::{Deserialize, Deserializer};

/// Not Checked Fileds
#[derive(Debug)]
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
