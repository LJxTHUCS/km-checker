use crate::{AbstractState, Commander, Error, Printer, StateFetcher, TestPort};
use core::fmt::Debug;

/// Check level (of retv and state).
#[derive(Debug, PartialEq, Eq)]
pub enum CheckLevel {
    /// No checking.
    None,
    /// Only print if mismatch.
    Relaxed,
    /// Print and return error if mismatch.
    Strict,
}

/// Model Checker.
pub struct Checker<C, T, F, P, S>
where
    C: Commander<S>,
    T: TestPort<S>,
    F: StateFetcher<S>,
    P: Printer,
    S: AbstractState + Debug,
{
    commander: C,
    port: T,
    fetcher: F,
    printer: P,
    state: S,
    /// Round counter.
    round: usize,
    /// Current execution step.
    step: ExecutionStep,
    /// Return value of last command.
    retv: isize,
}

/// Checker execution steps.
enum ExecutionStep {
    Init,
    Command,
    Check,
}

impl<C, T, F, P, S> Checker<C, T, F, P, S>
where
    C: Commander<S>,
    T: TestPort<S>,
    F: StateFetcher<S>,
    P: Printer,
    S: AbstractState + Debug,
{
    /// Construct a test runner.
    pub fn new(commander: C, port: T, fetcher: F, printer: P, state: S) -> Self {
        Self {
            commander,
            port,
            fetcher,
            printer,
            state,
            round: 0,
            step: ExecutionStep::Init,
            retv: 0,
        }
    }

    /// Action on Init step.
    ///
    /// 1. Get state from test port and update self.
    fn init(&mut self) -> Result<(), Error> {
        self.state.update(&self.fetcher.get_state()?);
        self.printer.print("[ Initial State ]");
        self.printer.print(&format!("{:?}", self.state));
        Ok(())
    }

    /// Action on Command step.
    ///
    /// 1. Get command from commander.
    /// 2. Execute command on self state and record the return value.
    /// 3. Send command to test port.
    fn command(&mut self) -> Result<(), Error> {
        self.printer
            .print(&format!("\x1b[1;32m[ Round {} ]\x1b[0m", self.round));
        self.round += 1;
        let command = self.commander.command()?;
        self.printer.print(&format!("Command: {:?}", command));
        self.retv = command.execute(&mut self.state);
        self.port.send_command(command.as_ref())
    }

    /// Action on Check step.
    ///
    /// 1. Get return value from test port and compare with self.
    /// 2. Get state from test port and compare with self.
    fn check(&mut self, retv_level: CheckLevel, state_level: CheckLevel) -> Result<(), Error> {
        let test_retv = self.port.get_result();
        self.printer.print(&format!(
            "Expected: {:#x}, Got: {:#x}",
            self.retv, test_retv
        ));
        if retv_level != CheckLevel::None && test_retv != self.retv {
            self.printer.print("\x1b[1;31mReturn value mismatch\x1b[0m");
            if retv_level == CheckLevel::Strict {
                return Err(Error::ReturnValueMismatch);
            }
        }
        let test_state = self.fetcher.get_state()?;
        if state_level != CheckLevel::None && !test_state.matches(&self.state) {
            self.printer.print("\x1b[1;31mState mismatch\x1b[0m");
            self.printer.print("Expected:");
            self.printer.print(&format!("{:?}", self.state));
            self.printer.print("Got:");
            self.printer.print(&format!("{:?}", test_state));
            if state_level == CheckLevel::Strict {
                return Err(Error::StateMismatch);
            }
        }
        // self.state.update(&test_state);
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
