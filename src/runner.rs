use crate::{error::Error, AbstractState, Kernel};

pub trait Commander {
    fn command(&mut self) -> String;
}

pub trait Printer {
    fn write(&mut self, s: &str);
}

pub trait TestPort<S>
where
    S: AbstractState,
{
    fn send(&mut self, event: &str);
    fn receive(&mut self) -> &S;
}
pub struct Runner<C, P, T, S>
where
    C: Commander,
    P: Printer,
    T: TestPort<S>,
    S: AbstractState,
{
    commander: C,
    printer: P,
    test_port: T,
    kernel: Kernel<S>,
}

impl<C, P, T, S> Runner<C, P, T, S>
where
    C: Commander,
    P: Printer,
    T: TestPort<S>,
    S: AbstractState,
{
    pub fn new(commander: C, printer: P, test_port: T, kernel: Kernel<S>) -> Self {
        Self {
            commander,
            printer,
            test_port,
            kernel,
        }
    }
    pub fn step(&mut self) -> Result<(), Error> {
        let event = self.commander.command();
        // Send command to test port
        self.test_port.send(&event);
        // Execute command in kernel model
        self.kernel.step(&event);
        // Receive state from test port
        let res = self.test_port.receive();
        // Compare state
        if !res.matches(&self.kernel.state) {
            return Err(Error::StateMismatch);
        }
        let output = format!("{:?}", serde_json::to_string(&self.kernel.state).unwrap());
        self.printer.write(&output);
        Ok(())
    }
}
