#[derive(Debug)]
pub enum ErrorKind {
    StateParseError,
    CommandNotFound,
    ExecutionFailed,
    StateMismatch,
    InvalidInput,
    IOError,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            message: None,
        }
    }
    pub fn with_message(kind: ErrorKind, message: String) -> Self {
        Self {
            kind,
            message: Some(message),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

#[macro_export]
macro_rules! format_err {
    ($kind:expr) => {
        Error::new($kind)
    };
    ($kind:expr, $($arg:tt)*) => {
        Error::with_message($kind, format!($($arg)*))
    };
}
