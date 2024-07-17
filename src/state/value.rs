use super::AbstractState;
use serde::{Deserialize, Serialize};

/// Ordered List of Values
#[derive(Serialize, Deserialize, Debug)]
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

/// Unordered Set of Values
#[derive(Serialize, Deserialize, Debug)]
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
