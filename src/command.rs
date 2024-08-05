use crate::{AbstractState, Error};
use core::fmt::Debug;

/// A command that can be executed on a state.
pub trait Command<T>: Debug
where
    T: AbstractState,
{
    /// Execute the command on the given state.
    fn execute(&self, state: &mut T) -> isize;
    /// Serialize the object to a byte array.
    fn to_bytes(&self) -> Vec<u8>;
}

/// Wrap a foreign-defined command as a model command
/// (command that can be executed on an abstract state) .
/// Implement `Deref`, `From`, and `Debug` for it.
///
/// Format: `model_command!(module_name,command_name)`.
#[macro_export]
macro_rules! model_command {
    ($($mod:ident)::*,$cmd:ident) => {
        pub struct $cmd(pub $($mod)::*::$cmd);

        impl $cmd {
            /// Command id.
            pub const ID: usize = $($mod)::*::$cmd::ID;
        }

        impl core::ops::Deref for $cmd {
            type Target = $($mod)::*::$cmd;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$($mod)::*::$cmd> for $cmd {
            fn from(cmd: $($mod)::*::$cmd) -> Self {
                Self(cmd)
            }
        }

        impl core::fmt::Debug for $cmd {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

/// Default `to_bytes` implementation for model commands. This macro requires
/// `km_command` as dependency.
///
/// Format: `impl_to_bytes!()`.
#[macro_export]
macro_rules! impl_to_bytes {
    () => {
        fn to_bytes(&self) -> Vec<u8> {
            let mut res = km_command::id_to_bytes(Self::ID);
            res.extend(self.0.to_bytes());
            res
        }
    };
}

/// Generate commands for both the abstract model and the target kernel.
pub trait Commander<S>
where
    S: AbstractState,
{
    /// Get the next command to execute.
    fn command(&mut self) -> Result<Box<dyn Command<S>>, Error>;
}
