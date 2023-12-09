// Copyright 2023 Natalie Baker // AGPLv3 //

pub(crate) mod util;

newtype_str_id!(pub StateId     );
newtype_str_id!(pub TransitionId);

mod state_machine;
pub use state_machine::*;

mod state_engine;
pub use state_engine::*;

mod transition;
pub use transition::*;


