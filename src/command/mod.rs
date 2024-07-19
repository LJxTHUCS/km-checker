use crate::{error::Result, state::AbstractState};

pub trait Command<T>
where
    T: AbstractState,
{
    /// Execute the command on the given state.
    fn execute(&self, state: &mut T) -> Result<usize>;
    /// Serialize the command to a string.
    fn stringify(&self) -> String;
}
