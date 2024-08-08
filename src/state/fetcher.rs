use super::AbstractState;
use crate::Error;
use std::{cell::RefCell, rc::Rc};

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
