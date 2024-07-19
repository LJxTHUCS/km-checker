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

// Associated value will not be checked for equality.
impl<T> PartialEq for Interval<T> {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}
impl<T> Eq for Interval<T> {}

impl<T> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.left.partial_cmp(&other.left)
    }
}
impl<T> Ord for Interval<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.left.cmp(&other.left)
    }
}

impl<T> Interval<T> {
    pub fn new(left: usize, right: usize, value: T) -> Self {
        Self { left, right, value }
    }
    /// Check if self overlaps with other.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.left < other.right && self.right > other.left
    }
    /// Check if self contains other strictly.
    pub fn contains(&self, other: &Self) -> bool {
        self.left < other.left && self.right > other.right
    }
    /// Get the intersection of self and other.
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
    /// Get the subset within self that is not in other.
    pub fn subtract(&self, other: &Self) -> Vec<Self>
    where
        T: Clone,
    {
        if !self.overlaps(other) {
            return vec![self.clone()];
        }
        let mut result = Vec::new();
        if self.left < other.left {
            result.push(Self::new(self.left, other.left, self.value.clone()));
        }
        if self.right > other.right {
            result.push(Self::new(other.right, self.right, self.value.clone()));
        }
        result
    }
}
