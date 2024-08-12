use crate::{port::TestPort, AbstractState, Commander, Error, Printer};
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

/// Checker execution steps.
enum CheckStep {
    /// Start of execution.
    Start,
    /// Initialize model.
    Init,
    /// Commander send command to both model and target.
    Command,
    /// Checker check retv.
    CheckRetv,
    /// Get state from target.
    GetState,
    /// Checker check state.
    CheckState,
}

/// Model Checker.
pub struct Checker<C, T, P, S>
where
    C: Commander<S>,
    T: TestPort<S>,
    P: Printer,
    S: AbstractState + Debug,
{
    /// Generator of commands.
    commander: C,
    /// Port to comunicate with target.
    port: T,
    /// Info printer.
    printer: P,
    /// Abstract state of model.
    state: S,
    /// Round counter.
    round: usize,
    /// Current execution step.
    step: CheckStep,
    /// Return value of last command.
    retv: isize,
}

impl<C, T, P, S> Checker<C, T, P, S>
where
    C: Commander<S>,
    T: TestPort<S>,
    P: Printer,
    S: AbstractState + Debug,
{
    /// Construct a test runner.
    pub fn new(commander: C, port: T, printer: P, state: S) -> Self {
        Self {
            commander,
            port,
            printer,
            state,
            round: 0,
            step: CheckStep::Start,
            retv: 0,
        }
    }

    /// Checker can be regarded as a finite state machine. This is the state transition function.
    ///
    /// State is transited as follows:
    ///
    /// Start -> Init* -> Command -> CheckRetv -> GetState* -> CheckState -> Command -> ...
    pub fn step(&mut self, retv_level: CheckLevel, state_level: CheckLevel) -> Result<(), Error> {
        match self.step {
            CheckStep::Start => {
                // Start retrieving initial state from target.
                self.port.start_state_retrieval()?;
                self.step = CheckStep::GetState;
            }
            CheckStep::Init => {
                // Finish state retrieval, update self.
                let init_state = self.port.finish_state_retrieval()?;
                self.state.update(&init_state);
                self.printer.print("[ Initial State ]");
                self.printer.print(&format!("{:?}", self.state));
                self.step = CheckStep::Command;
            }
            CheckStep::Command => {
                self.round += 1;
                self.printer
                    .print(&format!("\x1b[1;32m[ Round {} ]\x1b[0m", self.round));
                // Get command from commander.
                let command = self.commander.command(&self.state)?;
                self.printer.print(&format!("Command: {:?}", command));
                // Execute command on self state and record the return value.
                self.retv = command.execute(&mut self.state);
                // Send command to test port.
                self.port.send_command(command.as_ref())?;
                self.step = CheckStep::CheckRetv;
            }
            CheckStep::CheckRetv => {
                // Get return value of the command from test target and compare with model.
                let test_retv = self.port.receive_retv();
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
                // Start retrieving state from target.
                self.port.start_state_retrieval()?;
                self.step = CheckStep::GetState;
            }
            CheckStep::GetState => {
                // Get state data from test target.
                let finished = self.port.retrieve_state_data()?;
                self.step = if finished {
                    if self.round != 0 {
                        CheckStep::CheckState
                    } else {
                        CheckStep::Init
                    }
                } else {
                    CheckStep::GetState
                };
            }
            CheckStep::CheckState => {
                // Finish state retrieval, compare with model.
                let test_state = self.port.finish_state_retrieval()?;
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
                self.step = CheckStep::Command;
            }
        }
        Ok(())
    }

    /// Get a reference to the state.
    pub fn state(&self) -> &S {
        &self.state
    }
}
