mod interval;
mod value;
mod ignored;
mod ident;

pub use value::*;
pub use interval::*;
pub use ignored::*;
pub use ident::*;

/// Common Kernel State Type. With matches function and serde support
pub trait AbstractState {
    fn matches(&self, other: &Self) -> bool;
}

/// Default implementation of AbstractState for any PartialEq type.
impl<T> AbstractState for T
where
    T: Eq,
{
    fn matches(&self, other: &Self) -> bool {
        self == other
    }
}
