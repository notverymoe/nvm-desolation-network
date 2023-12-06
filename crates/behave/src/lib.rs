// Copyright 2023 Natalie Baker // AGPLv3 //

newtype_str_id!(pub StateId);

mod state_machine;
pub use state_machine::*;

mod state_engine;
pub use state_engine::*;

mod transition;
pub use transition::*;

mod str_id;
pub use str_id::*;



