use super::AbstractState;
use core::ops::{Deref, DerefMut};
use std::collections::BTreeMap;

/// Type that is checked value-by-value.
#[derive(Debug, Clone, Copy, Default)]
pub struct Value<T>(pub T);

impl<T> AbstractState for Value<T>
where
    T: Eq + Clone,
{
    fn matches(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    fn update(&mut self, other: &Self) {
        self.0 = other.0.clone();
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

/// Ordered list of values that are checked value-by-value.
#[derive(Debug, Clone, Default)]
pub struct ValueList<T>(pub Vec<T>);

impl<'a, T> AbstractState for ValueList<T>
where
    T: AbstractState + Clone,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a.matches(b))
    }
    fn update(&mut self, other: &Self) {
        self.0 = other.0.clone();
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

/// Unordered set of values that are checked value-by-value.
#[derive(Debug, Default)]
pub struct ValueSet<T>(pub Vec<T>);

impl<'a, T> AbstractState for ValueSet<T>
where
    T: AbstractState + Clone,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0.iter().any(|a| other.0.iter().any(|b| a.matches(b)))
    }
    fn update(&mut self, other: &Self) {
        self.0 = other.0.clone();
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

/// Map of values. Keys are checked by equality.
#[derive(Debug, Default)]
pub struct ValueMap<K, V>(pub BTreeMap<K, V>)
where
    K: Ord;

impl<K, V> AbstractState for ValueMap<K, V>
where
    K: Ord + Clone,
    V: AbstractState + Clone,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0
            .iter()
            .all(|(k, v)| other.0.get(k).map_or(false, |ov| v.matches(ov)))
    }
    fn update(&mut self, other: &Self) {
        self.0 = other.0.clone();
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
