// Copyright 2023 Natalie Baker // AGPLv3 //

mod state_machine;
mod state_engine;
mod state_machine_update;
mod transition;

pub(crate) mod util;

newtype_str_id!(pub State);

pub mod prelude {
    pub use crate::state_machine::*;
    pub use crate::state_engine::*;
    pub use crate::state_machine_update::*;
    pub use crate::transition::*;
    pub use crate::State;
    pub use crate::behave_define;
}


#[macro_export]
macro_rules! behave_define {
    ($marker:ident, $state:ident) => {
        pub const $state: $crate::prelude::State<$marker> = $crate::State::from_name(stringify!($state));
    };

    ($marker:ident, ($transition:ident, $target:ident, $source:expr)) => {
        pub const $transition: $crate::prelude::Transition<$marker> = $crate::prelude::Transition::from_name(stringify!($transition), $target, &$source);
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