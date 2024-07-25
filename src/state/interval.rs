use super::AbstractState;
use alloc::vec;
use alloc::vec::Vec;

/// A common interval type.
#[derive(Debug, Clone, Copy, Default)]
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
    /// Check if self contains a point
    pub fn contains(&self, point: usize) -> bool {
        self.left <= point && self.right > point
    }
    /// Check if self overlaps with other.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.left) || other.contains(self.left)
    }
    /// Check if self covers other strictly.
    pub fn covers(&self, other: &Self) -> bool {
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
