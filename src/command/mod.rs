use crate::{error::Result, state::AbstractState};

pub trait Command<T>
where
    T: AbstractState,
{
    /// Execute the command to the given state.
    fn execute(&self, state: &mut T) -> Result<()>;
    /// Serialize the command to a string.
    fn stringify(&self) -> String;
}
