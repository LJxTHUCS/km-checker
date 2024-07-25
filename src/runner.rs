use crate::{AbstractState, Command, Error};
use alloc::{boxed::Box, format};

/// Checking level (of retv and state).
#[derive(Debug, PartialEq, Eq)]
pub enum CheckLevel {
    /// No checking.
    None,
    /// Only print if mismatch.
    Relaxed,
    /// Print and return error if mismatch.
    Strict,
}

/// Generate commands for both the abstract model and the target kernel.
pub trait Commander<S>
where
    S: AbstractState,
{
    /// Get the next command to execute.
    fn command(&mut self) -> Result<Box<dyn Command<S>>, Error>;
}

/// Print test info to the output.
pub trait Printer<S>
where
    S: AbstractState,
{
    /// Print an info string to the output.
    fn print_str(&mut self, s: &str);
    /// Print the current state.
    fn print_state(&mut self, s: &S);
}

/// Communicate with the target kernel.
pub trait TestPort<S>
where
    S: AbstractState,
{
    /// Send a command to the test target.
    fn send_command(&mut self, command: &dyn Command<S>) -> Result<(), Error>;
    /// Receive the return value from the test target.
    fn get_retv(&mut self) -> isize;
    /// Receive current state from the test target.
    fn get_state(&mut self) -> Result<S, Error>;
}

/// Model Checking Runner.
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
    /// Current execution step.
    step: ExecutionStep,
    /// Return value of last command.
    retv: isize,
}

/// Runner execution steps.
enum ExecutionStep {
    Init,
    Command,
    Check,
}

impl<C, P, T, S> Runner<C, P, T, S>
where
    C: Commander<S>,
    P: Printer<S>,
    T: TestPort<S>,
    S: AbstractState,
{
    /// Construct a test runner.
    pub fn new(commander: C, printer: P, test_port: T, state: S) -> Self {
        Self {
            commander,
            printer,
            test_port,
            state,
            step: ExecutionStep::Init,
            retv: 0,
        }
    }

    /// Action on Init step.
    ///
    /// 1. Get state from test port and update self.
    fn init(&mut self) -> Result<(), Error> {
        self.state = self.test_port.get_state()?;
        self.printer.print_state(&self.state);
        Ok(())
    }

    /// Action on Command step.
    ///
    /// 1. Get command from commander.
    /// 2. Execute command on self state and record the return value.
    /// 3. Send command to test port.
    fn command(&mut self) -> Result<(), Error> {
        let command = self.commander.command()?;
        self.retv = command.execute(&mut self.state);
        self.test_port.send_command(command.as_ref())
    }

    /// Action on Check step.
    ///
    /// 1. Get return value from test port and compare with self.
    /// 2. Get state from test port and compare with self.
    fn check(&mut self, retv_level: CheckLevel, state_level: CheckLevel) -> Result<(), Error> {
        let test_retv = self.test_port.get_retv();
        if retv_level != CheckLevel::None && test_retv != self.retv {
            self.printer.print_str("Return value mismatch");
            self.printer
                .print_str(&format!("expected: {}, got: {}", self.retv, test_retv));
            if retv_level == CheckLevel::Strict {
                return Err(Error::ReturnValueMismatch);
            }
        }
        let test_state = self.test_port.get_state()?;
        if state_level != CheckLevel::None && !test_state.matches(&self.state) {
            self.printer.print_str("State mismatch");
            self.printer.print_str("expected:");
            self.printer.print_state(&test_state);
            self.printer.print_str("got:");
            self.printer.print_state(&self.state);
            if state_level == CheckLevel::Strict {
                return Err(Error::StateMismatch);
            }
        }
        Ok(())
    }

    /// Common checker test step.
    ///
    /// Init -> Command -> Check -> Command -> Check -> ...
    pub fn step(&mut self, retv_level: CheckLevel, state_level: CheckLevel) -> Result<(), Error> {
        match self.step {
            ExecutionStep::Init => {
                self.init()?;
                self.step = ExecutionStep::Command;
            }
            ExecutionStep::Command => {
                self.command()?;
                self.step = ExecutionStep::Check;
            }
            ExecutionStep::Check => {
                self.check(retv_level, state_level)?;
                self.step = ExecutionStep::Command;
            }
        }
        Ok(())
    }
}
