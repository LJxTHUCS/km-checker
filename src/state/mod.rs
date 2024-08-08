mod ignored;
mod interval;
mod value;

use crate::Error;
pub use ignored::Ignored;
pub use interval::Interval;
use std::{cell::RefCell, rc::Rc};
pub use value::{Value, ValueList, ValueMap, ValueSet};

/// Generic Kernel State Type.
pub trait AbstractState {
    /// Check if the current state matches the other state.
    fn matches(&self, other: &Self) -> bool;
    /// Update the current state with the other state.
    fn update(&mut self, other: &Self);
}

/// Implements AbstractState for some basic types
macro_rules! impl_AbstractState {
    (for $($t:ty),+) => {
        $(impl AbstractState for $t {
            fn matches(&self, other: &Self) -> bool {
                self == other
            }
            fn update(&mut self, other: &Self) { *self = other.clone(); }
        })*
    }
}

impl_AbstractState!(for u8, i8, u16, i16, u32, i32, u64, i64, u128,
    i128, usize, isize, f32, f64, bool, char, String, &str);

impl<T> AbstractState for Option<T>
where
    T: AbstractState + Clone,
{
    fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => a.matches(b),
            (None, None) => true,
            _ => false,
        }
    }
    fn update(&mut self, other: &Self) {
        match other {
            Some(other) => {
                if let Some(self_) = self {
                    self_.update(other);
                } else {
                    *self = Some(other.clone());
                }
            }
            None => *self = None,
        }
    }
}

/// Get abstract state from target kernel.
pub trait StateFetcher<S>
where
    S: AbstractState,
{
    /// Get abstract state from target kernel.
    fn get_state(&mut self) -> Result<S, Error>;
}

/// A fake state fetcher that holds a reference to an abstract state.
pub struct FakeStateFetcher<S>(pub Rc<RefCell<S>>);

impl<S> StateFetcher<S> for FakeStateFetcher<S>
where
    S: AbstractState + Clone,
{
    fn get_state(&mut self) -> Result<S, Error> {
        Ok(self.0.borrow().clone())
    }
}
