use crate::{event::Event, AbstractState, Error, ErrorKind, Result};
use std::collections::HashMap;

pub struct Kernel<S>
where
    S: AbstractState,
{
    pub state: S,
    pub events: HashMap<String, Box<dyn Event<S>>>,
}

impl<S> Kernel<S>
where
    S: AbstractState,
{
    pub fn new(state: S) -> Self {
        Self {
            state,
            events: HashMap::new(),
        }
    }
    pub fn register(&mut self, name: &str, event: Box<dyn Event<S>>) {
        self.events.insert(name.to_string(), event);
    }
    pub fn step(&mut self, event_name: &str) -> Result<()> {
        if let Some(event) = self.events.get_mut(event_name) {
            event.apply(&mut self.state)
        } else {
            Err(Error::new(ErrorKind::EventNotFound))
        }
    }
}
