use crate::{
    error::{Error, ErrorKind, Result},
    AbstractState, Kernel,
};

pub trait Commander {
    fn command(&mut self) -> Result<String>;
}

pub trait Printer<S>
where
    S: AbstractState,
{
    fn print_str(&mut self, s: &str) -> Result<()>;
    fn print_state(&mut self, s: &S) -> Result<()>;
}

pub trait TestPort<S>
where
    S: AbstractState,
{
    fn send(&mut self, event: &str) -> Result<()>;
    fn receive(&mut self) -> Result<&S>;
}
pub struct Runner<C, P, T, S>
where
    C: Commander,
    P: Printer<S>,
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
    P: Printer<S>,
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
    pub fn step(&mut self) -> Result<()> {
        let event = self.commander.command()?;
        // Send command to test port
        self.test_port.send(&event)?;
        // Execute command in kernel model
        self.kernel.step(&event)?;
        // Receive state from test port
        let res = self.test_port.receive()?;
        // Compare state
        self.printer.print_state(&res)?;
        self.printer.print_state(&self.kernel.state)?;
        if !res.matches(&self.kernel.state) {
            return Err(Error::new(ErrorKind::StateMismatch));
        }
        Ok(())
    }
}
