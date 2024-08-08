mod commander;

use crate::AbstractState;
pub use commander::Commander;
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
    ($($mod:ident)::*,$cmd:ident$(<$lt:lifetime>)?) => {
        pub struct $cmd$(<$lt>)?(pub $($mod)::*::$cmd$(<$lt>)?);

        impl$(<$lt>)? $cmd$(<$lt>)? {
            /// Command id.
            pub const ID: usize = $($mod)::*::$cmd::ID;
        }

        impl$(<$lt>)? core::ops::Deref for $cmd$(<$lt>)? {
            type Target = $($mod)::*::$cmd$(<$lt>)?;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl$(<$lt>)? From<$($mod)::*::$cmd$(<$lt>)?> for $cmd$(<$lt>)? {
            fn from(cmd: $($mod)::*::$cmd$(<$lt>)?) -> Self {
                Self(cmd)
            }
        }

        impl$(<$lt>)? core::fmt::Debug for $cmd$(<$lt>)? {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
    ($($mod:ident)::*, $cmd:ident$(<$lt:lifetime>)?, $state:ident, $execute_fn:block) => {
        model_command!($($mod)::*, $cmd$(<$lt>)?);

        impl$(<$lt>)? $crate::Command<$state> for $cmd$(<$lt>)? {
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
            $crate::impl_to_bytes!();
        }
    }
}
