use crate::{state::AbstractState, Error};
use alloc::string::String;

pub type ExecutionResult = Result<usize, Error>;
pub trait Command<T>
where
    T: AbstractState,
{
    /// Execute the command on the given state.
    fn execute(&self, state: &mut T) -> ExecutionResult;
    /// Serialize the command to a string.
    fn stringify(&self) -> String;
}
