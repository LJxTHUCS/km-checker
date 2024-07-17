use super::AbstractState;
use serde::{Deserialize, Serialize};

/// A common interval type.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Interval<T> {
    /// Left bound (inclusive).
    pub left: usize,
    /// Right bound (exclusive).
    pub right: usize,
    /// Associated value.
    pub value: T,
}

impl<T> AbstractState for Interval<T>
where
    T: AbstractState,
{
    fn matches(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right && self.value.matches(&other.value)
    }
}

impl<T> Interval<T> {
    pub fn new(left: usize, right: usize, value: T) -> Self {
        Self { left, right, value }
    }
    pub fn overlaps(&self, other: &Self) -> bool {
        self.left <= other.right && self.right >= other.left
    }
    pub fn contains(&self, other: &Self) -> bool {
        self.left <= other.left && self.right >= other.right
    }
    pub fn intersect(&self, other: &Self) -> Option<Self>
    where
        T: Clone,
    {
        if self.overlaps(other) {
            Some(Interval {
                left: self.left.max(other.left),
                right: self.right.min(other.right),
                value: self.value.clone(),
            })
        } else {
            None
        }
    }
}
