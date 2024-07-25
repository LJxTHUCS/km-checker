use super::AbstractState;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

/// Single Value
#[derive(Debug, Clone, Copy, Default)]
pub struct Value<T>(pub T);

impl<T> AbstractState for Value<T>
where
    T: Eq,
{
    fn matches(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Deref for Value<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Value<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Ordered List of Values
#[derive(Debug, Clone, Default)]
pub struct ValueList<T>(pub Vec<T>);

impl<'a, T> AbstractState for ValueList<T>
where
    T: AbstractState,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a.matches(b))
    }
}

impl<T> Deref for ValueList<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for ValueList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Unordered Set of Values
#[derive(Debug, Default)]
pub struct ValueSet<T>(pub Vec<T>);

impl<'a, T> AbstractState for ValueSet<T>
where
    T: AbstractState,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0.iter().any(|a| other.0.iter().any(|b| a.matches(b)))
    }
}

impl<T> Deref for ValueSet<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for ValueSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Map of Values
#[derive(Debug, Default)]
pub struct ValueMap<K, V>(pub BTreeMap<K, V>)
where
    K: Ord;

impl<K, V> AbstractState for ValueMap<K, V>
where
    K: Ord,
    V: AbstractState,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0
            .iter()
            .all(|(k, v)| other.0.get(k).map_or(false, |ov| v.matches(ov)))
    }
}

impl<K, V> Deref for ValueMap<K, V>
where
    K: Ord,
{
    type Target = BTreeMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K, V> DerefMut for ValueMap<K, V>
where
    K: Ord,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
