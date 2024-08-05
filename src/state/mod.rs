mod ignored;
mod interval;
mod value;

use crate::Error;
pub use ignored::*;
pub use interval::*;
pub use value::*;

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

/// Get abstract state from target kernel.
pub trait StateFetcher<S> {
    /// Get abstract state from target kernel.
    fn get_state(&mut self) -> Result<S, Error>;
}
