use crate::state::AbstractState;
use alloc::vec::Vec;

/// A command that can be executed on a state.
pub trait Command<T>
where
    T: AbstractState,
{
    /// Execute the command on the given state.
    fn execute(&self, state: &mut T) -> isize;
    /// Serialize the command to bytes.
    fn serialize(&self) -> Vec<u8>;
}
