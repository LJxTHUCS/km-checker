use crate::state::AbstractState;

pub trait Event<T>
where
    T: AbstractState,
{
    fn apply(&self, state: &mut T);
}
