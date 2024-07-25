#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// IO Error
    Io,
    /// State check failed
    StateMismatch,
    /// Return value check failed
    ReturnValueMismatch,
}
