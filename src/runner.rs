use crate::{
    error::{Error, ErrorKind, Result},
    AbstractState, Command,
};

pub trait Commander<S>
where
    S: AbstractState,
{
    fn command(&mut self) -> Result<Box<dyn Command<S>>>;
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
    fn send(&mut self, command: &dyn Command<S>) -> Result<()>;
    fn receive(&mut self) -> Result<&S>;
}
pub struct Runner<C, P, T, S>
where
    C: Commander<S>,
    P: Printer<S>,
    T: TestPort<S>,
    S: AbstractState,
{
    commander: C,
    printer: P,
    test_port: T,
    state: S,
}

impl<C, P, T, S> Runner<C, P, T, S>
where
    C: Commander<S>,
    P: Printer<S>,
    T: TestPort<S>,
    S: AbstractState,
{
    pub fn new(commander: C, printer: P, test_port: T, state: S) -> Self {
        Self {
            commander,
            printer,
            test_port,
            state,
        }
    }

    /// Run a single step of model checking.
    ///
    /// 1. Get command from commander
    /// 2. Send command to test port
    /// 3. Execute command on abstract state
    /// 4. Receive result from test port
    /// 5. Compare results
    ///
    /// Returns `StateMismatch` if a discrepancy is found.
    pub fn step(&mut self) -> Result<()> {
        let command = self.commander.command()?;
        self.printer.print_str(&command.stringify())?;
        // Send command to test port
        self.test_port.send(command.as_ref())?;
        // Execute command in kernel model
        command.execute(&mut self.state)?;
        // Receive state from test port
        let res = self.test_port.receive()?;
        // Compare state
        self.printer.print_state(&res)?;
        self.printer.print_state(&self.state)?;
        if !res.matches(&self.state) {
            return Err(Error::new(ErrorKind::StateMismatch));
        }
        Ok(())
    }
}
