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
    /// Init flag.
    init: bool,
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
            init: false,
            round: 0,
            step: CheckStep::Start,
            retv: 0,
        }
    }

    /// Action on Init step.
    ///
    /// 1. Get state from test port and update self.
    fn init(&mut self) -> Result<(), Error> {
        let init_state = self.port.finish_state_retrieval()?;
        self.state.update(&init_state);
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
        let command = self.commander.command(&self.state)?;
        self.printer.print(&format!("Command: {:?}", command));
        self.retv = command.execute(&mut self.state);
        self.port.send_command(command.as_ref())
    }

    /// Get return value of the comamnd from test target and compare with model.
    fn check_retv(&mut self, retv_level: CheckLevel) -> Result<(), Error> {
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
        Ok(())
    }

    /// Get state from test target and compare with model.
    fn check_state(&mut self, state_level: CheckLevel) -> Result<(), Error> {
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
        Ok(())
    }

    /// Common checker test step. Check step is updated as follows:
    ///
    /// Start -> Init* -> Command -> CheckRetv -> GetState* -> CheckState -> Command -> ...
    pub fn step(&mut self, retv_level: CheckLevel, state_level: CheckLevel) -> Result<(), Error> {
        match self.step {
            CheckStep::Start => {
                self.port.start_state_retrieval()?;
                self.step = CheckStep::GetState;
            }
            CheckStep::Init => {
                self.init()?;
                self.step = CheckStep::Command;
            }
            CheckStep::Command => {
                self.command()?;
                self.step = CheckStep::CheckRetv;
            }
            CheckStep::CheckRetv => {
                self.check_retv(retv_level)?;
                self.port.start_state_retrieval()?;
                self.step = CheckStep::GetState;
            }
            CheckStep::GetState => {
                let finished = self.port.retrieve_state_data()?;
                self.step = if finished {
                    if self.init {
                        CheckStep::CheckState
                    } else {
                        CheckStep::Init
                    }
                } else {
                    CheckStep::GetState
                };
            }
            CheckStep::CheckState => {
                self.check_state(state_level)?;
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
