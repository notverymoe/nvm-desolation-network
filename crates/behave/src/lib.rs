// Copyright 2023 Natalie Baker // AGPLv3 //

mod state_machine;
mod state_engine;
mod state_machine_update;
mod transition;

pub(crate) mod util;

pub mod prelude {
    pub use crate::state_machine::*;
    pub use crate::state_engine::*;
    pub use crate::state_machine_update::*;
    pub use crate::transition::*;
    pub use crate::behave_define;
}

#[macro_export]
macro_rules! behave_define {
    ($marker:ident, $state:ident) => {
        pub const $state: $crate::prelude::State<$marker> = $crate::prelude::State::from_name(stringify!($state));
    };

    ($marker:ident, ($transition:ident, $target:ident, $sources:expr)) => {
        pub const $transition: $crate::prelude::TransitionRecord<$marker> = $crate::prelude::TransitionRecord{
            id:     $crate::prelude::Transition::from_name(stringify!($transition)),
            target: $target,
            sources: &$sources
        };
    };

    ($marker:ident, $state:ident, $($args:tt),+ $(,)?) => {
        $crate::prelude::behave_define!($marker, $state);
        $crate::prelude::behave_define!($marker, $($args),+);
    };

    ($marker:ident, ($transition:ident, $target:ident, $sources:expr), $($args:tt),+ $(,)?) => {
        $crate::prelude::behave_define!($marker, ($transition, $target, $sources));
        $crate::prelude::behave_define!($marker, $($args),+);
    };

}