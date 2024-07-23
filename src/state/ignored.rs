use super::AbstractState;
use core::ops::{Deref, DerefMut};

/// Not Checked Fileds
#[derive(Debug, Clone)]
pub struct Ignored<T>(pub T);

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
