use super::Command;
use crate::{AbstractState, Error};

/// Generate commands for both the abstract model and the target kernel.
pub trait Commander<S>
where
    S: AbstractState,
{
    /// Get the next command to execute.
    fn command(&mut self) -> Result<Box<dyn Command<S>>, Error>;
}
