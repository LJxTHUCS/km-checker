#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// IO Error
    Io,
    /// State check failed
    StateMismatch,
    /// Return value check failed
    ReturnValueMismatch,
    /// Command execution failed (with i32 error code)
    ExecutionFail(i32),
}

pub type Result<T> = core::result::Result<T, Error>;
