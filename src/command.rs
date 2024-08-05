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

/// Default `to_bytes` implementation for model commands. 
/// This macro requires `km_command` as dependency.
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

/// Wrap a foreign-defined (typically `km_command`) command as a model 
/// command (command that can be executed on an abstract state) .
/// Implement `Deref`, `From`, and `Debug` for it.
/// 
/// If `execute_fn` is provided, it will be used to implenment
/// `Command` trait.
///
/// Format: 
/// 
/// - `model_command!(module_name, command_name)`.
/// - `model_command!(module_name, command_name, state_name, { execute_fn })`.
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
    ($($mod:ident)::*, $cmd:ident, $state:ident, $execute_fn:block) => {
        model_command!($($mod)::*, $cmd);

        impl $crate::Command<$state> for $cmd {
            fn execute(&self, state: &mut $state) -> isize {
                /// `get!()` => `self`; `get!(field)` => `self.field`
                #[allow(unused_macros)]
                macro_rules! get {
                    () => {
                        self
                    };
                    ($field:ident) => {
                        self.$field
                    };
                }
                /// `state!()` => `state`; `state!(field)` => `state.field`
                #[allow(unused_macros)]
                macro_rules! state {
                    () => {
                        state
                    };
                    ($field:ident) => {
                        state.$field
                    };
                }
                $execute_fn
            }
            impl_to_bytes!();
        }
    }
}

/// Generate commands for both the abstract model and the target kernel.
pub trait Commander<S>
where
    S: AbstractState,
{
    /// Get the next command to execute.
    fn command(&mut self) -> Result<Box<dyn Command<S>>, Error>;
}
