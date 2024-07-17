use super::AbstractState;
use core::hash::Hash;
use serde::{Deserialize, Serialize};

/// Ordered List of Identifiers
#[derive(Debug, Deserialize, Serialize)]
pub struct IdentList<T>(pub Vec<T>)
where
    T: Hash + Eq;

impl<'a, T> AbstractState for IdentList<T>
where
    T: Hash + Eq,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        // TODO: map ident
        true
    }
}

/// Unordered Set of Identifiers
#[derive(Debug, Deserialize, Serialize)]
pub struct IdentSet<T>(pub Vec<T>)
where
    T: Hash + Eq;

impl<'a, T> AbstractState for IdentSet<T>
where
    T: Hash + Eq,
{
    fn matches(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        // TODO: map ident
        true
    }
}
