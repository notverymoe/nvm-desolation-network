// Copyright 2023 Natalie Baker // AGPLv3 //

pub(crate) mod util;

newtype_str_id!(pub State);

#[macro_export]
macro_rules! behave_define {
    ($marker:ident, $state:ident) => {
        pub const $state: $crate::State<$marker> = $crate::State::from_name(stringify!($state));
    };

    ($marker:ident, ($transition:ident, $target:ident, $source:expr)) => {
        pub const $transition: $crate::Transition<$marker> = $crate::Transition::from_name(stringify!($transition), $target, &$source);
    };

    ($marker:ident, $state:ident, $($args:tt),+) => {
        behave_define!($marker, $state);
        behave_define!($marker, $($args),+);
    };

    ($marker:ident, ($transition:ident, $target:ident, $source:expr), $($args:tt),+) => {
        behave_define!($marker, ($transition, $target, $source));
        behave_define!($marker, $($args),+);
    };

}

mod state_machine;
pub use state_machine::*;

mod state_engine;
pub use state_engine::*;

mod transition;
pub use transition::*;


