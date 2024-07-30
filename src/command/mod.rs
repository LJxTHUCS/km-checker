use crate::state::AbstractState;
use alloc::vec::Vec;
use core::fmt::Debug;

/// Something that can be serialized to a byte array.
pub trait Serialize {
    /// Serialize the object to a byte array.
    fn serialize(&self) -> Vec<u8>;
}

/// A command that can be executed on a state.
pub trait Command<T>: Serialize + Debug
where
    T: AbstractState,
{
    /// Execute the command on the given state.
    fn execute(&self, state: &mut T) -> isize;
}

/// Wrap a command as a model command. Implement `Deref`, `From`, and `Debug` automatically.
///
/// Format: `model_command!(module_name,command_name)`.
#[macro_export]
macro_rules! model_command {
    ($($mod:ident)::*,$cmd:ident) => {
        pub struct $cmd(pub $($mod)::*::$cmd);

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

/// Implement `kernel_model_lib::Serialize` for a wrapped command.
///
/// Format: `impl_serialize!(command_name, to_vec function)`.
///
/// `to_vec` function - return type `Result<Vec<u8>,_>` .
#[macro_export]
macro_rules! impl_serialize {
    ($cmd:ident,$($fn:ident)::*) => {
        impl Serialize for $cmd {
            fn serialize(&self) -> Vec<u8> {
                $($fn)::*(&self.0).unwrap()
            }
        }
    };
}
