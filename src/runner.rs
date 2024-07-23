use crate::{AbstractState, Command, Error, ExecutionResult};
use alloc::boxed::Box;
use alloc::format;

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
    fn print_str(&mut self, s: &str) -> Result<(), Error>;
    /// Print the current state.
    fn print_state(&mut self, s: &S) -> Result<(), Error>;
}

/// Communicate with the target kernel.
pub trait TestPort<S>
where
    S: AbstractState,
{
    /// Send a command to the test target.
    fn send(&mut self, command: &dyn Command<S>) -> Result<(), Error>;
    /// Receive the return value from the test target.
    fn receive_retv(&mut self) -> ExecutionResult;
    /// Receive current state from the test target.
    fn receive_state(&mut self) -> Result<&S, Error>;
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
        }
    }

    /// Run a single step of model checking.
    ///
    /// 1. Get command from commander
    /// 2. Send command to test port
    /// 3. Execute command on abstract state
    /// 4. Check return value (if enabled)
    /// 5. Check state (if enabled)
    ///
    /// `ReturnValueMismatch` if return value discrepancy is found.
    /// `StateMismatch` if state discrepancy is found.
    pub fn step(&mut self, check_retv: bool, check_state: bool) -> Result<(), Error> {
        // Get command from commander
        let command = self.commander.command()?;
        self.printer
            .print_str(&format!("[command]: {}", command.stringify()))?;
        // Send command to test port
        self.test_port.send(command.as_ref())?;
        // Execute command in kernel model
        let model_ret = command.execute(&mut self.state);

        // Check return value
        if check_retv {
            // Receive return value from test port
            let test_ret = self.test_port.receive_retv();
            self.printer
                .print_str(&format!("[test retv]: {:?}", test_ret))?;
            self.printer
                .print_str(&format!("[model retv]: {:?}", model_ret))?;
            if test_ret != model_ret {
                return Err(Error::ReturnValueMismatch);
            }
        }

        // Check state
        if check_state {
            // Receive state from test port
            let test_state = self.test_port.receive_state()?;
            self.printer.print_str("[test state]: ")?;
            self.printer.print_state(test_state)?;
            self.printer.print_str("[model state]: ")?;
            self.printer.print_state(&self.state)?;
            if !test_state.matches(&self.state) {
                return Err(Error::StateMismatch);
            }
        }
        Ok(())
    }
}
