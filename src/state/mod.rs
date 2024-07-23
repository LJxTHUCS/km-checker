use alloc::string::String;

mod ident;
mod ignored;
mod interval;
mod value;

pub use ident::*;
pub use ignored::*;
pub use interval::*;
pub use value::*;

/// Generic Kernel State Type.
pub trait AbstractState {
    fn matches(&self, other: &Self) -> bool;
}

/// Implements AbstractState for some basic types
macro_rules! impl_AbstractState {
    (for $($t:ty),+) => {
        $(impl AbstractState for $t {
            fn matches(&self, other: &Self) -> bool {
                self == other
            }
        })*
    }
}

impl_AbstractState!(for u8, i8, u16, i16, u32, i32, u64, i64, u128,
    i128, usize, isize, f32, f64, bool, char, String, &str);
